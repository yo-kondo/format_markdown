# format_markdown
Markdownをmarkdownlintにあわせて修正します。

## 詳細仕様

* 指定したディレクトリ配下のmarkdownファイルをすべて読み込む。
* 見出し（#、##、###など）の下に空行を追加する。
* `>`だけの行は後ろのスペースを除去する。
* 改行を `\` から `  ` （スペース2つ）に変更する。
* 2つ並んだ改行がある場合、1つ削除する。
* 基本情報を修正する。

## 設定ファイル
プロジェクトルートに「settings.toml」を作成する。

``` toml
[Settings]
targetDir="c:/temp" # 変換対象のディレクトリ
```

## 基本情報の修正

``` markdown
# タイトル

## 基本情報

表紙 :\
![表紙](画像のパス)\
著者 : 著者\
出版社 : 出版社\
ISBN-10 : 9999999999\
ISBN-13 : 999-9999999999\
ASIN : XXXXXXXXXX\
発売日 : yyyy/mm/dd\
Amazon : [amazonへのリンク]()\
読了日 : yyyy/mm/dd\
その他 : 
```

上記テキストを以下のテキストに変換する。

``` markdown
# タイトル

![画像](画像のパス)

---

* Date: yyyy/mm/dd
* Author: 著者
* Publisher: 出版社
* ISBN-10: 9999999999
* ISBN-13: 999-9999999999
* ASIN: XXXXXXXXXX
* ReleaseDate: yyyy/mm/dd
* Link: [amazonへのリンク]()
* Tags:

---
```
