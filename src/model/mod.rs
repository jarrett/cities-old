mod model;
mod meta_model;
mod uvs_for_direction;
mod buffers;
mod program3d;
mod sprite_sheet;
mod sprite_pack;

pub use self::model::Model;
pub use self::program3d::Program3d;
pub use self::meta_model::MetaModel;
pub use self::buffers::Buffers;
pub use self::sprite_sheet::{Sprite, SpriteSheet};

use std::rc::Rc;
use std::collections::HashMap;

pub type MetaModelsMap = HashMap<String, Rc<MetaModel>>;