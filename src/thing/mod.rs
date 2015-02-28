mod thing;
mod meta_thing;

pub use self::thing::Thing;
pub use self::meta_thing::MetaThing;

use std::rc::Rc;
use std::collections::HashMap;

pub type MetaThingsMap = HashMap<String, Rc<MetaThing>>;