# rustreport: Rust での動的な PDF レポート生成ライブラリ

`rustreport` は、複雑な PDF レポートの作成を簡素化するために設計された Rust ライブラリです。タブ区切りのテキストデータから構造化されたレポートを生成することに優れており、動的なレイアウト、グルーピング、集計、ページ形式を可能にします。`rustreport` は 2 段階のプロセスを採用しており、柔軟なレポート定義と、自動ページ番号や合計などの強力な機能を提供します。
## サンプル
![Sample complex1.pdf](
)
## 主な機能

-   **タブ区切りデータ入力:** `rustreport` は、列がタブで区切られたテキストファイルからデータを処理できます。この形式は多くのソースから簡単に生成でき、柔軟なデータパイプラインを提供します。
-   **レポート定義:**
    -   **ページ構造:** ページサイズ (例: A4, Letter) と向き (縦、横) を定義します。
    -   **ヘッダー:** 各ページの上部に表示されるページヘッダー (`PageHeader`) と、レポートをセクションに分割するグループヘッダー (`GroupHeader`) を作成します。
    -   **詳細:** レポートの本体を 1 行ずつレンダリングする `Detail` セクションを定義します。
    -   **集計:** グループまたはレポートの最後に合計やその他の集計情報を計算して表示する `Summary` セクションを含めます。
    -   **フッター:** 各ページの下部に表示される `Footer` 要素を追加します。
    -   **レポートの集計**: レポートの最後に合計またはその他の集計情報を計算して表示する `ReportSummary` セクションを含めます。
-   **動的なレイアウト:** ライブラリは、以下のためのメソッドを提供します。
    -   **テキストの配置:** 特定の座標にテキストを書き込む (`write_text`)、または指定された領域内でテキストを右揃えする (`write_text_right`)。
    -   **線の描画:** スタイルの制御付きで、水平線、垂直線、および斜線を描画します。
    -   **長方形の描画:** 塗りつぶしまたは枠線のある長方形を作成します。
    -   **画像の埋め込み:** 画像をレポートに挿入します。
    - **改ページ**: 改ページを挿入します。
-   **フォント管理:**
    -   TTF ファイルからフォントをロードします。
    -   アクティブなフォントとフォントサイズを設定します。
    -   テキストの色とスタイルを制御します。
-   **グラフィックのカスタマイズ:**
    -   枠線の色と太さを設定します。
    -   塗りつぶしの色を設定します。
    -   線に破線パターンを使用します。
-   **2 段階の PDF 作成:**
    1.  **テキストバッファの生成:** `Context` オブジェクトは、レイアウトとコマンドを表す中間テキストバッファを作成します。
    2.  **PDF への変換:** 次に、`convert()` メソッドがこのバッファを処理し、`printpdf` を使用して最終的な PDF を生成します。
-   **自動ページ番号と合計:** `flags` システムにより、自動ページ番号の配置のための "__pagetotal__" などの制御フラグが可能になり、`Summary` セクションで合計を計算するための `sum_work` HashMap が提供されます。
- **柔軟な出力**: 最終的な PDF または中間テキストファイルを出力します。

## コアコンポーネント

### `Context`

`Context` 構造体は `rustreport` の中心です。レポートの状態を管理します。以下を含みます。

-   **ページ寸法:** `page_height`, `page_width`
-   **PDF 出力:** `doc` (`printpdf` ドキュメント), `pages` (`PdfPage` オブジェクトのコレクション).
-   **フォント情報:** `font_files`, `font_name`, `font_cache`, `font_id`, `font_size`.
-   **中間テキストバッファ:** `buffer` (コマンドが格納される `String`).
- **描画操作**: `opttext` , `optgraphic`.
-   **レポート構造:** `detail`, `page_header`, `group_header`, `footor`, `summary`, `report_summary` (トレイトオブジェクトのベクター).
-   **データ入力:** `input` (タブ区切りのデータ).
-   **レンダリング状態:** `cur_line`, `cur_vpos`, `footer_vpos` (位置の追跡).
-   **アキュムレータ:** `sum_work` (実行中の合計を格納するため), `vers`, `flags`.
- **現在のページ情報** `page` `page_report` `page_total`
- **最大グループレベル** `max_level`

**主なメソッド:**

-   `new()`: 新しい空のレポートコンテキストを作成します。
-   `set_page(ps: PageSize, pt: PageOrientation)`: ページサイズ (`PageSize::A4` または `PageSize::Letter`) と向き (`PageOrientation::Portrait` または `PageOrientation::Landscape`) を設定します。
- `set_page_by_mm(h:f64,w:f64)`:  カスタム mm でページを設定します。
-   `set_font_dir_and_name(fontDir: &str, fontName: &str)`: ロードするフォントのディレクトリと名前を指定します。
-   `set_font(fontName: &str)`: 現在のフォントを設定します。
-   `set_font_size(font_size: i32)`: フォントサイズを設定します。
-   `set_text_color(r: u8, g: u8, b: u8)`: 現在のテキストの色 (RGB) を設定します。
-   `set_text_greyScale(g: u8)`: 現在のテキストの色 (グレイスケール) を設定します。
-   `set_outline_color(r: u8, g: u8, b: u8)`: 現在の枠線の色 (RGB) を設定します。
-    `set_outline_greyScale(g: u8)`: 現在の枠線(グレイスケール)を設定します。
-   `set_outline_thickness(t: f32)`: 枠線の太さを設定します。
-    `set_fill_color(r: u8, g: u8, b: u8)`: 塗りつぶしの色 (RGB) を設定します。
-   `set_fill_greyScale(g: u8)`: 塗りつぶしの色 (グレイスケール)を設定します。
-   `write_text(x: f64, y: f64, text: &str)`: 指定された座標にテキストを書き込みます。
-   `write_text_right(x: f64, y: f64, text: &str)`: 指定された座標に右揃えでテキストを書き込みます。
-   `set_dash_pattern(dash1: i32, gap1: i32, dash2: i32, gap2: i32)`: 破線パターンを設定します。
- `reset_dash_pattern()`: 破線パターンをリセットします。
-   `write_line_horizontal(x1: f64, y1: f64, x2: f64)`: 水平線を描画します。
-  `write_line_horizontal_strlen(x1: f64, y1: f64, s: String, ofsettl: f64, ofsettr: f64)`: テキスト付きの水平線を描画します。
-  `write_line_horizontal_right_strlen(x1: f64, y1: f64, s: String, ofsettl: f64, ofsettr: f64)`: テキスト付きの右揃えの水平線を描画します。
-   `write_line_vertical(x1: f64, y1: f64, y2: f64)`: 垂直線を描画します。
-   `write_line(x1: f64, y1: f64, x2: f64, y2: f64)`: 2 点間に線を描画します。
-   `write_rect(x1: f64, y1: f64, x2: f64, y2: f64, fill: bool)`: 塗りつぶしまたは輪郭のある長方形を描画します。
-   `write_image(x: f64, y: f64, w: f64, h: f64, img: &str)`: 指定された位置と寸法に画像を埋め込みます。
-  `new_page()`: 改ページを挿入します。
- `write_buffer(filename: &str)`: 中間のテキストバッファをファイルに保存します。
- `read_buffer(filename: &str)`: ファイルから中間テキストバッファを読み込みます。
-   `convert()`: テキストバッファを `printpdf` の命令に変換します。
-   `save(filename: &str)`: PDF ファイルを保存します。
- `get_pdf_bytes()`: メモリ内で PDF を生成します。
- `getWidthOfString(text: String)` : 文字列の幅を取得します。
-   `exec()`: レポート定義 (ヘッダー、詳細、サマリーなど) を順番に実行します。

### トレイト (`exec.rs`)

`rustreport` は、レポートのさまざまな部分を表す複数のトレイトを定義しています。

-   **`Detail`:**
    -   `GetHeight(&self, context: &mut Context) -> f32`: 詳細行の高さを返します。
    -   `Execute(&self, context: &mut Context)`: 詳細行をレンダリングします。
    - `BreakCheckBefore(&self,context:&mut Context)->i32`: Detailの前に、ページまたはグループの改行を確認します。
    - `BreakCheckAfter(&self,context:&mut Context)->i32`: Detailの後に、ページまたはグループの改行を確認します。

-   **`PageHeader`:**
    -   `Execute(&self, context: &mut Context)`: ページヘッダーをレンダリングします。

-   **`GroupHeader`:**
    -   `GetHeight(&self, context: &mut Context) -> f32`: グループヘッダーの高さを返します。
    -   `Execute(&self, context: &mut Context)`: グループヘッダーをレンダリングします。

-   **`Summary`:**
    -   `GetHeight(&self, context: &mut Context) -> f32`: 集計行の高さを返します。
    -   `Execute(&self, context: &mut Context)`: 集計行をレンダリングします。
- **`Footer`**:
    - `Execute(&self,context:&mut Context)`: フッターを実行します。
- **`ReportSummary`**:
  -`GetHeight(&self, context: &mut Context) -> f32`: レポート集計行の高さを返します。
    - `Execute(&self,context:&mut Context)`: レポート集計を実行します。

### ユーティリティ関数 (`exec.rs`)

- `f64_roundedd2(f:f64)->f64`: f64 を小数点以下 2 桁に丸めます。
- `f64_to_string_commad2(f:f64)->String`: f64 を小数点以下 2 桁とコンマ区切りの文字列に書式設定します。
- `f64_to_string_commad1(f:f64)->String`: f64 を小数点以下 1 桁とコンマ区切りの文字列に書式設定します。
- `i64_to_string_comma(i:i64)->String`: i64 をコンマ区切りの文字列に書式設定します。
- `get_yen_string()->String`: 円記号を返します。

## ワークフロー

1.  **データ準備:** レポートデータを含むタブ区切りテキストファイルを作成します。
2.  **コンテキストの作成:** レポートを管理するための `Context` オブジェクトを作成します。
3.  **レポートの設定:**
    -   ページサイズと向きを設定します (`set_page`)。
    -   フォントをロードします (`set_font_dir_and_name`)。
    -   フォントとフォントサイズを設定します (`set_font`, `set_font_size`)。
    -   必要に応じて、色、線パターンなどを設定します。
    -   `cur_vpos` と `footer_vpos` を設定します。
    -   `max_level` を設定します。
    -   フラグを設定します。
4. **入力データ**: `context.input` にタブ区切りのテキストファイルを読み込みます。
5.  **レポート構造の定義:**
    -   レポートの各セクションのレンダリング方法を定義するために、`Detail`, `PageHeader`, `GroupHeader`, `Summary`,`Footer`, `ReportSummary` トレイトを実装する構造体を作成します。
    -   それらを `context` の対応するベクターに追加します (例: `context.detail.push(...)`)。
6. **集計の準備**: `sum_work`を初期化します。
7.  **レポートの実行:** `context.exec()` を呼び出します。これにより、入力データを反復処理し、必要に応じて詳細、ヘッダー、サマリー、フッターオブジェクトのメソッドを呼び出します。
8. **PDF 命令への変換**: `context.convert()` を呼び出して、バッファを PDF 命令に変換します。
9.  **PDF の生成:** `context.save()` を呼び出して、生成された PDF をファイルに保存します。
10. **代替出力**: 中間ファイルを保存するために、`context.write_buffer()` を呼び出します。

## 使用例


