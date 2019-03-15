/// markdownファイルを整形します。
pub fn format_markdown(lines: String) -> String {
    let text = add_header_after_empty_line(text);
    let text = delete_quote_empty(text);
    let text = change_space(text);
    let text = delete_two_new_line(text);
    unimplemented!();
}

/// 見出し（#、##、###など）の下に空行を追加します。
fn add_header_after_empty_line(text: String) -> String {
    unimplemented!();
}

/// >だけの行は後ろのスペースを除去します。
fn delete_quote_empty(text: String) -> String {
    unimplemented!();
}

/// 改行を `\` から `  ` （スペース2つ）に変更します。
fn change_space(text: String) -> String {
    unimplemented!();
}

/// 2つ並んだ改行がある場合、1つ削除します。
fn delete_two_new_line(text: String) -> String {
    unimplemented!();
}
