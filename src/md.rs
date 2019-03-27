use chrono::Local;
use chrono::offset::TimeZone;
use regex::Regex;

use crate::info::Information;

/// markdownファイルを整形します。
pub fn format_markdown(file_name: String, text: String) -> String {
    // 改行で分割してリストに格納
    let mut lines = Vec::new();
    for line in text.lines() {
        lines.push(line.to_string());
    }

    // MarkdownLintで警告が出ないように修正
    let lines = add_header_after_empty_line(lines);
    let lines = delete_quote_empty(lines);
    let lines = change_space(lines);
    let lines = delete_two_new_line(lines);

    // 基本情報を変更
    let info = get_information(file_name, &lines);
    let lines = change_information(info);

    lines.join("\r\n")
}

/// 見出し（#、##、###など）の下に空行を追加します。
fn add_header_after_empty_line(lines: Vec<String>) -> Vec<String> {
    let header_regex = Regex::new(r"^#+ ").unwrap();
    let mut rtn_vec = Vec::new();

    for (i, v) in lines.iter().enumerate() {
        if header_regex.is_match(&v) {
            if !lines[i + 1].is_empty() {
                rtn_vec.push(v.to_string());
                rtn_vec.push("".to_string());
                continue;
            }
        }
        rtn_vec.push(v.to_string());
    }
    rtn_vec
}

/// `>`だけの行は後ろのスペースを除去します。
fn delete_quote_empty(lines: Vec<String>) -> Vec<String> {
    let quote_regex = Regex::new(r"> $").unwrap();
    let mut rtn_vec = Vec::new();

    for v in lines {
        if quote_regex.is_match(&v) {
            rtn_vec.push(v.trim_end().to_string());
        } else {
            rtn_vec.push(v);
        }
    }
    rtn_vec
}

/// 改行を`￥`から`  `（スペース2つ）に変更します。
fn change_space(lines: Vec<String>) -> Vec<String> {
    let backslash_regex = Regex::new(r"\\$").unwrap();
    let mut rtn_vec = Vec::new();

    for v in lines {
        if backslash_regex.is_match(&v) {
            rtn_vec.push(String::from(v.trim_end_matches(r"\").to_string() + "  "));
        } else {
            rtn_vec.push(v);
        }
    }
    rtn_vec
}

/// 2つ並んだ改行がある場合、1つ削除します。
fn delete_two_new_line(lines: Vec<String>) -> Vec<String> {
    let mut rtn_vec = Vec::new();
    let mut is_new_line = false;

    for v in lines {
        if v.is_empty() {
            if is_new_line {
                // 空行を追加しない
                continue;
            }
            is_new_line = true;
        } else {
            is_new_line = false;
        }
        rtn_vec.push(v.to_string());
    }
    rtn_vec
}

/// 基本情報を取得します。
fn get_information(file_name: String, lines: &Vec<String>) -> Information {
    let mut info = Information::new();
    for line in lines.to_owned() {
        // タイトル
        let index_title = "# ";
        if line.starts_with(index_title) {
            info.title = line
                .replace(index_title, "")
                .trim_end()
                .to_string();
            continue;
        }

        // 表紙の画像パス
        let index_front_image = "![表紙](";
        if line.starts_with(index_front_image) {
            info.front_image.push(line
                .trim_end()
                .to_string());
            continue;
        }

        // 読了日
        let index_reading_date = "読了日 : ";
        if line.starts_with(index_reading_date) {
            let date = line.replace(&index_reading_date, "");
            let date = Local.datetime_from_str(
                &format!("{} {}", date, "00:00:00"), "%Y/%m/%d %H:%M:%S");
            let date = match date {
                Ok(t) => t,
                Err(e) => panic!(
                    "読了日の日付変換でエラーが発生しました。ファイル名: {}\n{}", &file_name, e),
            };
            info.reading_date = date;
            continue;
        }

        // 著者
        let index_author = "著者 : ";
        if line.starts_with(index_author) {
            info.author = line
                .replace(index_author, "")
                .trim_end()
                .to_string();
            continue;
        }

        // 出版社
        let index_publisher = "出版社 : ";
        if line.starts_with(index_publisher) {
            info.publisher = line
                .replace(index_publisher, "")
                .trim_end()
                .to_string();
        }

        // ISBN-10
        let index_isbn10 = "ISBN-10 : ";
        if line.starts_with(index_isbn10) {
            info.isbn10 = line
                .replace(index_isbn10, "")
                .replace("－", "-")
                .trim_end()
                .to_string();;
        }

        // ISBN-13
        let index_isbn13 = "ISBN-13 : ";
        if line.starts_with(index_isbn13) {
            info.isbn13 = line
                .replace(index_isbn13, "")
                .replace("－", "-")
                .trim_end()
                .to_string();
        }

        // ASIN
        let index_asin = "ASIN : ";
        if line.starts_with(index_asin) {
            info.asin = line
                .replace(index_asin, "")
                .replace("－", "-")
                .trim_end()
                .to_string();
        }

        // 発売日
        let index_release_date = "発売日 : ";
        if line.starts_with(index_release_date) {
            let date = line.replace(&index_release_date, "");
            let date = Local.datetime_from_str(
                &format!("{} {}", date, "00:00:00"), "%Y/%m/%d %H:%M:%S");
            let date = match date {
                Ok(t) => t,
                Err(e) => panic!(
                    "発売日の日付変換でエラーが発生しました。ファイル名: {}\n{}", &file_name, e),
            };
            info.release_date = date;
            continue;
        }

        // Amazon URL
        let index_link = "Amazon : ";
        if line.starts_with(index_link) {
            info.link = line
                .replace(index_link, "")
                .trim_end()
                .to_string();
        }

        // タグ
        let index_tags = "その他 : ";
        if line.starts_with(index_tags) {
            info.tags = line.replace(index_tags, "");
        }
    }
    info
}

/// 元の基本情報を削除し、新しい基本情報を追加します。
fn change_information(info: Information) -> Vec<String> {
    unimplemented!();
}

mod test {
    use chrono::Local;
    use chrono::offset::TimeZone;

    use crate::info::Information;
    use crate::md::*;

    #[test]
    fn test() {
        // TODO: デバッグ用のtest

        let info = Information::new();
        let mut info2 = Information::new();
        info2.title = "hoge".to_string();

        let info_str = format!("{:?}", info);
        let info_str2 = format!("{:?}", info2);

//        assert_eq!(info_str, info_str2);
    }

    #[test]
    fn add_header_after_empty_line_test1() {
        let input = vec![
            String::from("# header1"),
            String::from("## header2"),
            String::from(""),
            String::from("## header3"),
            String::from(""),
            String::from(""),
        ];
        let expected = vec![
            String::from("# header1"),
            String::from(""),
            String::from("## header2"),
            String::from(""),
            String::from("## header3"),
            String::from(""),
            String::from(""),
        ];
        let actual = add_header_after_empty_line(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn delete_quote_empty_test1() {
        let input = vec![
            String::from("> "),
            String::from(""),
            String::from("> aaa"),
            String::from("> "),
            String::from("> bbb"),
            String::from(">"),
            String::from(""),
            String::from("  > "),
            String::from("  >"),
            String::from(""),
        ];
        let expected = vec![
            String::from(">"),
            String::from(""),
            String::from("> aaa"),
            String::from(">"),
            String::from("> bbb"),
            String::from(">"),
            String::from(""),
            String::from("  >"),
            String::from("  >"),
            String::from(""),
        ];
        let actual = delete_quote_empty(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn change_space_test1() {
        let input = vec![
            String::from(r"aaa\"),
            String::from(r""),
            String::from(r"> aaa\"),
        ];
        let expected = vec![
            String::from(r"aaa  "),
            String::from(r""),
            String::from(r"> aaa  "),
        ];
        let actual = change_space(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn delete_two_new_line_test1() {
        let input = vec![
            String::from("aaa"),
            String::from(""),
            String::from(""),
            String::from("> bbb"),
            String::from(""),
        ];
        let expected = vec![
            String::from("aaa"),
            String::from(""),
            String::from("> bbb"),
            String::from(""),
        ];
        let actual = delete_two_new_line(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_information_test1() {
        let input = vec![
            String::from("# タイトル"),
            String::from(""),
            String::from("## 基本情報"),
            String::from(""),
            String::from("表紙 :  "),
            String::from("![表紙](画像のパス)  "),
            String::from("著者 : 著者の名前  "),
            String::from("出版社 : 出版社の名前  "),
            String::from("ISBN-10 : 1234567890  "),
            String::from("ISBN-13 : 123-1234567890  "),
            String::from("ASIN : ABCDEFGHIJ  "),
            String::from("発売日 : 2018/01/02  "),
            String::from("Amazon : [amazonへのリンク](https://example.com)  "),
            String::from("読了日 : 2019/03/04  "),
            String::from("その他 : タグ1, タグ2"),
        ];
        let expected = Information {
            title: "タイトル".to_string(),
            front_image: vec![
                "![表紙](画像のパス)".to_string(),
            ],
            reading_date: Local.ymd(2019, 3, 4).and_hms(0, 0, 0),
            author: "著者の名前".to_string(),
            publisher: "出版社の名前".to_string(),
            isbn10: "1234567890".to_string(),
            isbn13: "123-1234567890".to_string(),
            asin: "ABCDEFGHIJ".to_string(),
            release_date: Local.ymd(2018, 1, 2).and_hms(0, 0, 0),
            link: "[amazonへのリンク](https://example.com)".to_string(),
            tags: "タグ1, タグ2".to_string(),
        };
        let actual = get_information("file_name".to_string(),&input);
        assert_eq!(expected, actual);
    }
}
