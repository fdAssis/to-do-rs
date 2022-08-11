use serde_json::value::Value;
use serde_json::Map;

use crate::state::write_to_file;
use crate::FILE;

pub trait Delete {
    fn delete(&self, title: &String, state: &mut Map<String, Value>) {
        state.remove(title);

        write_to_file(FILE, state);
        println!("\n\n{} is being deleted\n\n", title);
    }
}
