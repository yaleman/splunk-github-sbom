use serde_json::Value;
use serde_yaml;

fn main() {
    let filename = "action.yml";

    // read filename to string
    let contents = std::fs::read_to_string(filename).expect("Something went wrong reading the file");

    let yaml: Value = serde_yaml::from_str(&contents).unwrap();

    let file = yaml.as_object().unwrap();

    let inputs = file.get("inputs").unwrap().as_object().unwrap();

    let rows: Vec<String> = inputs.into_iter().map(|(key, value)| {
        println!("Key: {key:?} value: {value:?}");

        format!("|{}|{}|{}|{:?}|",
            key,
            value.get("description").unwrap().as_str().unwrap(),
            value.get("required").unwrap_or(&Value::Bool(true)).as_bool().unwrap(),
            value.get("default").unwrap_or(&Value::Null).as_str().unwrap_or(""),
        )
    }).collect();
    rows.into_iter().for_each(|r| println!("{}", r));
}