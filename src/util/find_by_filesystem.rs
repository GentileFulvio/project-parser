use std::fs;
use crate::util::{get_working_dir};

struct FindParams<'a> {
    location: &'a str,
    check: Vec<&'a str>,
    ignore: Vec<&'a str>
}

fn contains_any_strings(data: &str, strings: Vec<&str>) -> bool {
    for entry in strings {
        if data.contains(entry) {
            return true;
        }
    }

    return false;
}

fn find(params: FindParams) {
    let location = params.location;
    let check = params.check;
    let ignore = params.ignore;
    let dir = fs::read_dir(location).unwrap();
    // let mut result = Vec::new();

    for entry in dir {
        let data = entry.unwrap();
        let path = data.path();
        let path_str = path.to_str().unwrap();

        if contains_any_strings(path_str, ignore.clone()) {
            continue
        }

        if data.file_type().unwrap().is_dir() {
            let res = find(FindParams{
                ignore: ignore.clone(),
                check: check.clone(),
                location: path_str.clone()
            });

//            for m in res {
                // result.push(m.clone());
//            }
        } else {
            if contains_any_strings(path_str, check.clone()) {
                println!("{}", path_str);
                // result.push(path_str.clone());
            }
        }
    }

    // result.clone()
}

pub fn find_by_filesystem() {
    let cwd = get_working_dir().unwrap();

    let result = find(FindParams{
        location: cwd.as_str(),
        check: vec!["package.json"],
        ignore: vec!["node_modules"]
    });

//    for entry in result {
//        println!("{}", entry);
//    }
}