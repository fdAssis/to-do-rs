use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use serde_json::value::Value;
use serde_json::Map;
pub trait Get {
    fn get(&self, title: &String, state: &Map<String, Value>) {
        let mut table = Table::new();

        let item: Option<&Value> = state.get(title);

        match item {
            Some(result) => {
                let status = result.get("status").unwrap().to_string().replace('\"', "");
                let last_update = result
                    .get("last_update")
                    .unwrap()
                    .to_string()
                    .replace('\"', "");

                let status = match status.as_str() {
                    "done" => Cell::new("Done")
                        .add_attribute(Attribute::Bold)
                        .fg(Color::Green)
                        .set_alignment(CellAlignment::Center),
                    _ => Cell::new("Pending")
                        .add_attributes(vec![Attribute::Bold, Attribute::SlowBlink])
                        .fg(Color::Red)
                        .set_alignment(CellAlignment::Center),
                };

                table
                    .load_preset(UTF8_FULL)
                    .set_content_arrangement(ContentArrangement::DynamicFullWidth)
                    .set_header(vec![
                        Cell::new("To-Do")
                            .add_attribute(Attribute::Bold)
                            .fg(Color::Black)
                            .bg(Color::White)
                            .set_alignment(CellAlignment::Center),
                        Cell::new("Status")
                            .add_attribute(Attribute::Bold)
                            .fg(Color::Black)
                            .bg(Color::White)
                            .set_alignment(CellAlignment::Center),
                        Cell::new("Last Update")
                            .add_attribute(Attribute::Bold)
                            .fg(Color::Black)
                            .bg(Color::White)
                            .set_alignment(CellAlignment::Center),
                    ])
                    .add_row(vec![
                        Cell::new(title)
                            .add_attribute(Attribute::Bold)
                            .fg(Color::Yellow)
                            .set_alignment(CellAlignment::Center),
                        status,
                        Cell::new(last_update)
                            .add_attribute(Attribute::Bold)
                            .fg(Color::Green)
                            .set_alignment(CellAlignment::Center),
                    ]);

                println!("{table}");
            }
            None => println!("item: {} was not found", title),
        }
    }
    fn get_all(&self, state: &Map<String, Value>) {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::DynamicFullWidth)
            .set_header(vec![
                Cell::new("To-Do")
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Black)
                    .bg(Color::White)
                    .set_alignment(CellAlignment::Center),
                Cell::new("Status")
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Black)
                    .bg(Color::White)
                    .set_alignment(CellAlignment::Center),
                Cell::new("Last Update")
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Black)
                    .bg(Color::White)
                    .set_alignment(CellAlignment::Center),
            ]);
        for item in state.iter() {
            let status = item.1.get("status").unwrap().to_string().replace('\"', "");
            let last_update = item
                .1
                .get("last_update")
                .unwrap()
                .to_string()
                .replace('\"', "");

            let status = match status.as_str() {
                "done" => Cell::new("Done")
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Green)
                    .set_alignment(CellAlignment::Center),
                _ => Cell::new("Pending")
                    .add_attributes(vec![Attribute::Bold, Attribute::SlowBlink])
                    .fg(Color::Red)
                    .set_alignment(CellAlignment::Center),
            };

            table.add_row(vec![
                Cell::new(item.0)
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Yellow)
                    .set_alignment(CellAlignment::Center),
                status,
                Cell::new(last_update)
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Green)
                    .set_alignment(CellAlignment::Center),
            ]);
        }
        println!("{table}");
    }
}
