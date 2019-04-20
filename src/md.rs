use regex::Regex;

lazy_static! {
        static ref HEADER_REGEX: Regex = {
            Regex::new(r"^#+ ").unwrap()
        };
        static ref QUOTE_REGEX: Regex = {
            Regex::new(r"> $").unwrap()
        };
        static ref BACKSLASH_REGEX: Regex = {
            Regex::new(r"\\$").unwrap()
        };
        static ref FRONT_IMAGE: Regex = {
            Regex::new(r"!\[表紙.*]\(").unwrap()
        };
    }

/// markdownファイルを整形します。
pub fn format_markdown(text: String) -> String {
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

    lines.join("\r\n")
}

/// 見出し（#、##、###など）の下に空行を追加します。
fn add_header_after_empty_line(lines: Vec<String>) -> Vec<String> {
    let mut rtn_vec = Vec::new();

    for (i, v) in lines.iter().enumerate() {
        if HEADER_REGEX.is_match(&v) {
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
    let mut rtn_vec = Vec::new();

    for v in lines {
        if QUOTE_REGEX.is_match(&v) {
            rtn_vec.push(v.trim_end().to_string());
        } else {
            rtn_vec.push(v);
        }
    }
    rtn_vec
}

/// 改行を`￥`から`  `（スペース2つ）に変更します。
fn change_space(lines: Vec<String>) -> Vec<String> {
    let mut rtn_vec = Vec::new();

    for v in lines {
        if BACKSLASH_REGEX.is_match(&v) {
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

#[cfg(test)]
mod test {
    use crate::md::*;

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
}
