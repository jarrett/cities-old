use std::old_io::File;
use cgmath::*;
use futil::*;
use std::default::Default;

pub struct MetaModel {
    author_name: String,
    model_name: String,
    shape: u8,
    x_size: f32,
    y_size: f32,
    z_size: f32,
    uvs: Vec<UvsForDirection>
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
          x_size: x_size, y_size: y_size, z_size: z_size, uvs: uvs
        }
    }
    
    pub fn author_name(&self) -> &String { &self.author_name }
    
    pub fn model_name(&self) -> &String { &self.model_name }
}

struct UvsForDirection {
    top_back:     Vector2<f32>,
    top_right:    Vector2<f32>,
    top_front:    Vector2<f32>,
    top_left:     Vector2<f32>,
    bottom_left:  Vector2<f32>,
    bottom_front: Vector2<f32>,
    bottom_right: Vector2<f32>
}

impl UvsForDirection {
    fn from_file(file: &mut File) -> UvsForDirection {
        UvsForDirection {
            top_back:     read_vector_2(file),
            top_right:    read_vector_2(file),
            top_front:    read_vector_2(file),
            top_left:     read_vector_2(file),
            bottom_left:  read_vector_2(file),
            bottom_front: read_vector_2(file),
            bottom_right: read_vector_2(file),
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