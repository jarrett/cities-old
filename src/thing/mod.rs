mod thing;
mod meta_thing;
mod z_sorted;

pub use self::thing::Thing;
pub use self::meta_thing::MetaThing;
pub use self::z_sorted::ZSorted;

use std::rc::Rc;
use std::collections::HashMap;

pub type MetaThingsMap = HashMap<String, Rc<MetaThing>>;