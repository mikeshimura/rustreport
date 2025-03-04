# rustreport: A Dynamic PDF Report Generation Library in Rust

`rustreport` is a Rust library designed to simplify the creation of complex PDF reports. It excels at generating structured reports from tab-separated text data, allowing for dynamic layouts, grouping, summaries, and page formatting. `rustreport` uses a two-step process, enabling flexible report definition and powerful features like automatic page numbering and totals.
## 日本語の説明
![readmejp.md](https://github.com/mikeshimura/rustreport/blob/master/readmejp.md
)
## Sample
![Sample complex1.jpeg](https://raw.githubusercontent.com/mikeshimura/rustreport/refs/heads/master/jpeg/complex1_page-0001.jpg
)
Additional Sample located bottom.
## Key Features

-   **Tab-Separated Data Input:** `rustreport` can process data from text files where columns are separated by tabs. This format is easily generated from many sources, providing a flexible data pipeline.
-   **Report Definition:**
    -   **Page Structure:** Define the page size (e.g., A4, Letter) and orientation (Portrait, Landscape).
    -   **Headers:** Create page headers (`PageHeader`) that appear at the top of each page and group headers (`GroupHeader`) that divide the report into sections.
    -   **Details:** Define the `Detail` section, which renders the main body of the report, row by row.
    -   **Summaries:** Include `Summary` sections to calculate and display totals or other aggregated information at the end of groups or the report.
    -   **Footers:** Add `Footer` elements that appear at the bottom of each page.
    -   **Report Summaries**: Include `ReportSummary` sections to calculate and display totals or other aggregated information at the end of the report.
-   **Dynamic Layout:** The library provides methods for:
    -   **Text Positioning:** Write text at specific coordinates (`write_text`), or right-align text within a given area (`write_text_right`).
    -   **Line Drawing:** Draw horizontal, vertical, and diagonal lines with control over styling.
    -   **Rectangle Drawing:** Create filled or stroked rectangles.
    -   **Image Embedding:** Insert images into the report.
    - **Page Break:** insert a page break
-   **Font Management:**
    -   Load fonts from TTF files.
    -   Set the active font and font size.
    -   Control text color and style.
-   **Graphics Customization:**
    -   Set outline colors and thicknesses.
    -   Set fill colors.
    -   Use dash patterns for lines.
-   **Two-Step PDF Creation:**
    1.  **Text Buffer Generation:** The `Context` object creates an intermediate text buffer representing the layout and commands.
       2.  **PDF Conversion:** The `convert()` method then processes this buffer and uses `printpdf` to generate the final PDF. I use function of `printpdf` version 0.8. Until 0.8 is available from crates.io, please use form github repoositiory. I also use revised `genpdf` which is included in source file to measure the width of a string.
-   **Automatic Page Numbering and Totals:** The `flags` system allows for control flags such as "__pagetotal__" for automatic page number placement and provides a `sum_work` HashMap for the `Summary` section to calculate totals.
-  **Flexible output**: Output the final PDF or an intermediate text file.

## Core Components

### `Context`

The `Context` struct is the heart of `rustreport`. It manages the state of the report, including:

-   **Page Dimensions:** `page_height`, `page_width`
-   **PDF Output:** `doc` (the `printpdf` document), `pages` (the collection of `PdfPage` objects).
-   **Font Information:** `font_files`, `font_name`, `font_cache`, `font_id`, `font_size`.
-   **Intermediate Text Buffer:** `buffer` (the `String` where commands are stored).
- **Draw Operation**: `opttext` , `optgraphic`.
-   **Report Structure:** `detail`, `page_header`, `group_header`, `footor`, `summary`, `report_summary` (vectors of trait objects).
-   **Data Input:** `input` (the tab-separated data).
-   **Rendering State:** `cur_line`, `cur_vpos`, `footer_vpos` (position tracking).
-   **Accumulators:** `sum_work` (for storing running totals), `vers`, `flags`.
- **Current page information** `page` `page_report` `page_total`
- **Maximum group level** `max_level`

**Key Methods:**

-   `new()`: Creates a new, empty report context.
-   `set_page(ps: PageSize, pt: PageOrientation)`: Sets the page size (`PageSize::A4` or `PageSize::Letter`) and orientation (`PageOrientation::Portrait` or `PageOrientation::Landscape`).
- `set_page_by_mm(h:f64,w:f64)`:  Sets the page by custom mm
-   `set_font_dir_and_name(fontDir: &str, fontName: &str)`: Specifies the directory and name of a font to be loaded.
-   `set_font(fontName: &str)`: Sets the current font.
- Font is assumed -Regular suffix. If you want to use bold, you need to load the font in other directory with -Regular suffix.
-   `set_font_size(font_size: i32)`: Sets the font size.
-   `set_text_color(r: u8, g: u8, b: u8)`: Sets the current text color (RGB).
-   `set_text_greyScale(g: u8)`: Set the current text color (greyScale).
-   `set_outline_color(r: u8, g: u8, b: u8)`: Sets the current outline color (RGB).
-    `set_outline_greyScale(g: u8)`: Sets the current outline (greyScale).
-   `set_outline_thickness(t: f32)`: Sets the outline thickness.
-    `set_fill_color(r: u8, g: u8, b: u8)`: Sets the fill color (RGB).
-   `set_fill_greyScale(g: u8)`: Sets the fill color (greyScale).
-   `write_text(x: f64, y: f64, text: &str)`: Writes text at the specified coordinates.
-   `write_text_right(x: f64, y: f64, text: &str)`: Writes text right-aligned at the specified coordinates.
-   `set_dash_pattern(dash1: i32, gap1: i32, dash2: i32, gap2: i32)`: Set the dash pattern
- `reset_dash_pattern()`: Reset the dash pattern
-   `write_line_horizontal(x1: f64, y1: f64, x2: f64)`: Draws a horizontal line.
-  `write_line_horizontal_strlen(x1: f64, y1: f64, s: String, ofsettl: f64, ofsettr: f64)`: Draw a horizontal line with text.
-  `write_line_horizontal_right_strlen(x1: f64, y1: f64, s: String, ofsettl: f64, ofsettr: f64)`: Draw a horizontal line right align with text.
-   `write_line_vertical(x1: f64, y1: f64, y2: f64)`: Draws a vertical line.
-   `write_line(x1: f64, y1: f64, x2: f64, y2: f64)`: Draws a line between two points.
-   `write_rect(x1: f64, y1: f64, x2: f64, y2: f64, fill: bool)`: Draws a rectangle, optionally filled.
-   `write_image(x: f64, y: f64, w: f64, h: f64, img: &str)`: Embeds an image at the specified position and dimensions.
-  `new_page()`: Insert a page break.
- `write_buffer(filename: &str)`: Save the intermediate text buffer to a file.
- `read_buffer(filename: &str)`: Read an intermediate text buffer from a file.
-   `convert()`: Converts the text buffer into `printpdf` instructions.
-   `save(filename: &str)`: Saves the PDF file.
-   `get_pdf_bytes()`: Generate the PDF in memory
-  `getWidthOfString(text: String)` : get the width of a string.
-   `exec()`: Executes the report definition (headers, details, summaries, etc.) in order.
### Traits (`exec.rs`)

`rustreport` defines several traits that represent the different parts of a report:

-   **`Detail`:**
    -   `GetHeight(&self, context: &mut Context) -> f32`: Returns the height of a detail row.
    -   `Execute(&self, context: &mut Context)`: Renders a detail row.
    - `BreakCheckBefore(&self,context:&mut Context)->i32`: Check for page or group break before detail.
    - `BreakCheckAfter(&self,context:&mut Context)->i32`: Check for page or group break after detail.

-   **`PageHeader`:**
    -   `Execute(&self, context: &mut Context)`: Renders the page header.

-   **`GroupHeader`:**
    -   `GetHeight(&self, context: &mut Context) -> f32`: Returns the height of a group header.
    -   `Execute(&self, context: &mut Context)`: Renders the group header.

-   **`Summary`:**
    -   `GetHeight(&self, context: &mut Context) -> f32`: Returns the height of a summary row.
    -   `Execute(&self, context: &mut Context)`: Renders a summary row.
- **`Footer`**:
    - `Execute(&self,context:&mut Context)`: Execute the footer
- **`ReportSummary`**:
  -`GetHeight(&self, context: &mut Context) -> f32`: Returns the height of a report summary row.
    - `Execute(&self,context:&mut Context)`: Execute the report summary.

### Utility functions (`exec.rs`)

- `f64_roundedd2(f:f64)->f64`: Round f64 to 2 decimals.
- `f64_to_string_commad2(f:f64)->String`: format f64 to string with 2 decimals and comma.
- `f64_to_string_commad1(f:f64)->String`: format f64 to string with 1 decimals and comma.
- `i64_to_string_comma(i:i64)->String`: format i64 to string with comma.
- `get_yen_string()->String`: return the yen sign.

## Workflow

1.  **Data Preparation:** Create a tab-separated text file with your report data.
2.  **Context Creation:** Create a `Context` object to manage the report.
3.  **Report Setup:**
    -   Set the page size and orientation (`set_page`).
    -   Load your fonts (`set_font_dir_and_name`).
    -   Set the font and font size (`set_font`, `set_font_size`).
    -   Optionally, set colors, line patterns, etc.
    -   Set `cur_vpos` and `footer_vpos`
    -   Set `max_level`
    - Set your flags
4. **Input data**: Load your tab-separated text file in `context.input`.
5.  **Define Report Structure:**
    -   Create structs that implement the `Detail`, `PageHeader`, `GroupHeader`, `Summary`, `Footer`, `ReportSummary` traits to define how each section of the report is rendered.
    -   Add them to the `context`'s corresponding vectors (e.g., `context.detail.push(...)`).
6. **Prepare Summary**: Initialize the `sum_work`
7.  **Execute the Report:** Call `context.exec()`. This iterates through the input data and calls the methods of the detail, header, summary, and footer objects as appropriate.
8. **Convert**: call `context.convert()` to convert the buffer to PDF instruction.
9.  **Generate PDF:** Call `context.save()` to save the generated PDF to a file.
10. **Alternative output** : Call `context.write_buffer()` to save an intermediate file.

## Example Usage

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
## Dependencies

-   `genpdfrev`
-   `image`
-   `printpdf`
- `num_format`
- `serde_json`

## Installation

Add the following to your `Cargo.toml` file:

```toml

[dependencies] 
rustreport = "0.1" 
genpdfrev="" 
image="" 
printpdf="" 
num_format="" 
serde_json="*"

```
## Additional Sample
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
## Sample of generated text file complex1.txt
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
## License

MIT

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.



