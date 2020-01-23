mod util;
use std::fs;

fn find(location: &str) {
    let dir = fs::read_dir(location).unwrap();
    
    for entry in dir {
        let data = entry.unwrap();
        let path = data.path();
        let path_str = path.to_str().unwrap();

        if path_str.contains("node_modules") {
            continue
        }

        if data.file_type().unwrap().is_dir() {
            find(path_str)
        } else {
            if path_str.contains("package.json") {
                println!("{}", path_str)
            }
        }
    }
}

fn find_by_filesystem() {
    let cwd = util::get_working_dir().unwrap();

    find(cwd.as_str());
}

fn main() {
    find_by_filesystem();
}
