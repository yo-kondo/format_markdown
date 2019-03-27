use chrono::{Date, DateTime, Local};
use chrono::offset::TimeZone;

/// 基本情報
#[derive(Debug, PartialEq)]
pub struct Information {
    /// タイトル
    pub title: String,
    /// 表紙の画像パス
    pub front_image: Vec<String>,
    /// 読了日
    pub reading_date: DateTime<Local>,
    /// 著者
    pub author: String,
    /// 出版社
    pub publisher: String,
    /// ISBN-10
    pub isbn10: String,
    /// ISBN-13
    pub isbn13: String,
    /// ASIN
    pub asin: String,
    /// 発売日
    pub release_date: DateTime<Local>,
    /// Amazon URL
    pub link: String,
    /// タグ
    pub tags: String,
}

impl Information {
    /// 構造体を初期化する関連関数。
    pub fn new() -> Information {
        // TODO: Date<Local>の最小値がわからない
        let y = 1900;
        let m = 1;
        let d = 1;

        Information {
            title: "".to_string(),
            front_image: Vec::new(),
            reading_date: Local.ymd(y, m, d).and_hms(0, 0, 0),
            author: "".to_string(),
            publisher: "".to_string(),
            isbn10: "".to_string(),
            isbn13: "".to_string(),
            asin: "".to_string(),
            release_date: Local.ymd(y, m, d).and_hms(0, 0, 0),
            link: "".to_string(),
            tags: "".to_string(),
        }
    }
}