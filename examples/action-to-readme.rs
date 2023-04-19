use regex::Regex;
use serde_json::Value;
use serde_yaml;
use std::path::PathBuf;

fn replace_in_file(replacement: String, filename: String) {
    // load the file into a string
    let file_contents =
        std::fs::read_to_string(&filename).expect("Something went wrong reading the file");

    // use regular expressions to search for the string indexes between <!-- arguments table start --> and <!-- arguments table end -->
    let re = Regex::new(
        r"<!-- arguments table start -->(?P<to_replace>(?s)(.*))<!-- arguments table end -->",
    )
    .unwrap();

    // replace the string between those indexes with the replacement string
    let result = match re.captures(&file_contents) {
        Some(val) => val,
        None => panic!("Could not find the tags in the readme to replace!"),
    };

    let range = match result.name("to_replace") {
        Some(val) => val.range(),
        None => panic!("Could not find the to_replace named group!"),
    };

    let string_start = file_contents.split_at(range.start).0;
    let string_end = file_contents.split_at(range.end).1;

    let file_result = format!("{}\n{}\n{}", string_start, replacement, string_end);

    // write out the file
    std::fs::write(PathBuf::from(filename), file_result).expect("Unable to write file");
}

fn main() {
    let filename = "action.yml";

    // read filename to string
    let contents =
        std::fs::read_to_string(filename).expect("Something went wrong reading the file");

    let yaml: Value =
        serde_yaml::from_str(&contents).expect(&format!("Couldn't parse {}", filename));

    let file = yaml
        .as_object()
        .expect(&format!("Couldn't parse {}", filename));

    let inputs = file
        .get("inputs")
        .expect(&format!("Couldn't parse {}", filename))
        .as_object()
        .unwrap();

    // pull the file data
    let mut rows: Vec<Vec<String>> = inputs
        .into_iter()
        .map(|(key, value)| {
            vec![
                key.to_string(),
                value
                    .get("description")
                    .unwrap()
                    .as_str()
                    .expect("Description failed to parse!")
                    .to_string(),
                value
                    .get("required")
                    .unwrap_or(&Value::Bool(true))
                    .as_bool()
                    .unwrap()
                    .to_string(),
                value
                    .get("default")
                    .unwrap_or(&Value::Null)
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
            ]
        })
        .collect();

    // headers
    rows.insert(
        0,
        vec![
            "Argument".to_string(),
            "Description".to_string(),
            "Required".to_string(),
            "Default".to_string(),
        ],
    );

    // work out the length of each column
    let mut col_stats: Vec<usize> = vec![0; rows.first().unwrap().len()];
    rows.iter().for_each(|row| {
        // for each column in the row, if the length of the column is greater than the current max in the col_stats, update the col_stats
        row.iter().enumerate().for_each(|(i, col)| {
            if &col.len() > col_stats.get(i).unwrap_or(&1) {
                col_stats[i] = col.len();
            }
        })
    });

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
    replace_in_file(result_str, "README.md".to_string());
    eprintln!("Successfully updated file!");
}
