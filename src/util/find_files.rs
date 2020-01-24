use std::fs;

pub struct FindParams<'a> {
    pub location: &'a str,
    pub check: Vec<&'a str>,
    pub ignore: Vec<&'a str>
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

pub fn find_by_string(params: FindParams) -> Vec<String> {
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
            let res = find_by_string(FindParams{
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