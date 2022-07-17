use serde::Serialize;
use serde_json::{self, json};
use std::env::{args};
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::os::unix::prelude::FileExt;
use std::path::Path;
use std::{vec};

use crate::LANG;

fn check_file(stdin_lines: Vec<String>, content: &Vec<Vec<String>>) -> io::Result<()> {
    let base_path = String::from("");
    let dir_suffix = &stdin_lines[1];
    let file_name = base_path + dir_suffix.as_str();
    if !Path::new(&file_name).exists() {
        let file = File::create(file_name.clone()).unwrap();
        file.write_at(b"{}", 0).unwrap();
    }
    let c = content.to_vec();
    println!("{:#?}", c);
    write_json(file_name.clone(), &c);
    Ok(())
}
fn write_json(file_name: String, content: &Vec<Vec<String>>) {
    let data = fs::read_to_string(&file_name).unwrap();
    let mut json: serde_json::Value = serde_json::from_str(&data).unwrap();
    let f = &content[0];
    for (i, v) in LANG.into_iter().enumerate() {
        for (ii, vv) in f.into_iter().enumerate() {
            let mut num = i;
            if i > content.len() - 2 {
                // 英文
                num = 1;
            }
            json[v][vv] = json!(&content[num + 1][ii]);
        }
    }
    println!("{:#?}", json);
    let file = File::create(file_name).unwrap();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(&file, formatter);
    json.serialize(&mut ser).unwrap();
}

pub fn write_website_with_wiki() {
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
