extern crate glob;
mod util;
use glob::{glob, MatchOptions, glob_with};
use std::{thread, time};
use std::sync::mpsc;
use std::time::Duration;

fn writeDataToFile(line: &str, file: &str) {

}

fn search() {
    let pattern = format!("/**/package.json");
    let options = MatchOptions{
        case_sensitive: false,
        require_literal_leading_dot: false,
        require_literal_separator: false
    };

//    let (sender, receiver) = mspc::channel();
//
//    thread::Builder::new().name("child1".to_string()).spawn(move || {
//
//    });

    let now = time::Instant::now();

	for entry in glob_with(pattern.as_str(), options).expect("Failed to read glob pattern") {
        thread::sleep(Duration::from_secs(1));
        println!("{}", now.elapsed().as_secs());

        match entry {
            Ok(path) => {
                match path.to_str() {
                    Some(data) => {
                        // if !data.contains("node_modules") && !data.starts_with("tools") {
                            println!("{}", data)
                        // }
                    },
                    None => println!("Unable to stringify buffer")
                }
            },
            Err(e) => println!("{:?}", e),
        }
    }
}

fn main() {
    match util::get_working_dir() {
        Some(cwd) => {
            println!("{}", cwd.as_str());
        },
        None => println!("Unable to retrieve current working directory")
    }

   search();
}
