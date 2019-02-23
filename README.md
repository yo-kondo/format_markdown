# format_markdown
Markdownをmarkdownlintにあわせて修正します。

## 詳細仕様
* 指定したディレクトリ配下のmarkdownファイルをすべて読み込む。
* 見出し（#、##、###など）の下に空行を追加する。
* `>`だけの行は後ろのスペースを除去する。
