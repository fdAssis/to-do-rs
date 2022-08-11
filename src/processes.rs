use serde_json::value::Value;
use serde_json::Map;

use super::to_do::structs::done::Done;
use super::to_do::structs::pending::Pending;
use super::to_do::structs::traits::create::Create;
use super::to_do::structs::traits::delete::Delete;
use super::to_do::structs::traits::edit::Edit;
use super::to_do::structs::traits::get::Get;
use super::to_do::ItemType;

fn _process_pending(item: Pending, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();

    match command.as_str() {
        "get" => item.get(&item.super_struct.title, &state),
        "get_all" => item.get_all(&state),
        "create" => item.create(
            &item.super_struct.title,
            &item.super_struct.status,
            &item.super_struct.last_update,
            &mut state,
        ),
        "delete" => item.delete(&item.super_struct.title, &mut state),
        "edit" => item.set_to_done(&item.super_struct.title, &mut state),
        _ => println!("command: {} not supported", command),
    }
}

fn _process_done(item: Done, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();

    match command.as_str() {
        "get" => item.get(&item.super_struct.title, &state),
        "get_all" => item.get_all(&state),
        "delete" => item.delete(&item.super_struct.title, &mut state),
        "edit" => item.set_to_pending(&item.super_struct.title, &mut state),
        _ => println!("command: {} not supported", command),
    }
}

pub fn process_input(item: ItemType, command: String, state: &Map<String, Value>) {
    match item {
        ItemType::Peding(item) => _process_pending(item, command, state),
        ItemType::Done(item) => _process_done(item, command, state),
    }
}
