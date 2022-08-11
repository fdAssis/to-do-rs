use chrono::Local;
use serde_json::value::Value;
use serde_json::{json, Map};

use crate::state::write_to_file;
use crate::FILE;
pub trait Edit {
    fn set_to_done(&self, title: &String, state: &mut Map<String, Value>) {
        state.insert(
            title.to_string(),
            json!({
            "last_update": format!("{}", Local::now().format("%e/%b/%Y-%H:%M:%S")),
             "status":String::from("done")
            }),
        );

        write_to_file(FILE, state);
        println!("\n\n{} is being set to done\n\n", title);
    }

    fn set_to_pending(&self, title: &String, state: &mut Map<String, Value>) {
        state.insert(
            title.to_string(),
            json!({
            "last_update": format!("{}", Local::now().format("%e/%b/%Y-%H:%M:%S")),
             "status":String::from("pending")
            }),
        );
        write_to_file(FILE, state);
        println!("\n\n{} is being set to pending\n\n", title);
    }
}
