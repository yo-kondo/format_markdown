use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::prelude::*;
use std::path::Path;

use regex::Regex;
use walkdir::WalkDir;

#[cfg(test)]
mod main_tests;

fn main() {
    println!("対象ディレクトリを指定してください。");
    let mut target_directory: String;
    loop {
        target_directory = input_data();

        if exist_directory(&target_directory) {
            break;
        }
    }

    let md_files = get_markdown_file(&target_directory);

    for file_name in md_files {
        let lines = get_all_text(&file_name);

        let lines = add_header_after_empty_line(lines);
        let lines = delete_quote_empty(lines);
        write_file(file_name, lines);
    }
}

/// 標準入力に入力された値を返します。
fn input_data() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("E001: 標準入力の取得でエラーが発生しました。");
    input.trim().to_string()
}

/// 指定したディレクトリが存在するか確認します。
fn exist_directory(directory: &str) -> bool {
    let path = Path::new(&directory);
    if !path.exists() {
        println!("E002: 対象のディレクトリが存在しません。");
        return false;
    }

    if path.is_file() {
        println!("E003: ファイルではなくディレクトリを指定してください。");
        return false;
    }
    true
}

/// 指定したディレクトリからMarkdownファイルのリストを返します。
fn get_markdown_file(directory: &str) -> Vec<String> {
    let mut return_vec = Vec::new();

    for entry in WalkDir::new(directory) {
        let dir_entry = entry
            .expect("E004: ディレクトリ走査中にエラーが発生しました。");

        let path = dir_entry.path();
        if path.is_dir() {
            // ディレクトリは無視
            continue;
        }

        let extension = match path.extension() {
            Some(t) => t,
            // 拡張子のないファイルは無視
            None => continue,
        };
        if path.is_file() && extension == "md" {
            let path_str = path.to_str()
                .expect("E005: ディレクトリ走査中にエラーが発生しました。");

            return_vec.push(String::from(path_str));
        }
    }
    return_vec
}

/// 指定したファイルを行単位で読み込みます。
fn get_all_text(file_name: &str) -> Vec<String> {
    let mut return_vec = Vec::new();

    let file = File::open(file_name)
        .expect("E006: ファイルのオープンに失敗しました。");

    for line in BufReader::new(file).lines() {
        let s = line
            .expect("E007: ファイルの読み込みに失敗しました。");
        return_vec.push(s);
    }
    // std::io::BufRead.lines(self) は最後の改行は返されないため、最後の空行を追加する。
    return_vec.push(String::from(""));
    return_vec
}

/// 見出し（#、##、###など）の下に空行を追加します。
fn add_header_after_empty_line(lines: Vec<String>) -> Vec<String> {
    let header_regex = Regex::new(r"^#+ ").unwrap();
    let mut return_vec = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        if header_regex.is_match(&line) {
            if !lines[i + 1].is_empty() {
                return_vec.push(line.to_string());
                return_vec.push("".to_string());
                continue;
            }
        }
        return_vec.push(line.to_string());
    }
    return_vec
}

/// >だけの行は後ろのスペースを除去します。
fn delete_quote_empty(lines: Vec<String>) -> Vec<String> {
    let quote_regex = Regex::new(r"> $").unwrap();
    let mut return_vec = Vec::new();

    for line in lines {
        if quote_regex.is_match(&line) {
            return_vec.push(line.trim_end().to_string());
        } else {
            return_vec.push(line);
        }
    }
    return_vec
}

/// ファイルに書き込みます。
fn write_file(file_name: String, write_string: Vec<String>) {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_name)
        .expect("E008: ファイルのオープンに失敗しました。");

    let mut writer = BufWriter::new(file);
    for text in write_string {
        writeln!(&mut writer, "{}", text)
            .expect("E009: ファイルの書き込みに失敗しました。");
    }
}
