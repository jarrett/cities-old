mod adapter;

use std::path::Path;
use std::fs::File;
use std::cmp::{min, max};
use image::{DynamicImage, GenericImage, ImageFormat, Rgba};
use freetype as ft;

use errors::GameError;

pub use self::adapter::Adapter;
use self::adapter::{DynamicImageAdapter};

// Text rendering proceeds in two phases: layout and rasterization. In the layout phase,
// we build a tree representing the text's position in space. In the rasterization phase,
// we copy the contents of this data structure onto a bitmap. The rasterization target can
// be any object that implements the text::Adapter trait.
// 
// The end result of the layout phase is a tree as show below.
// => means "owns many," and -> means "owns one."
// 
// RenderedString
//   => RenderedLine
//   => RenderedWord
//     => RenderedChar
//     -> ft::BitmapGlyph
//     -> ft::Bitmap
// 
// For background info on FreeType, see:
// http://www.freetype.org/freetype2/docs/tutorial/step1.html
// http://www.freetype.org/freetype2/docs/tutorial/step2.html
// 
// Important metrics (http://www.freetype.org/freetype2/docs/glyphs/Image3.png):
// 
// - BitmapGlyph.left() is the distance to translate right when copying from the
//   glyph's buffer to the target image. In other words, when computing the target X
//   coordinate, add this value. This value is in screen pixels.
// 
// - BitmapGlyph.top() is the height above the baseline in screen pixels.
// 
// - Bitmap.rows() - BitmapGlyph.top() is the height below the baseline in screen pixels.
// 
// - GlyphSlot.advance().x is the amount to add to pen X after drawing the current glyph.
//   (If using kerning, add the kerning pair delta to pen X as well.)

struct RenderedString {
  pub w: i32,
  pub h: i32,
  pub lines: Vec<RenderedLine>
}

struct RenderedLine {
  pub w: i32,
  pub x: i32, // Distance from line's left edge to string's left edge.
  pub y: i32, // Distance from baseline to string's top edge.
  pub min_y: i32, // Relative to the baseline.
  pub max_y: i32, // Relative to the baseline.
  pub words: Vec<RenderedWord>
}

struct RenderedWord {
  pub x: i32, // Relative to the left edge of the line.
  pub w: i32,
  pub min_y: i32, // Extent below the baseline.
  pub max_y: i32, // Extent above the baseline.
  pub chars: Vec<RenderedChar>
}

struct RenderedChar {
  pub x: i32, // Relative to the left edge of the word.
  pub glyph_index: u32,
  pub advance: i32,
  pub bitmap_glyph: ft::BitmapGlyph,
  pub bitmap: ft::Bitmap
}

pub enum Align {
  Left,
  Center
}

pub fn render_string(string: &str, size: i32, max_w: Option<i32>, align: Align) -> Result<(), GameError> {  
  // http://www.freetype.org/freetype2/docs/tutorial/step1.html
  
  // A Library is a set of typefaces, e.g. the regular, bold, and italic variants
  // of Fira Sans.
  let library = try!(ft::Library::init());
  
  // Load a single typeface into the library.
  let face = try!(library.new_face(&Path::new("assets/fonts/FiraSans-Light.otf"), 0));
  
  // Although FreeType can read vector fonts, its internal representation is raster. So
  // we tell FreeType how tall, in pixels, we'd like each glyph to be.
  try!(face.set_pixel_sizes(0, size as u32));
  
  let rendered_string = try!(RenderedString::new(
    &face,
    string,
    (size as f32 * 1.1) as i32, // Leading.
    (size as f32 * 0.3) as i32, // Space width.
    max_w,
    align
  ));
  
  // Initialize the target image.
  let mut adapter = DynamicImageAdapter::new(
    DynamicImage::new_rgba8(rendered_string.w as u32, rendered_string.h as u32)
  );
  
  rendered_string.copy(&mut adapter);
  
  let mut file = try!(File::create("glyph-test.png"));
  adapter.img.save(&mut file, ImageFormat::PNG);
  
  Ok(())
}

impl RenderedString {
  fn new<S: AsRef<str>>(
    // Be sure to configure the character height by calling set_pixel_sizes first.
    face: &ft::Face,
    string: S,
    // Line height. I.e. number of pixels from baseline to baseline.
    leading: i32,
    space_w: i32,
    max_w: Option<i32>,
    align: Align
  ) -> Result<RenderedString, GameError> {
    // The string to be rendered could be any string-like type. Normalize it to &str.
    let string: &str = string.as_ref();
    
    // Render each word in the string.
    let mut rendered_words: Vec<RenderedWord> = try!(string
      .split_whitespace()
      .map(|w| RenderedWord::new(face, w))
      .collect());
    rendered_words.reverse();
    
    let mut rendered_string = RenderedString {
      w: 0, h: 0, lines: Vec::new()
    };
    
    // Consume the list of words. The result is a list of lines. Update the total
    // width as we go.
    while !rendered_words.is_empty() {
      let line = RenderedLine::new(face, &mut rendered_words, space_w, max_w);
      rendered_string.w = max(rendered_string.w, line.w);
      rendered_string.lines.push(line);
    }
    
    // Compute the height of the rendered string.
    rendered_string.h =
      // Space between lines.
      (rendered_string.lines.len() as i32 - 1) * leading +
      // Space above the uppermost baseline.
      rendered_string.lines.first().map_or(0, |l| l.max_y) -
      // Space below the lowermost baseline. It's a negative number, so we subtract.
      rendered_string.lines.last().map_or(0, |l| l.min_y);
    
    // Position each line within the rendered string.
    let mut baseline_y: i32 = rendered_string.lines.first().map_or(0, |l| l.max_y);
    for line in rendered_string.lines.iter_mut() {
      line.y = baseline_y;
      line.x = match align {
        Align::Left => 0,
        Align::Center => (rendered_string.w - line.w) / 2
      };
      baseline_y += leading;
    }
    
    Ok(rendered_string)
  }
  
  // Copies this character's Freetype bitmap onto a destination bitmap.
  fn copy<A: Adapter>(&self, adapter: &mut A) {
    // The current position of the baseline relative to the top of the destination bitmap.
    let mut pen_y: i32 = self.lines.first().map_or(0, |l| -1 * l.min_y);
    
    for line in self.lines.iter() {
      line.copy(adapter);
    }
  }
}

impl RenderedLine {
  // Takes a mutable Vec of words in reverse order. If a max width is given, pops from
  // the Vec, adding words onto the current line until the max width has been reached.
  // If a max width is not given, consumes the entire Vec.
  fn new(face: &ft::Face, words: &mut Vec<RenderedWord>, space_w: i32, max_w: Option<i32>) -> RenderedLine {
    let mut line = RenderedLine {
      w: 0, min_y: 0, max_y: 0, x: 0, y: 0, words: Vec::new()
    };
    while words.len() != 0 && line.can_fit_word(words.last().unwrap(), max_w) {
      let mut word = words.pop().unwrap();
      word.x = line.w;
      line.w += word.w + space_w;
      line.min_y = min(line.min_y, word.min_y);
      line.max_y = max(line.max_y, word.max_y);
      line.words.push(word);
    }
    line.w -= space_w; // No space after the last word.
    line
  }
  
  fn can_fit_word(&self, word: &RenderedWord, max_w: Option<i32>) -> bool {
    max_w.map_or(true, |mw| self.w + word.w <= mw)
  }
  
  fn copy<A: Adapter>(&self, adapter: &mut A) {
    for word in self.words.iter() {
      word.copy(adapter, self.x, self.y);
    }
  }
}

impl RenderedWord {
  fn new(face: &ft::Face, string: &str) -> Result<RenderedWord, GameError> {
    let mut rendered_chars: Vec<RenderedChar> = Vec::new();
    // Relative to the word's left edge. Whereas line_pen_x is relative
    // to the left edge of the line.
    let mut word_pen_x: i32 = 0;
    
    // Kerning depends on the previous glyph. Updated in the chars loop.
    let mut prev_glyph_index: u32 = 0;
    
    // Y coords are relative to the baseline. Up is positive.
    let mut min_y: i32 = 0;
    let mut max_y: i32 = 0;
    
    for chr in string.chars() {
      let mut rchr = try!(RenderedChar::new(&face, chr));
      
      min_y = min(min_y, rchr.bitmap_glyph.top() - rchr.bitmap.rows());
      max_y = max(max_y, rchr.bitmap_glyph.top());
      
      // FreeType can compute a kerning adjustment for any two pairs of glyphs, given
      // each glyph index. The adjustment can be positive or negative.
      // 
      // For some reason, nearly all kerning pairs are 0. We're probably calling the
      // Freetype API incorrectly.
      let kerning: i32 = face.get_kerning(
        prev_glyph_index, rchr.glyph_index, ft::face::KerningMode::KerningDefault
      ).map(|v| (v.x >> 6) as i32).unwrap_or(0);
      rchr.x = word_pen_x + kerning;
      word_pen_x += rchr.advance + kerning;
      prev_glyph_index = rchr.glyph_index;
      
      rendered_chars.push(rchr);
    }
    
    Ok(RenderedWord {
      w: word_pen_x,
      x: 0,
      min_y: min_y,
      max_y: max_y,
      chars: rendered_chars
    })
  }
  
  // dest_x is the distance from the line's left edge to the destination's left edge.
  // dest_y is the distance from the baseline to the destination's top edge.
  fn copy<A: Adapter>(&self, adapter: &mut A, dest_x: i32, dest_y: i32) {
    let word_dest_x = self.x + dest_x;
    for rchr in self.chars.iter() {
      rchr.copy(adapter, word_dest_x, dest_y);
    }
  }
}

impl RenderedChar {
  // If word_pen_x is None, defaults to 0.
  fn new(face: &ft::Face, chr: char) -> Result<RenderedChar, GameError> {
    let code_point: usize = chr as usize;
    
    // get_char_index takes a Unicode code point and returns an index to a specific
    // glyph in the typeface. This mapping is necessary because, for example, code
    // point 65 (capital A) might not be the 65th glyph in the typeface.
    let glyph_index: u32 = face.get_char_index(code_point);
    
    // Each face has exactly one glyph slot. The slot stores whatever glyph is
    // currently being operated on. So in order to render multiple glyphs to an image,
    // we need to load the glyphs one at a time into the glyph slot. Then, we can copy
    // each glyph out using get_glyph.
    // 
    // The RENDER flag is equivalent to FT_LOAD_RENDER in the FreeType C API. This is
    // a shorthand; it tells FreeType to call render_glyph after loading. Rendering
    // generates a raster version of the glyph and caches it to the glyph slot. We can
    // then access that raster image via face.glyph().bitmap().
    try!(face.load_glyph(glyph_index, ft::face::DEFAULT));
    
    let glyph_slot = face.glyph();
    let glyph = try!(glyph_slot.get_glyph());
    let bitmap_glyph = try!(glyph.to_bitmap(ft::render_mode::RenderMode::Normal, None));
    let bitmap = bitmap_glyph.bitmap();
    
    Ok(RenderedChar {
      x: 0,
      glyph_index: glyph_index,
      advance: (glyph_slot.advance().x >> 6) as i32,
      bitmap_glyph: bitmap_glyph,
      bitmap: bitmap
    })
  }
  
  // dest_x is the distance from the words's left edge to the destination's left edge.
  // dest_y is the distance from the baseline to the destination's top edge.
  fn copy<A: Adapter>(&self, adapter: &mut A, dest_x: i32, dest_y: i32) {
    // Get a reference to the pixel data in the Bitmap.
    let buffer: &[u8] = self.bitmap.buffer();
    
    // The distance from the bitmap's top edge to the baseline.
    let top = self.bitmap_glyph.top();
    
    // bmp_x and bmp_y are relative to the upper-left of the Freetype bitmap.
    for bmp_y in 0..self.bitmap.rows() {
      for bmp_x in 0..self.bitmap.width() {
        let adapter_x: i32 = dest_x + bmp_x + self.x + self.bitmap_glyph.left();
        let adapter_y: i32 = dest_y + bmp_y - top;
        if adapter_x < 0 { panic!("adapter_x was {}", adapter_x); }
        if adapter_y < 0 { panic!("adapter_y was {}", adapter_y); }
        adapter.safe_put_pixel(
          adapter_x as u32,
          adapter_y as u32,
          buffer[
            (bmp_y * self.bitmap.width() + bmp_x)
            as usize
          ]
        );
      }
    }
  }
  
  fn width(&self) -> i32 {
    self.bitmap.width()
  }
}