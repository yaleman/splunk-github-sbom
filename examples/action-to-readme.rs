use serde_json::Value;
use serde_yaml;

fn main() {
    let filename = "action.yml";

    // read filename to string
    let contents = std::fs::read_to_string(filename).expect("Something went wrong reading the file");

    let yaml: Value = serde_yaml::from_str(&contents).unwrap();

    let file = yaml.as_object().unwrap();

    let inputs = file.get("inputs").unwrap().as_object().unwrap();


    let mut rows: Vec<Vec<String>> = inputs.into_iter().map(|(key, value)| {
        // println!("Key: {key:?} value: {value:?}");
        vec![
            key.to_string(),
            value.get("description").unwrap().as_str().unwrap().to_string(),
            value.get("required").unwrap_or(&Value::Bool(true)).as_bool().unwrap().to_string(),
            value.get("default").unwrap_or(&Value::Null).as_str().unwrap_or("").to_string(),
        ]
    }).collect();

    rows.insert(0, vec![
        "Argument".to_string(),
        "Description".to_string(),
        "Required".to_string(),
        "Default".to_string(),
        ]);

    let mut col_stats: Vec<usize>  = vec![0;rows.first().unwrap().len()];

    rows.iter().for_each(|row| {
        // for each column in the row, if the length of the column is greater than the current max in the col_stats, update the col_stats
        row.iter().enumerate().for_each(|(i, col)| {
            if &col.len() > col_stats.get(i).unwrap_or(&1) {
                col_stats[i] = col.len();
            }

        })
    });
    // eprintln!("col stats: {:?}", col_stats);

    let mut result_str = String::new();

    rows.into_iter().enumerate().for_each(|(row_i, row)| {
        result_str.push_str("|");
        row.into_iter().enumerate().for_each(|(i, col)| {
            let width = col_stats[i];
            result_str.push_str(&format!(" {:<width$} |", col));
        });
        result_str.push_str("\n");
        if row_i == 0 {
            result_str.push_str("|");
            col_stats.iter().for_each(|width| {
                result_str.push_str(&format!(" {:<width$} |", "-".repeat(*width)));
            });
            result_str.push_str("\n");
        }
    });
    println!("{}", result_str);
}