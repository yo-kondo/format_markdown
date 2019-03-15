#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::{fs, io};
use std::io::Write;

mod md;

/// Settings構造体
#[derive(Debug, Deserialize)]
struct Settings {
    target_dir: String,
}

/// エントリポイント
fn main() {
    let setting_data = get_settings_toml(String::from("./settings.toml"));
    let md_files = get_markdown_files(setting_data.target_dir);

    for file_name in md_files {
        // ファイル読み込み
        let text = match fs::read_to_string(&file_name) {
            Ok(t) => t,
            Err(e) => panic!(
                "ファイル読み込みでエラーが発生しました。ファイル名: {}\n{}", &file_name, e),
        };
        let format_text = md::format_markdown(text);

        // ファイル書き込み
        let write_file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&file_name)
            .unwrap();
        let mut buf_writer = io::BufWriter::new(write_file);
        match buf_writer.write(format_text.as_bytes()) {
            Ok(_) => {}
            Err(e) => panic!(
                "ファイル書き込みでエラーが発生しました。ファイル名: {}\n{}", &file_name, e),
        }
    }
}

/// Settings.tomlをデシリアライズします。
fn get_settings_toml(file_name: String) -> Settings {
    let read_line = match fs::read_to_string(file_name) {
        Ok(t) => t,
        Err(e) => panic!("設定ファイルの読み込みでエラーが発生しました。\n{}", e),
    };
    return match toml::from_str(&read_line) {
        Ok(t) => t,
        Err(e) => panic!("設定ファイルのデシリアライズでエラーが発生しました。\n{}", e),
    };
}

/// 対象のディレクトリからMarkdownファイルのリスト取得します。
fn get_markdown_files(dir: String) -> Vec<String> {
    let mut rtn_vec = Vec::new();

    for val in walkdir::WalkDir::new(dir) {
        let path = match val {
            Ok(t) => t,
            Err(e) => panic!("ディレクトリの走査でエラーが発生しました。\n{}", e),
        };
        let path = path.path();
        if path.is_dir() {
            // ディレクトリは無視
            continue;
        }
        let extension = match path.extension() {
            Some(t) => t,
            // 拡張子のないファイルは無視
            None => continue,
        };
        if extension != "md" {
            continue;
        }

        let path_str = match path.to_str() {
            Some(t) => t,
            None => continue,
        };
        rtn_vec.push(String::from(path_str));
    }
    rtn_vec
}

mod test {
    #[test]
    fn get_settings_toml_test1() {
        use crate::get_settings_toml;
        let set = get_settings_toml(String::from(r".\test_data\settings.toml"));
        assert_eq!(set.target_dir, String::from(r"c:/temp"));
    }

    #[test]
    fn get_markdown_files_test1() {
        use crate::get_markdown_files;
        let expected = vec![
            String::from(r".\test_data\test1\dir1\dir2\file3.md"),
            String::from(r".\test_data\test1\dir1\dir2\file5.md"),
            String::from(r".\test_data\test1\dir1\file2.md"),
        ];
        let actual = get_markdown_files(String::from(r".\test_data\test1"));
        assert_eq!(expected, actual);
    }
}
