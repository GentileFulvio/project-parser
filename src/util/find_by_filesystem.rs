extern crate serde_json;
use std::fs;
use crate::util::{get_working_dir};
use serde_json::{Value};
use std::ops::Add;

pub fn get_package_as_json(path: &str) -> Value {
    let package_json = fs::read_to_string(path).unwrap();
    let json_object: Value = serde_json::from_str(package_json.as_str()).unwrap();

    return json_object;
}

pub fn extract_dependencies(value: Value) -> Value {
    let dependencies = value["dependencies"].clone();

    return dependencies;
}

pub fn has_property(value: Value, property: &str) -> bool {
    match value.get(property) {
        Some(res) => true,
        None => false
    }
}

pub fn extract_package_name(value: Value) -> Value {
    return value["name"].clone();
}

pub fn find_by_filesystem(package_files: Vec<String>, service_files: Vec<String>) {
    let cwd = get_working_dir().unwrap();

    let service_names = service_files.clone().into_iter().map(|x| {
        let json_object = get_package_as_json(x.as_str());

        (extract_package_name(json_object.clone()), extract_dependencies(json_object.clone()))
    });
    let package_names = package_files.clone().into_iter().map(|x| {
        let json_object = get_package_as_json(x.as_str());

        extract_package_name(json_object)
    });

    println!("========= Services {} =========", service_files.iter().count());

    for (name, deps) in service_names {
        let package_dependencies = package_files.clone().into_iter().fold(String::new(), |acc, p| {
            let json_object = get_package_as_json(p.as_str());

            let name = extract_package_name(json_object);

            let depends_on_package = has_property(deps.clone(), name.as_str().unwrap());

            return if depends_on_package {
                acc.add(name.as_str().unwrap())
            } else {
                acc
            }
        });

        println!("Service: {}", name.as_str().unwrap());
        if package_dependencies.len() > 0 {
            println!("\t Depends on |");
            println!("\t \t {}", package_dependencies);
        }
    }

    println!("========= Packages {} =========", package_files.iter().count());

    for entry in package_names {
        println!("Package: {}", entry.as_str().unwrap())
    }
}