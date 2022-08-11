use serde_json::value::Value;
use serde_json::{json, Map};

use crate::state::write_to_file;
use crate::FILE;

pub trait Create {
    fn create(
        &self,
        title: &String,
        status: &String,
        last_update: &String,
        state: &mut Map<String, Value>,
    ) {
        state.insert(
            title.to_string(),
            json!({"status": status, "last_update": last_update}),
        );

        write_to_file(FILE, state);
        println!("\n\n{} is being create\n\n", title);
    }
}
