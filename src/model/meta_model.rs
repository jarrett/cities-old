use std::old_io::File;
use std::old_io::fs;
use std::mem;
use libc::{c_void};
use std::rc::Rc;
use std::collections::HashMap;
use cgmath::*;
use gl;
use gl::types::*;

use futil::*;
use model;
use camera::Camera;
use texture::{Spritesheet, Sprite};
use super::uvs_for_direction::UvsForDirection;
use super::{MetaModelsMap, Buffers};

pub struct MetaModel {
    author_name: String,
    model_name: String,
    shape: u8,
    x_size: f32,
    y_size: f32,
    z_size: f32,
    uvs: Vec<UvsForDirection>,
    sprites: Vec<Rc<Sprite>>,
    index_offset: u16 // Offset into the index VBO.
}

impl MetaModel {
    pub fn from_file(path: &Path, spritesheet: &Spritesheet) -> MetaModel {
        let mut file = File::open(path).unwrap();
        
        // Read header.
        file.read_be_u16().unwrap(); // Header size.
        file.read_be_u16().unwrap(); // Version.
        let shape = file.read_u8().unwrap();
        file.read_u8().unwrap(); // Image embedded.
        let author_name = read_string_16(&mut file);
        let model_name = read_string_16(&mut file);
        
        // Read geometry.
        file.read_be_u16().unwrap(); // Geometry section size.
        let x_size = file.read_be_f32().unwrap();
        let y_size = file.read_be_f32().unwrap();
        let z_size = file.read_be_f32().unwrap();
        let mut uvs = Vec::with_capacity(8);
        let mut sprites = Vec::with_capacity(8);
        for direction in 0u8..8u8 {
            uvs.push(UvsForDirection::from_file(&mut file));
            let sprite_name = format!("{}-{}-{}", author_name, model_name, direction);
            let sprite = spritesheet.by_name.get(&sprite_name).expect(format!(
                "Texture not found for {}. Known textures: {}",
                sprite_name, spritesheet.format_all()
            ).as_slice()).clone();
            sprites.push(sprite);
        }
        
        MetaModel {
          author_name: author_name, model_name: model_name, shape: shape,
          x_size: x_size, y_size: y_size, z_size: z_size, uvs: uvs,
          sprites: sprites, index_offset: 0
        }
    }
    
    pub fn load_dir(path: &Path, buffers: &mut Buffers, spritesheet: &Spritesheet) -> MetaModelsMap {
        let mut map: MetaModelsMap = HashMap::new();
        for path in fs::walk_dir(path).unwrap() {
            match path.extension_str() {
                Some("model") => {
                    let mut mm = MetaModel::from_file(&path, spritesheet);
                    mm.buffer(buffers);
                    let key = format!("{}-{}", mm.author_name(), mm.model_name());
                    map.insert(key, Rc::new(mm));
                },
                _ => {}
            }
        }
        map
    }
    
    pub fn author_name(&self) -> &String { &self.author_name }
    
    pub fn model_name(&self) -> &String { &self.model_name }
    
    pub fn buffer(&mut self, buffers: &mut Buffers) {
        // See doc/model-rendering.md for a diagram of these vertices.
        let tb_pos = Vector3::new(self.x_size / -2.0, self.y_size / -2.0, self.z_size);
        let tr_pos = Vector3::new(self.x_size /  2.0, self.y_size / -2.0, self.z_size);
        let tf_pos = Vector3::new(self.x_size /  2.0, self.y_size /  2.0, self.z_size);
        let tl_pos = Vector3::new(self.x_size / -2.0, self.y_size /  2.0, self.z_size);
        let bl_pos = Vector3::new(self.x_size / -2.0, self.y_size /  2.0, 0.0);
        let bf_pos = Vector3::new(self.x_size /  2.0, self.y_size /  2.0, 0.0);
        let br_pos = Vector3::new(self.x_size /  2.0, self.y_size / -2.0, 0.0);
        
        self.index_offset = buffers.indices.len() as u16;
        
        // For each direction.
        // Positions and UVs get 96 vectors: 4 verts per quad * 3 quads * 8 directions.
        // The positions are 3d vectors; the UVs are 2d.
        for (direction, duvs) in self.uvs.iter().enumerate() {
            // Offset into the attributes VBO. We'll use this
            // when we buffer the indices.
            let o = buffers.positions.len() as u16; 
            
            buffers.positions.push_all(&[
                // Top quad: 0 - 3.
                tb_pos, tl_pos, tf_pos, tr_pos,
                // Left quad: 4 - 7.
                tl_pos, bl_pos, bf_pos, tf_pos,
                // Right quad: 8 - 11.
                tf_pos, bf_pos, br_pos, tr_pos
            ]);
            
            let sprite = &self.sprites[direction];
            let tb = sprite.in_sheet_space(&duvs.tb);
            let tl = sprite.in_sheet_space(&duvs.tl);
            let tf = sprite.in_sheet_space(&duvs.tf);
            let tr = sprite.in_sheet_space(&duvs.tr);
            let bl = sprite.in_sheet_space(&duvs.bl);
            let bf = sprite.in_sheet_space(&duvs.bf);
            let br = sprite.in_sheet_space(&duvs.br);
            
            buffers.uvs.push_all(&[
                // Top quad: 0 - 3.
                tb, tl, tf, tr,
                // Left quad: 4 - 7.
                tl, bl, bf, tf,
                // Right quad: 8 - 11.
                tf, bf, br, tr
            ]);
            
            buffers.indices.push_all(&[
                // Top quad.
                o +  0, o +  1, o +  3,
                o +  1, o +  2, o +  3,
                // Left quad.
                o +  4, o +  5, o +  7,
                o +  5, o +  6, o +  7,
                // Right quad.
                o +  8, o +  9, o + 11,
                o +  9, o + 10, o + 11
            ]);
        }
    }
    
    pub fn draw(
        &self, program: &model::Program3d, buffers: &Buffers,
        camera: &Camera, abs_position: &Vector3<f32>,
        direction: u8
    ) {
        if !buffers.uploaded { panic!("Called draw before uploading buffers"); }
        unsafe {            
            gl::BindVertexArray(buffers.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, buffers.index_buffer);
            gl::UseProgram(program.id);
            gl::UniformMatrix4fv(program.model_view_idx, 1, gl::FALSE, mem::transmute(&camera.model_view));
            gl::UniformMatrix4fv(program.projection_idx, 1, gl::FALSE, mem::transmute(&camera.projection));
            gl::Uniform3fv(program.origin_idx, 1, mem::transmute(abs_position));
            gl::Uniform1i(program.direction_idx, direction as GLint);
            gl::Uniform1i(program.orbit_idx, camera.orbit as GLint);
            program.bind_textures(self.sprites[direction as usize].texture_id);
            // The offset into the index buffer determines which sprite to draw. Each
            // sprite has its own set of six triangles.
            // 
            // We select the sprite based on the direction the model is facing relative
            // to the camera. This takes into account both direction and camera.orbit.
            // 
            // In the 0th sprite, the front of the model faces down and to the left
            // in screen space. As the sprite index increases, the model rotates
            // clockwise. Thus, in the 2nd sprite, the front faces up and to the left.
            // And so on. Similarly, as the camera orbit increments, the world
            // rotates clockwise. One camera orbit is 90 degrees, which is worth two
            // directional steps.
            //
            // Number of elements to draw = 3 quads * 6 verts per quad * 2 bytes per vert.
            // 
            // FIXME: The number of elements to draw and the offset
            // should be different for 2d models.
            let sprite_num: u16 = (direction + camera.orbit * 2) as u16 % 8;
            let offset = self.index_offset + sprite_num * 36;
            gl::DrawElements(gl::TRIANGLES, 18, gl::UNSIGNED_SHORT, offset as *const c_void);
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MetaModel;
    use assertions::*;
    
    #[test]
    fn from_file() {
        let meta_model: MetaModel = MetaModel::from_file(&Path::new(
            "assets/models/jarrett-test.model"
        ));
        
        assert_eq!(&"jarrett".to_string(), &meta_model.author_name);
        assert_eq!(&"test".to_string(), &meta_model.model_name);
        assert_eq!(0, meta_model.shape);
        assert_eq_f32(2.3520656824111940, meta_model.x_size);
        assert_eq_f32(2.4116761684417725, meta_model.y_size);
        assert_eq_f32(2.2839789390563965, meta_model.z_size);
        assert_eq!(8, meta_model.uvs.len());
        
        // We spot-check the UV coordinates. There are 112 total floats comprising the
        // coords, so it's not practical to assert all of them here.
        
        // Direction 0, top-back.
        assert_eq_f32(0.355713993310928340, meta_model.uvs[0].tb.x);
        assert_eq_f32(0.009803906083106995, meta_model.uvs[0].tb.y);
        
        // Direction 2, bottom-left.
        assert_eq_f32(0.009803935885429382, meta_model.uvs[2].bl.x);
        assert_eq_f32(0.797001838684082000, meta_model.uvs[2].bl.y);
    }
}