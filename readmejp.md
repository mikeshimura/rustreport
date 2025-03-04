# rustreport: Rust での動的な PDF レポート生成ライブラリ

`rustreport` は、複雑な PDF レポートの作成を簡素化するために設計された Rust ライブラリです。タブ区切りのテキストデータから構造化されたレポートを生成することに優れており、動的なレイアウト、グルーピング、集計、ページ形式を可能にします。`rustreport` は 2 段階のプロセスを採用しており、柔軟なレポート定義と、自動ページ番号や合計などの強力な機能を提供します。
## サンプル
![Sample complex1.jpeg](https://raw.githubusercontent.com/mikeshimura/rustreport/refs/heads/master/jpeg/complex1_page-0001.jpg
)
後ろに追加のサンプルがあります。

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
```rust
use rustreport::*; 
use printpdf::BorderStyleValues;  
use rustreport::PageSize::A4;  
use rustreport::PageOrientation:: Portrait; 
// Sample Detail struct 
struct MyDetail; 
impl exec::Detail for MyDetail { 
// ... (implement GetHeight, Execute, BreakCheckBefore, BreakCheckAfter) ... }
// Sample PageHeader struct 
struct MyPageHeader; 
impl exec::PageHeader for MyPageHeader { 
// ... (implement Execute) ... }
// Sample GroupHeader struct 
struct MyGroupHeader;
impl exec::GroupHeader for MyGroupHeader { 
// ... (implement GetHeight, Execute) ... } 
// Sample Summary struct 
struct MySummary; 
impl exec::Summary for MySummary { 
// ... (implement GetHeight, Execute) ... } 
// Sample Footer struct 
struct MyFooter;
impl exec::Footer for MyFooter { 
// ... (implement Execute) ... } 
fn main() { // Create a new report context 
    let mut context = Context::new(); 
    // load the tab-separated data 
    let txt = std::fs::read_to_string( " mydata. txt" )  .expect("Unable to read file"); 
    context.input = txt .lines() .map(|line| { line.trim() .split("\t") 
        .map(|s| s.to_string()) .collect::<Vec<String>>() }) 
        .collect::<Vec<Vec<String>>>(); 
    // Set the page 
    context.set_page(A4,  Portrait); 
    // Load Fonts 
    context.set_font_dir_and_name( "assets\\fonts\\Roboto\\static" ,  "Roboto_Condensed");  
    context.set_font_dir_and_name( "assets\\fonts\\Roboto\\static\\bold" ,  "Roboto_CondensedBold" ) ;  
    // Set the font 
    context.set_font_size( 12) ;  
    // Set the position 
    context.footer_vpos = 265.0; 
    context.cur_vpos = 0.0; 
    context.cur_line = 0; 
    // Set the sum_work 
    context.sum_work.insert( " g1amtcum" . to_string( ) ,  serde_json::Value::from( 0.0 as f64)); 
    context.sum_work.insert( " g2amtcum" . to_string( ) ,  serde_json::Value::from( 0.0 as f64)); 
    context.sum_work.insert( " g1hrcum" . to_string( ) ,  serde_json::Value::from( 0.0 as f64)); 
    context.sum_work.insert( " g2hrcum" . to_string( ) ,  serde_json::Value::from( 0.0 as f64)); 
    context.sum_work.insert( " g2item" . to_string( ) ,  serde_json::Value::from( 0.0 as f64));
    //add the structs
    context.detail.push(Box::new(MyDetail {}));
    context.page_header.push(Box::new(MyPageHeader {}));
    context.group_header.push(Box::new(MyGroupHeader {}));
    context.summary.push(Box::new(MySummary {}));
    context.footor.push( Box:: new( MyFooter {})); 
        // Set maximum level 
    context.max_level = 2; 
        // Process the report definition 
    context.exec();
    // Convert to PDF 
    context.convert(); 
    // Save the PDF 
context.save("myreport. pdf" ) ; 
}

```
## サンプル追加
![Sample simple1.jpeg](https://raw.githubusercontent.com/mikeshimura/rustreport/refs/heads/master/jpeg/simple1_page-0001.jpg
)
![Sample simple1.jpeg](https://raw.githubusercontent.com/mikeshimura/rustreport/refs/heads/master/jpeg/simple1_page-0002.jpg
)
![Sample medium1.jpeg](https://raw.githubusercontent.com/mikeshimura/rustreport/refs/heads/master/jpeg/medium1_page-0001.jpg
)
![Sample medium1.jpeg](https://raw.githubusercontent.com/mikeshimura/rustreport/refs/heads/master/jpeg/medium1_page-0002.jpg
)
![Sample complex1.jpeg](https://raw.githubusercontent.com/mikeshimura/rustreport/refs/heads/master/jpeg/complex1_page-0001.jpg
)
![Sample complex1.jpeg](https://raw.githubusercontent.com/mikeshimura/rustreport/refs/heads/master/jpeg/complex1_page-0002.jpg
)
![Sample complex2.jpeg](https://raw.githubusercontent.com/mikeshimura/rustreport/refs/heads/master/jpeg/complex2_page-0001.jpg
)
![Sample complex2.jpeg](https://raw.githubusercontent.com/mikeshimura/rustreport/refs/heads/master/jpeg/complex2_page-0002.jpg
)
## 生成された中間テキストファイル例 complex1.txt
```text
P	A4	Portrait
FF	Roboto_Condensed	assets\fonts\Roboto\static
FF	Roboto_CondensedBold	assets\fonts\Roboto\static\bold
FS	12
V	PAGE	1	TOTALPAGE	2
I	20	50	15	15	C:\a\wkrust\rustpdf\assets\apple.jpg
F	Roboto_CondensedBold
FS	18
FG	90
OG	90
R	49	70	49.5	85	Y
R	150	40	150.5	64	Y
R	150	69	150.5	93	Y
FG	0
TL	145	33	TAX INVOICE
FS	9
TL	153	45	Test Consulting Corp.
TL	153	51	123 Hyde Street
TL	153	57	San Francisco, Calfornia
TL	153	63	USA
TL	139	74	To
TL	153	74	MS Softech Corp.
TL	153	80	3019 Oakwood Lane
TL	153	86	Torrance, California
TL	153	92	USA
TL	14	73	Tax Invoice No:
TL	14	79	Tax Invoice Date:
TL	14	85	Payment Due Date:
TL	52	73	TEST-INV-2015001
TL	52	79	2024/12/18
TL	52	85	2025/01/17
OG	90
FG	90
R	11	106	199	113.5	Y
OG	0
TL	14	110	Type
TL	40	110	Description
TL	161	110	Hours
TL	184	110	Amount
F	Roboto_CondensedBold
FS	10
TL	14	120	SUB-TASK
TL	40	120	LOGIN screen
FG	90
OG	90
R	11	123	199	123.3	Y
OG	0
F	Roboto_Condensed
FS	10
TL	14	136	2024/12/01
TL	40	136	HTML Programming
TR	170	136	1.5
TR	196	136	75.00
F	Roboto_Condensed
FS	10
TL	14	142	2024/12/01
TL	40	142	Login Action Programming
TR	170	142	3.0
TR	196	142	150.00
F	Roboto_Condensed
FS	10
TL	14	148	2024/12/01
TL	40	148	Logout Action Programming
TR	170	148	1.5
TR	196	148	75.00
F	Roboto_Condensed
FS	10
TL	14	154	2024/12/02
TL	40	154	Password Check Logic
TR	170	154	6.0
TR	196	154	300.00
F	Roboto_Condensed
FS	10
TL	14	160	2024/12/02
TL	40	160	Database Update Programming
TR	170	160	1.5
TR	196	160	75.00
OS	0.2
OG	90
LH	11	162	199
OG	0
F	Roboto_CondensedBold
FS	10
TR	170	166	13.5 Hrs
TR	196	166	675.00 USD
F	Roboto_CondensedBold
FS	10
TL	14	173	SUB-TASK
TL	40	173	LOGIN unit test
FG	90
OG	90
R	11	176	199	176.3	Y
OG	0
F	Roboto_Condensed
FS	10
TL	14	189	2024/12/04
TL	40	189	Login Unit test
TR	170	189	1.0
TR	196	189	50.00
F	Roboto_Condensed
FS	10
TL	14	195	2024/12/04
TL	40	195	Logout Unit test
TR	170	195	1.5
TR	196	195	75.00
F	Roboto_Condensed
FS	10
TL	14	201	2024/12/05
TL	40	201	Session Timeout Test
TR	170	201	2.0
TR	196	201	100.00
F	Roboto_Condensed
FS	10
TL	14	207	2024/12/05
TL	40	207	Database Error Unit Test
TR	170	207	3.0
TR	196	207	150.00
OS	0.2
OG	90
LH	11	209	199
OG	0
F	Roboto_CondensedBold
FS	10
TR	170	213	7.5 Hrs
TR	196	213	375.00 USD
F	Roboto_CondensedBold
FS	10
TL	14	220	SUB-TASK
TL	40	220	Server session management
FG	90
OG	90
R	11	223	199	223.3	Y
OG	0
F	Roboto_Condensed
FS	10
TL	14	236	2024/12/06
TL	40	236	Session Management Programming
TR	170	236	1.5
TR	196	236	75.00
F	Roboto_Condensed
FS	10
TL	14	242	2024/12/07
TL	40	242	Session Database Daily Maintenanace
TR	170	242	3.0
TR	196	242	150.00
F	Roboto_Condensed
FS	10
TL	14	248	2024/12/08
TL	40	248	Session Unit Test
TR	170	248	1.5
TR	196	248	75.00
OS	0.2
OG	90
LH	11	250	199
OG	0
F	Roboto_CondensedBold
FS	10
TR	170	254	6.0 Hrs
TR	196	254	300.00 USD
F	Roboto_CondensedBold
FS	10
TL	14	261	SUB-TASK
TL	40	261	ORDER ENTRY screen
FG	90
OG	90
R	11	264	199	264.3	Y
OG	0
F	Roboto_Condensed
FS	10
TL	100	280	Page:
TR	112	280	1
NP
V	PAGE	2	TOTALPAGE	2
OG	90
FG	90
R	11	28	199	35.5	Y
OG	0
TL	14	32	Type
TL	40	32	Description
TL	161	32	Hours
TL	184	32	Amount
F	Roboto_Condensed
FS	10
TL	14	42	2024/12/08
TL	40	42	HTML Programming
TR	170	42	6.0
TR	196	42	300.00
F	Roboto_Condensed
FS	10
TL	14	48	2024/12/08
TL	40	48	Stock Update Programming
TR	170	48	1.5
TR	196	48	75.00
F	Roboto_Condensed
FS	10
TL	14	54	2024/12/08
TL	40	54	Customer Order History Programming
TR	170	54	1.0
TR	196	54	50.00
F	Roboto_Condensed
FS	10
TL	14	60	2024/12/08
TL	40	60	Auto Reorder Programming
TR	170	60	1.5
TR	196	60	75.00
OS	0.2
OG	90
LH	11	62	199
OG	0
F	Roboto_CondensedBold
FS	10
TR	170	66	10.0 Hrs
TR	196	66	500.00 USD
F	Roboto_CondensedBold
FS	10
TL	14	73	SUB-TASK
TL	40	73	ORDER ENTRY unit test
FG	90
OG	90
R	11	76	199	76.3	Y
OG	0
F	Roboto_Condensed
FS	10
TL	14	89	2024/12/08
TL	40	89	Stock Update Test
TR	170	89	2.0
TR	196	89	100.00
F	Roboto_Condensed
FS	10
TL	14	95	2024/12/08
TL	40	95	Customer Order Test
TR	170	95	3.0
TR	196	95	150.00
F	Roboto_Condensed
FS	10
TL	14	101	2024/12/08
TL	40	101	Auto Reorder Test
TR	170	101	2.0
TR	196	101	100.00
OS	0.2
OG	90
LH	11	103	199
OG	0
F	Roboto_CondensedBold
FS	10
TR	170	107	7.0 Hrs
TR	196	107	350.00 USD
F	Roboto_CondensedBold
FS	10
TR	143	132	Total:
TR	170	132	44.0 Hrs
TR	196	132	2,200.00 USD
OS	0.2
OG	90
LH	11	110	199
OG	0
OS	0.2
OG	90
LH	11	117	199
OG	0
F	Roboto_CondensedBold
FS	10
TR	170	121	0.0 Hrs
TR	196	121	0.00 USD
TR	143	138	Tax:
TR	170	138	7.75%
TR	196	138	170.50 USD
OS	0.3
LH	170	143	199
F	Roboto_CondensedBold
FS	12
TR	143	152	AMOUT DUE:
TR	196	152	2,370.50 USD
F	Roboto_Condensed
FS	10
TL	100	280	Page:
TR	112	280	2
```
