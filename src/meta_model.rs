use std::old_io::File;
use std::old_io::fs;
use std::rc::Rc;
use std::collections::HashMap;
use std::default::Default;
use cgmath::*;
use futil::*;
use model;

pub type MetaModelsMap = HashMap<String, Rc<MetaModel>>;

pub struct MetaModel {
    author_name: String,
    model_name: String,
    shape: u8,
    x_size: f32,
    y_size: f32,
    z_size: f32,
    uvs: Vec<UvsForDirection>,
    vbo_offset: u16
}

impl MetaModel {
    pub fn from_file(path: &Path) -> MetaModel {
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
        for _ in 0u8..8u8 {
            uvs.push(UvsForDirection::from_file(&mut file));
        }
        
        MetaModel {
          author_name: author_name, model_name: model_name, shape: shape,
          x_size: x_size, y_size: y_size, z_size: z_size, uvs: uvs, vbo_offset: 0
        }
    }
    
    pub fn load_dir(path: &Path, buffers: &mut model::Buffers) -> MetaModelsMap {
        let mut map: MetaModelsMap = HashMap::new();
        for path in fs::walk_dir(path).unwrap() {
            match path.extension_str() {
                Some("model") => {
                    let mut mm = MetaModel::from_file(&path);
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
    
    pub fn buffer(&mut self, buffers: &mut model::Buffers) {
        // See doc/model-rendering.md for a diagram of these vertices.
        let tb_pos = Vector3::new(self.x_size /  2.0, self.y_size / -2.0, self.z_size);
        let tr_pos = Vector3::new(self.x_size /  2.0, self.y_size /  2.0, self.z_size);
        let tf_pos = Vector3::new(self.x_size / -2.0, self.y_size /  2.0, self.z_size);
        let tl_pos = Vector3::new(self.x_size / -2.0, self.y_size / -2.0, self.z_size);
        let bl_pos = Vector3::new(self.x_size / -2.0, self.y_size / -2.0, 0.0);
        let bf_pos = Vector3::new(self.x_size / -2.0, self.y_size /  2.0, 0.0);
        let br_pos = Vector3::new(self.x_size /  2.0, self.y_size /  2.0, 0.0);
        
        self.vbo_offset = buffers.positions.len() as u16;
        
        // For each direction.
        for duvs in self.uvs.iter() {
            buffers.positions.push_all(&[
                // Top quad.
                tb_pos, tl_pos, tf_pos, tr_pos,
                // Left quad.
                tl_pos, bl_pos, bf_pos, tf_pos,
                // Right quad.
                tf_pos, bf_pos, br_pos, tr_pos
            ]);
        
            buffers.uvs.push_all(&[
                // Top quad.
                duvs.tb, duvs.tl, duvs.tf, duvs.tr,
                // Left quad.
                duvs.tl, duvs.bl, duvs.bf, duvs.tf,
                // Right quad.
                duvs.tf, duvs.bf, duvs.br, duvs.tr
            ]);
        
            // Offset into the attribute arrays.
            let o: u16 = buffers.positions.len() as u16;
        
            buffers.indices.push_all(&[
                // Top quad.
                o +  0, o +  1, o +  2, o +  3,
                // Left quad.
                o +  4, o +  5, o +  6, o +  7,
                // Right quad.
                o +  8, o +  9, o + 10, o + 11
            ]);
        }
    }
    
    pub fn draw(&self, model_program: &model::Program, abs_position: Vector3<f32>) {
        
    }
}

struct UvsForDirection {
    // See doc/model-rendering.md for a diagram of these vertices.
    tb: Vector2<f32>,
    tr: Vector2<f32>,
    tf: Vector2<f32>,
    tl: Vector2<f32>,
    bl: Vector2<f32>,
    bf: Vector2<f32>,
    br: Vector2<f32>
}

impl UvsForDirection {
    fn from_file(file: &mut File) -> UvsForDirection {
        UvsForDirection {
            tb: read_vector_2(file),
            tr: read_vector_2(file),
            tf: read_vector_2(file),
            tl: read_vector_2(file),
            bl: read_vector_2(file),
            bf: read_vector_2(file),
            br: read_vector_2(file),
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
        assert_eq!(0.355713993310928340, meta_model.uvs[0].top_back.x);
        assert_eq!(0.009803906083106995, meta_model.uvs[0].top_back.y);
        
        // Direction 2, bottom-left.
        assert_eq!(0.009803935885429382, meta_model.uvs[2].bottom_left.x);
        assert_eq!(0.797001838684082000, meta_model.uvs[2].bottom_left.y);
    }
}