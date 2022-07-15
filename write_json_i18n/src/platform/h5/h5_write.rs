use serde_json::{self, json, Error};
use serde::Serialize;
use std::env::{self, args};
use std::fs::{self, File};
use std::io::{self, BufRead };
use std::os::unix::prelude::FileExt;
use std::path::Path;
use std::vec;
use std::thread;

use crate::LANG;

fn check_file(stdin_lines: Vec<String>, content: &Vec<Vec<String>>) -> io::Result<()> {
    let base_path = String::from("");
    let dir_suffix = &stdin_lines[1];
    let dir_name = base_path + dir_suffix.as_str();
    if !Path::new(&dir_name).exists() {
        fs::create_dir(&dir_name)?;
    }
    let mut pool = vec![];
    for (index, lang) in LANG.into_iter().enumerate() {
        let file_name = dir_name.clone() + "/" + lang + ".json";
        let c = content.to_vec();
        pool.push(thread::spawn(move || {
            if !Path::new(&file_name).exists() {
                let file = File::create(&file_name).unwrap();
                file.write_at(b"{}", 0).unwrap();
            }
            if c.len() - 1 > index {
                write_json(file_name, &c[0], &c[index + 1]);
            }
        }));
    }
    for child in pool {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
    Ok(())
}
fn write_json(file_name: String, keys: &Vec<String>, content: &Vec<String>) {
    let data = fs::read_to_string(&file_name).unwrap();
    let mut json: serde_json::Value = serde_json::from_str(&data).unwrap();
    for (i, v) in keys.iter().enumerate() {
        json[v] = json!(content[i]);
    }
    // println!("{:#?}", json);
    let file = File::create(file_name).unwrap();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(&file, formatter);
    json.serialize(&mut ser).unwrap();
    // serde_json::to_writer_pretty(&file, &json).unwrap();
}

pub fn write_h5_with_wiki() {
    let stdin_lines: Vec<_> = args().collect();
    let file = fs::File::open("./file.txt").unwrap();
    let reader = io::BufReader::new(file);
    let mut v: Vec<Vec<String>> = vec![];
    for _ in LANG {
        v.push(vec![]);
    }
    for line in reader.lines() {
        if let Ok(line) = line {
            let item: Vec<String> = line
                .split("😈")
                .enumerate()
                .map(|(i, x)| {
                    let mut x = x.trim().to_string();
                    if i == 0 {
                        let mut pre_fix = String::new();
                        if stdin_lines.len() > 2 {
                            pre_fix = stdin_lines[2].clone() + ".";
                        }
                        x.insert_str(0, pre_fix.as_str());
                    }
                    x
                })
                .collect();
            for (ii, vv) in item.iter().enumerate() {
                v[ii].push(vv.to_string());
            }
        }
    }

    let re = check_file(stdin_lines, &v);
    match re {
        Ok(_) => {}
        Err(e) => {
            panic!("io error {:#?}", &e);
        }
    };
}
