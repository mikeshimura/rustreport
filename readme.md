# rustreport: A Dynamic PDF Report Generation Library in Rust

`rustreport` is a Rust library designed to simplify the creation of complex PDF reports. It excels at generating structured reports from tab-separated text data, allowing for dynamic layouts, grouping, summaries, and page formatting. `rustreport` uses a two-step process, enabling flexible report definition and powerful features like automatic page numbering and totals.
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
    2.  **PDF Conversion:** The `convert()` method then processes this buffer and uses `printpdf` to generate the final PDF.
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
## License

[Add your license information here]

## Contributing

[Add your contribution guidelines here]


