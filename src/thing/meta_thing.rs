use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::collections::HashMap;
use std::fs::File;
use std::fs;
use std::result::Result;
use std::io;
use byteorder::{ReadBytesExt, BigEndian};
use cgmath::Vector3;
use futil::{read_string_16, read_vector_3, IoErrorLine};

use model::MetaModel;
use model::MetaModelsMap;
use super::MetaThingsMap;

pub struct MetaThing {
    author_name: String,
    thing_name: String,
    models: Vec<ModelInclusion>
}

impl MetaThing {
    pub fn from_file(meta_models_map: &MetaModelsMap, path: &Path) -> Result<MetaThing, IoErrorLine> {
        let mut file = tryln!(File::open(path));
        
        // Read header.
        tryln!(file.read_u16::<BigEndian>()); // Header size.
        tryln!(file.read_u16::<BigEndian>()); // Version.
        let author_name = tryln!(read_string_16(&mut file));
        let thing_name = tryln!(read_string_16(&mut file));
        tryln!(file.read_u8()); // Config key size.
        
        // Read models.
        tryln!(file.read_u32::<BigEndian>()); // Size of models section.
        let model_count = tryln!(file.read_u16::<BigEndian>());
        let mut models: Vec<ModelInclusion> = Vec::with_capacity(model_count as usize);
        for _ in 0u16..model_count {
            let author_name = tryln!(read_string_16(&mut file));
            let model_name = tryln!(read_string_16(&mut file));
            let key: String = format!("{}-{}", author_name, model_name);
            let meta_model: &Rc<MetaModel> = match meta_models_map.get(&key) {
                Some(ref mm) => { mm },
                None => { return Err((
                    io::Error::new(
                        io::ErrorKind::Other,
                        format!("{} referenced model {}, which doesn't exist.", path.display(), key)
                    ),
                    file!(), line!()
                )); }
            };
            let meta_model = meta_model.clone();
            let direction = tryln!(file.read_u8());
            let offset = tryln!(read_vector_3(&mut file)); // Model's origin relative to the thing's origin.
            models.push(ModelInclusion {
                meta_model: meta_model, direction: direction, offset: offset
            });
        }
               
        Ok(MetaThing {
            author_name: author_name, thing_name: thing_name, models: models
        })
    }
    
    pub fn load_dir(meta_models_map: &MetaModelsMap, path: &Path) -> Result<MetaThingsMap, IoErrorLine> {
        let mut map: MetaThingsMap = HashMap::new();
        let walk = tryln!(fs::walk_dir(path));
        for entry in walk {
            let path: &PathBuf = &entry.unwrap().path();
            match path.extension() {
                Some(os_str) if os_str == "thing" => {
                    let result = MetaThing::from_file(meta_models_map, path);
                    match result {
                        Ok(mt) => {
                            let key = format!("{}-{}", mt.author_name(), mt.thing_name());
                            map.insert(key, Rc::new(mt));
                        },
                        Err(string) => { return Err(string); }
                    }
                }
                _ => ()
            }
        }
        Ok(map)
    }
    
    pub fn author_name(&self) -> &String { &self.author_name }
    
    pub fn thing_name(&self) -> &String { &self.thing_name }
    
    pub fn full_name(&self) -> String {
        format!("{}-{}", self.author_name, self.thing_name)
    }
    
    pub fn models(&self) -> &Vec<ModelInclusion> { &self.models }
}

struct ModelInclusion {
    pub meta_model: Rc<MetaModel>,
    pub direction: u8,
    pub offset: Vector3<f32>
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::collections::HashMap;
    use std::path::Path;
    use std::default::Default;
    use super::MetaThing;
    use model::MetaModel;
    use model::MetaModelsMap;
    use texture::Spritesheet;
    
    #[test]
    fn test_from_file() {
        let meta_model: Rc<MetaModel> = Rc::new(
            MetaModel::from_file(
                &Path::new("assets/models/jarrett-test.model"),
                None
            ).unwrap()
        );
        let mut meta_models_map: MetaModelsMap = HashMap::new();
        meta_models_map.insert("jarrett-test".to_string(), meta_model);
        let meta_thing: MetaThing = MetaThing::from_file(
            &meta_models_map, &Path::new("assets/things/jarrett-test.thing")
        ).unwrap();
        assert_eq!(&"test", &meta_thing.thing_name);
        assert_eq!(&"jarrett", &meta_thing.author_name);
        assert_eq!(1, meta_thing.models.len());
        assert_eq!(&"test", meta_thing.models.get(0).unwrap().meta_model.model_name());
        assert_eq!(&"jarrett", meta_thing.models.get(0).unwrap().meta_model.author_name());
    }
}