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

    false
}

fn contains_all_strings(data: &str, strings: Vec<&str>) -> bool {
    for entry in strings {
        if !data.contains(entry) {
            return false;
        }
    }

    true
}

fn find(params: FindParams) -> Vec<String> {
    let location = params.location;
    let check = params.check;
    let ignore = params.ignore;
    let dir = fs::read_dir(location).unwrap();
    let mut result = Vec::new();

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

            for m in res {
                 result.push(m);
            }
        } else {
            if contains_all_strings(path_str, check.clone()) {
                result.push(String::from(path_str));
            }
        }
    }

     return result
}

pub fn find_by_filesystem() {
    let cwd = get_working_dir().unwrap();

    let packages_result = find(FindParams{
        location: cwd.as_str(),
        check: vec!["package.json", "packages"],
        ignore: vec!["node_modules"]
    });

    let services_result = find(FindParams{
        location: cwd.as_str(),
        check: vec!["package.json", "services"],
        ignore: vec!["node_modules"]
    });

    services_result.into_iter().for_each(|x| {
        println!("{}", fs::read_to_string(x).unwrap());
    });
}