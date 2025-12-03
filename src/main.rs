use std::{fs, io, path::Path};

mod day1;
mod day2;
mod day3;
// mod day4;
// mod day5;
// mod day6;
// mod day7;
// mod day8;
// mod day9;
// mod day10;
// mod day11;
// mod day12;

type EntryFn = fn(&str) -> String;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let inputs = read_inputs(Path::new("./inputs"))?;
    let functions = functions();

    for (i, (name, content)) in inputs.iter().enumerate() {
        if let Some(function) = functions.get(i) {
            println!("Day{}: {}", name, function(content));
        } else {
            eprintln!("no function for input {}", name);
        }
    }
    Ok(())
}

fn functions() -> Vec<EntryFn> {
    vec![
        day1::entry,
        day2::entry,
        day3::entry,
    ]
}

fn read_inputs(dir: &Path) -> io::Result<Vec<(String, String)>> {
    let mut items = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or_default()
                .to_string();
            let content = fs::read_to_string(&path)?;
            items.push((name, content));
        }
    }

    items.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(items)
}
