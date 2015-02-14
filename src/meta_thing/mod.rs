use std::rc::Rc;
use std::old_io::File;
use std::result::Result;
use cgmath::*;
use futil::*;
use meta_model::MetaModel;
use std::collections::HashMap;

pub struct MetaThing {
    author_name: String,
    thing_name: String,
    models: Vec<ModelInclusion>
}

impl MetaThing {
    pub fn from_file(path: &Path, meta_models_map: HashMap<&str, Rc<MetaModel>>) -> Result<MetaThing, String> {
        let mut file = File::open(path).unwrap();
        
        // Read header.
        file.read_be_u16().unwrap(); // Header size.
        file.read_be_u16().unwrap(); // Version.
        let author_name = read_string_16(&mut file);
        let thing_name = read_string_16(&mut file);
        file.read_u8().unwrap(); // Config key size.
        
        // Read models.
        file.read_be_u32().unwrap(); // Size of models section.
        let model_count = file.read_be_u16().unwrap();
        let mut models: Vec<ModelInclusion> = Vec::with_capacity(model_count as usize);
        for _ in 0u16..model_count {
            let author_name = read_string_16(&mut file);
            let model_name = read_string_16(&mut file);
            let key: String = format!("{}-{}", author_name, model_name);
            let meta_model: Option<&Rc<MetaModel>> = meta_models_map.get(key.as_slice());
            if meta_model.is_none() {
                let known_models = ""; // FIXME
                return Err(
                  format!(
                      "{} referenced model {}, but that model doesn't exist. Known models are: {}",
                      path.display(), key, known_models
                  )
                );
            }
            let meta_model = meta_model.unwrap().clone();
            let direction = file.read_u8().unwrap();
            let origin = read_vector_3(&mut file); // Model's origin relative to the thing's origin.
            models.push(ModelInclusion {
                meta_model: meta_model, direction: direction, origin: origin
            });
        }
               
        Ok(MetaThing {
            author_name: author_name, thing_name: thing_name, models: models
        })
    }
    
    pub fn author_name(&self) -> &String { &self.author_name }
    
    pub fn thing_name(&self) -> &String { &self.thing_name }
}

struct ModelInclusion {
    meta_model: Rc<MetaModel>,
    direction: u8,
    origin: Vector3<f32>
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::collections::HashMap;
    use super::MetaThing;
    use meta_model::MetaModel;
    
    #[test]
    fn from_file() {
        let meta_model = Rc::new(MetaModel::from_file(&Path::new("assets/models/jarrett-test.model")));
        let mut meta_models_map: HashMap<&str, Rc<MetaModel>> = HashMap::new();
        meta_models_map.insert("jarrett-test", meta_model);
        let meta_thing: MetaThing = MetaThing::from_file(
            &Path::new("assets/things/jarrett-test.thing"), meta_models_map
        ).unwrap();
        assert_eq!(&"test", &meta_thing.thing_name);
        assert_eq!(&"jarrett", &meta_thing.author_name);
        assert_eq!(1, meta_thing.models.len());
        assert_eq!(&"test", meta_thing.models.get(0).unwrap().meta_model.model_name());
        assert_eq!(&"jarrett", meta_thing.models.get(0).unwrap().meta_model.author_name());
    }
}