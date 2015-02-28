mod model;
mod meta_model;
mod uvs_for_direction;
mod buffers;
mod program3d;

pub use self::model::Model;
pub use self::program3d::Program3d;
pub use self::meta_model::MetaModel;
pub use self::buffers::Buffers;

use std::rc::Rc;
use std::collections::HashMap;

pub type MetaModelsMap = HashMap<String, Rc<MetaModel>>;