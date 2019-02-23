#[cfg(test)]
mod main_tests {
    use crate::*;

    #[test]
    fn get_markdown_file_test() {
        let got = get_markdown_file(r"./test_data\no001");
        assert_eq!(3, got.len());
        assert_eq!(got[0], r"./test_data\no001\dir1\test1.md");
        assert_eq!(got[1], r"./test_data\no001\dir1\test2.md");
        assert_eq!(got[2], r"./test_data\no001\test4.md");
    }

    #[test]
    fn get_all_text_test() {
        let got = get_all_text(r"./test_data\no001\dir1\test1.md");
        assert_eq!(10, got.len());
        assert_eq!(got[0], "# head1");
        assert_eq!(got[1], "## head2");
        assert_eq!(got[2], "line1");
        assert_eq!(got[3], "");
        assert_eq!(got[4], "> line2");
        assert_eq!(got[5], "> ");
        assert_eq!(got[6], "> line3");
        assert_eq!(got[7], "");
        assert_eq!(got[8], "");
        assert_eq!(got[9], "");
    }

    #[test]
    fn add_header_after_empty_line_test() {
        let got = add_header_after_empty_line(
            vec![
                String::from("# head1"),
                String::from("## head2"),
                String::from("line1"),
                String::from(""),
                String::from("> line2"),
                String::from("> "),
                String::from("> line3"),
                String::from(""),
                String::from("## head3"),
                String::from(""),
                String::from("line4"),
                String::from(""),
                String::from(""),
            ]);
        assert_eq!(15, got.len());
        assert_eq!(got[0], "# head1");
        assert_eq!(got[1], "");
        assert_eq!(got[2], "## head2");
        assert_eq!(got[3], "");
        assert_eq!(got[4], "line1");
        assert_eq!(got[5], "");
        assert_eq!(got[6], "> line2");
        assert_eq!(got[7], "> ");
        assert_eq!(got[8], "> line3");
        assert_eq!(got[9], "");
        assert_eq!(got[10], "## head3");
        assert_eq!(got[11], "");
        assert_eq!(got[12], "line4");
        assert_eq!(got[13], "");
        assert_eq!(got[14], "");
    }

    #[test]
    fn delete_quote_empty_test() {
        let got = delete_quote_empty(
            vec![
                String::from("# head1"),
                String::from("## head2"),
                String::from("line1"),
                String::from(""),
                String::from("> line2"),
                String::from("> "),
                String::from("> line3"),
                String::from(""),
                String::from("## head3"),
                String::from(""),
                String::from("line4"),
                String::from(""),
                String::from(""),
            ]);
        assert_eq!(13, got.len());
        assert_eq!(got[0], "# head1");
        assert_eq!(got[1], "## head2");
        assert_eq!(got[2], "line1");
        assert_eq!(got[3], "");
        assert_eq!(got[4], "> line2");
        assert_eq!(got[5], ">");
        assert_eq!(got[6], "> line3");
        assert_eq!(got[7], "");
        assert_eq!(got[8], "## head3");
        assert_eq!(got[9], "");
        assert_eq!(got[10], "line4");
        assert_eq!(got[11], "");
        assert_eq!(got[12], "");
    }
}
