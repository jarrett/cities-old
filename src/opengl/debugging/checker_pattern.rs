// Returns a checker texture encoded as RGBA U8.
pub fn checker(w: usize, h: usize, square_size: usize) -> Vec<u8> {
  let mut v = Vec::with_capacity(w * h * 4);
  for y in 0..h {
    for x in 0..w {
      if ((x / square_size) + (y / square_size)) % 2 == 0 {
        v.push(255);
        v.push(255);
        v.push(255);
      } else {
        v.push(0);
        v.push(0);
        v.push(255);
      }
    }
  }
  v
}