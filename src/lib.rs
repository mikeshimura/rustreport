pub mod exec;

use genpdfrev;
use genpdfrev::fonts::{Font, FontCache, FontFamily};
use genpdfrev::render::Page;
use image::{DynamicImage, GenericImageView};
use printpdf::ops::*;
use printpdf::{
    BlendMode, Cmyk, ExtendedGraphicsStateBuilder, FontId, Greyscale, Line, LineCapStyle,
    LineDashPattern, LineJoinStyle, Mm, PaintMode, ParsedFont, PdfDocument, Point, Polygon, Pt,
    RawImage, WindingOrder, XObjectTransform,
};
use printpdf::{Color, Rgb};
use std::collections::HashMap;
use std::{fmt, fs};
use serde_json::Value;

pub struct Context {
    pub page_height: f32,
    pub page_width: f32,
    pub pages: Vec<PdfPage>,
    pub font_files: HashMap<String, FontId>,
    pub font_name: String,
    pub font_cache: Option<FontCache>,
    pub font_id: FontId,
    pub font_size: i32,
    pub buffer: Vec<String>,
    pub doc: PdfDocument,
    pub opttext: Vec<Op>,
    pub optgraphic: Vec<Op>,
    pub genpdffonts: HashMap<String, FontFamily<Font>>,
    pub input: Vec<Vec<String>>,
    pub cur_line:i32,
    pub cur_vpos: f64,
    pub footer_vpos: f64,
    pub sum_work:HashMap<String,Value>,
    pub vers:HashMap<String,String>,
    pub page:i32,
    pub page_report:i32,
    pub page_total:i32 ,
    pub flags:HashMap<String,bool>,
    pub detail:Vec<Box<dyn exec::Detail>>,
    pub page_header:Vec<Box<dyn exec::PageHeader>>,
    pub footor:Vec<Box<dyn exec::Footer>>,
    pub summary:Vec<Box<dyn exec::Summary>>,
    pub report_summary:Vec<Box<dyn exec::ReportSummary>>,
}
impl Clone for Context {
    fn clone(&self) -> Self {
        Context {
            page_height: self.page_height,
            page_width: self.page_width,
            pages: self.pages.clone(),
            font_files: self.font_files.clone(),
            font_name: self.font_name.clone(),
            font_cache: self.font_cache.clone(),
            font_id: self.font_id.clone(),
            font_size: self.font_size,
            buffer: self.buffer.clone(),
            doc: self.doc.clone(),
            opttext: self.opttext.clone(),
            optgraphic: self.optgraphic.clone(),
            genpdffonts: self.genpdffonts.clone(),
            input: self.input.clone(),
            cur_line: self.cur_line,
            cur_vpos: self.cur_vpos,
            footer_vpos: self.footer_vpos,
            sum_work: self.sum_work.clone(),
            vers: self.vers.clone(),
            page: self.page,
            page_report: 0,
            page_total: self.page_total,
            flags: self.flags.clone(),
            detail: Vec::new(), // Do not clone the detail field
            page_header: Vec::new(),
            footor: Vec::new(),
            summary: Vec::new(),
            report_summary: Vec::new(),

        }
    }
}
#[derive(PartialEq)]
pub enum PageOrientation {
    Portrait,
    Landscape,
}

impl fmt::Display for PageOrientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PageOrientation::Portrait => write!(f, "Portrait"),
            PageOrientation::Landscape => write!(f, "Landscape"),
        }
    }
}

#[derive(PartialEq)]
pub enum PageSize {
    A4,
    Letter,
}

impl fmt::Display for PageSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PageSize::A4 => write!(f, "A4"),
            PageSize::Letter => write!(f, "Letter"),
        }
    }
}
impl Context {
    pub fn new() -> Context {
        Context {
            page_height: 0.0,
            page_width: 0.0,
            pages: Vec::new(),
            font_files: HashMap::new(),
            font_name: String::from(""),
            font_cache: None,
            font_id: FontId::new(),
            opttext: Vec::new(),
            optgraphic: Vec::new(),
            genpdffonts: HashMap::new(),
            font_size: 0,
            cur_line:0,
            cur_vpos: 0.0,
            footer_vpos:0.0,
            buffer: Vec::new(),
            doc: PdfDocument::new("PDF"),
            input: Vec::new(),
            sum_work:  HashMap::new(),
            vers: HashMap::new(),
            page:0,
            page_report: 0,
            page_total:0,
            flags: HashMap::new(),
            detail: vec![],
            page_header: vec![],
            footor: vec![],
            summary: vec![],
            report_summary: vec![],
        }
    }
}
impl Context {
    pub fn set_page(&mut self, ps: PageSize, pt: PageOrientation) {
        self.buffer
            .push(format!("P\t{}\t{}\n", ps.to_string(), pt.to_string(),));
    }

    pub fn set_page_by_mm(&mut self, h: f64, w: f64) {
        self.page_height = h as f32;
        self.page_width = w as f32;
        self.buffer
            .push(format!("PM\t{}\t{}\n", self.page_width, self.page_height));
    }
}
impl Context {
    pub fn get_buffer(&self) -> String {
        self.buffer.join("")
    }
    pub fn set_font_dir_and_name(&mut self, fontName: &str, fontDir: &str) {
        self.font_name = String::from(fontName);
        self.buffer.push(format!("FF\t{}\t{}\n", fontDir, fontName));
    }
    pub fn set_font(&mut self, fontName: &str) {
        self.font_name = String::from(fontName);
        self.buffer.push(format!("F\t{}\n", self.font_name));
    }
    pub fn set_font_size(&mut self, font_size: i32) {
        self.font_size = i32::from(font_size);
        self.buffer.push(format!("FS\t{}\n", self.font_size));
    }
    pub fn set_text_color(&mut self, r: u8, g: u8, b: u8) {
        self.buffer.push(format!("TC\t{}\t{}\t{}\n", r, g, b));
    }
    pub fn set_text_greyScale(&mut self, g: u8) {
        self.buffer.push(format!("TG\t{}\n", g));
    }
    pub fn set_outline_color(&mut self, r: u8, g: u8, b: u8) {
        self.buffer.push(format!("OC\t{}\t{}\t{}\n", r, g, b));
    }
    pub fn set_outline_thickness(&mut self, t: u8) {
        self.buffer.push(format!("OS\t{}\n", t));
    }
    pub fn set_outline_greyScale(&mut self, g: u8) {
        self.buffer.push(format!("OG\t{}\n", g));
    }
    pub fn set_fill_color(&mut self, r: u8, g: u8, b: u8) {
        self.buffer.push(format!("FC\t{}\t{}\t{}\n", r, g, b));
    }
    pub fn set_fill_greyScale(&mut self, g: u8) {
        self.buffer.push(format!("FG\t{}\n", g));
    }
    pub fn write_text(&mut self, x: f64, y: f64, text: &str) {
        self.buffer.push(format!("TL\t{}\t{}\t{}\n", x, y, text));
    }
    pub fn write_text_right(&mut self, x: f64, y: f64, text: &str) {
        self.buffer.push(format!("TR\t{}\t{}\t{}\n", x, y, text));
    }
    pub fn set_dash_pattern(&mut self, dash1: i32, gap1: i32, dash2: i32, gap2: i32) {
        self.buffer
            .push(format!("DP\t{}\t{}\t{}\t{}\n", dash1, gap1, dash2, gap2));
    }
    pub fn reset_dash_pattern(&mut self) {
        self.buffer.push("RDP\n".to_string());
    }
    pub fn write_line_horizontal(&mut self, x1: f64, y1: f64, x2: f64) {
        self.buffer.push(format!("LH\t{}\t{}\t{}\n", x1, y1, x2));
    }
    pub fn write_line_vertical(&mut self, x1: f64, y1: f64, y2: f64) {
        self.buffer.push(format!("LV\t{}\t{}\t{}\n", x1, y1, y2));
    }
    pub fn write_line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        self.buffer
            .push(format!("L\t{}\t{}\t{}\t{}\n", x1, y1, x2, y2));
    }
    pub fn write_rect(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, fill: &str) {
        self.buffer
            .push(format!("R\t{}\t{}\t{}\t{}\t{}\n", x1, y1, x2, y2, fill));
    }
    pub fn write_image(&mut self, x: f64, y: f64, w: f64, h: f64, img: &str) {
        self.buffer
            .push(format!("I\t{}\t{}\t{}\t{}\t{}\n", x, y, w, h, img));
    }
    pub fn new_page(&mut self) {
        self.buffer.push("NP\n".to_string());
    }
    pub fn write_buffer(&mut self, filename: &str) {
        //self.bufferの内容をファイルに書き出す
        let mut txt = self.get_buffer();
        //これをfilenameに書き出す
        std::fs::write(filename, txt).expect("Unable to write file");
    }
    pub fn read_buffer(&mut self, filename: &str) {
        //filenameの内容をself.bufferに読み込む
        let txt = std::fs::read_to_string(filename).expect("Unable to read file");
        self.buffer = txt.split("\n").map(|x| x.to_string()).collect();
    }
}
impl Context {
    pub fn convert(&mut self) {
        let buffer_lines: Vec<String> = self.buffer.clone();
        for line in buffer_lines.iter() {
            let v: Vec<&str> = line.split("\t").collect();
            match v[0].trim() {
                "P" => {
                    let ps = match v[1] {
                        "A4" => PageSize::A4,
                        "Letter" => PageSize::Letter,
                        _ => PageSize::A4,
                    };
                    let pt = match v[2] {
                        "Portrait" => PageOrientation::Portrait,
                        "Landscape" => PageOrientation::Landscape,
                        _ => PageOrientation::Portrait,
                    };
                    let mut width = 0.0;
                    let mut height = 0.0;
                    if ps == PageSize::A4 && pt == PageOrientation::Portrait {
                        width = 210.0;
                        height = 297.0;
                    }
                    if ps == PageSize::A4 && pt == PageOrientation::Landscape {
                        width = 297.0;
                        height = 210.0;
                    }
                    if ps == PageSize::Letter && pt == PageOrientation::Portrait {
                        width = 216.0;
                        height = 279.0;
                    }
                    if ps == PageSize::Letter && pt == PageOrientation::Landscape {
                        width = 279.0;
                        height = 216.0;
                    }
                    self.create_doc(width, height)
                }
                "PM" => {
                    let h = v[1].parse::<f32>().unwrap();
                    let w = v[2].parse::<f32>().unwrap();
                    self.create_doc(w, h)
                }
                "FF" => {
                    let fontName = v[1];
                    let fontDir = v[2].trim();
                    let font_family = genpdfrev::fonts::from_files(fontDir, fontName, None)
                        .expect("Failed to load font family");
                    let mut doc2 = genpdfrev::Document::new(font_family.clone());
                    let mut fontcache = doc2.font_cache().clone();
                    let font = fontcache.default_font_family();
                    self.genpdffonts.insert(fontName.to_string(), font);
                    self.font_cache = Some(fontcache.clone());
                    use std::fs;

                    let font_path = format!("{}/{}-Regular.ttf", fontDir, fontName);
                    let TTF = fs::read(font_path).expect("Failed to read font file");
                    let font = ParsedFont::from_bytes(&*TTF, 0).unwrap();
                    let font_id = self.doc.add_font(&font).clone();
                    self.font_files
                        .insert(fontName.to_string(), font_id.clone());
                    self.font_id = font_id.clone();
                    self.font_name = fontName.to_string();
                }
                "F" => {
                    let fontName = v[1].trim();
                    self.font_name = fontName.to_string();
                    self.font_id = self.font_files.get(fontName).unwrap().clone();
                }
                "FS" => {
                    let font_size = v[1].trim().parse::<i32>().unwrap();
                    self.font_size = font_size;
                }
                "TC" => {
                    let r = v[1].parse::<f32>().unwrap();
                    let g = v[2].parse::<f32>().unwrap();
                    let b = v[3].trim().parse::<f32>().unwrap();
                    self.opttext.push(Op::SetFillColor {
                        col: Color::Rgb(Rgb::new(r, g, b, None)),
                    });
                    self.optgraphic.push(Op::SetFillColor {
                        col: Color::Rgb(Rgb::new(r, g, b, None)),
                    });
                }
                "TG" => {
                    let g = v[1].trim().parse::<u8>().unwrap();
                    let gs = g as f32 / 100.0;
                    self.opttext.push(Op::SetFillColor {
                        col: Color::Rgb(Rgb::new(gs, gs, gs, None)),
                    });
                    self.optgraphic.push(Op::SetFillColor {
                        col: Color::Rgb(Rgb::new(gs, gs, gs, None)),
                    });
                }
                "OC" => {
                    let r = v[1].parse::<f32>().unwrap();
                    let g = v[2].parse::<f32>().unwrap();
                    let b = v[3].trim().parse::<f32>().unwrap();
                    self.opttext.push(Op::SetOutlineColor {
                        col: Color::Rgb(Rgb::new(r, g, b, None)),
                    });
                    self.optgraphic.push(Op::SetOutlineColor {
                        col: Color::Rgb(Rgb::new(r, g, b, None)),
                    });
                }
                "OG" => {
                    let g = v[1].trim().parse::<u8>().unwrap();
                    let gs = g as f32 / 100.0;
                    self.opttext.push(Op::SetOutlineColor {
                        col: Color::Rgb(Rgb::new(gs, gs, gs, None)),
                    });
                    self.optgraphic.push(Op::SetOutlineColor {
                        col: Color::Rgb(Rgb::new(gs, gs, gs, None)),
                    });
                }
                "OS" => {
                    let g = v[1].trim().parse::<u8>().unwrap();
                    self.optgraphic
                        .push(Op::SetOutlineThickness { pt: Pt(g as f32) });
                    self.opttext
                        .push(Op::SetOutlineThickness { pt: Pt(g as f32) });
                }
                "FC" => {
                    let r = v[1].parse::<u8>().unwrap();
                    let g = v[2].parse::<u8>().unwrap();
                    let b = v[3].trim().parse::<u8>().unwrap();
                    self.optgraphic.push(Op::SetFillColor {
                        col: Color::Rgb(Rgb::new(r as f32, g as f32, b as f32, None)),
                    });
                }
                "FG" => {
                    let g = v[1].trim().parse::<u8>().unwrap();
                    let gs = g as f32 / 100.0;
                    self.optgraphic.push(Op::SetFillColor {
                        col: Color::Rgb(Rgb::new(gs, gs, gs, None)),
                    });
                }
                "TL" => {
                    let x = v[1].parse::<f32>().unwrap();
                    let y = self.page_height - v[2].parse::<f32>().unwrap();
                    let text = v[3].trim();
                    self.opttext.push(Op::StartTextSection);
                    self.opttext.push(Op::SetTextCursor {
                        pos: Point::new(Mm(x), Mm(y)),
                    });
                    self.opttext.push(Op::WriteText {
                        text: text.to_string(),
                        font: self.font_id.clone(),
                        size: Pt(self.font_size as f32),
                    });
                    self.opttext.push(Op::EndTextSection);
                }
                "TR" => {
                    let x = v[1].parse::<f32>().unwrap();
                    let y = self.page_height - v[2].parse::<f32>().unwrap();
                    let text = v[3].trim();
                    let width: genpdfrev::Mm = self
                        .genpdffonts
                        .get(&self.font_name)
                        .unwrap()
                        .regular
                        .str_width(
                            &self.font_cache.clone().unwrap(),
                            text,
                            self.font_size as u8,
                        )
                        .into();
                    let f64_width: f64 = width.into();
                    let modx: f32 = (x - f64_width as f32);
                    let modx_pt = Pt(modx as f32 * 72.0 / 25.4);
                    let y_pt = Pt(y as f32 * 72.0 / 25.4);
                    let xmm = Mm::from(modx_pt);
                    let ymm = Mm::from(y_pt);
                    self.opttext.push(Op::StartTextSection);
                    self.opttext.push(Op::SetTextCursor {
                        pos: Point::new(xmm, ymm),
                    });
                    self.opttext.push(Op::WriteText {
                        text: text.to_string(),
                        font: self.font_id.clone(),
                        size: Pt(self.font_size as f32),
                    });
                    self.opttext.push(Op::EndTextSection);
                }
                "DP" => {
                    let dash1 = v[1].parse::<i32>().unwrap();
                    let gap1 = v[2].parse::<i32>().unwrap();
                    let dash2 = v[3].parse::<i32>().unwrap();
                    let gap2 = v[4].trim().parse::<i32>().unwrap();
                    let dash_pattern = LineDashPattern {
                        dash_1: Some(dash1 as i64),
                        gap_1: Some(gap1 as i64),
                        dash_2: Some(dash2 as i64),
                        gap_2: Some(gap2 as i64),
                        ..Default::default()
                    };
                    self.optgraphic
                        .push(Op::SetLineDashPattern { dash: dash_pattern });
                }
                "RDP" => {
                    let dash_pattern = LineDashPattern {
                        dash_1: None,
                        gap_1: None,
                        dash_2: None,
                        gap_2: None,
                        ..Default::default()
                    };
                    self.optgraphic
                        .push(Op::SetLineDashPattern { dash: dash_pattern });
                }
                "LH" => {
                    let x1 = v[1].parse::<f32>().unwrap();
                    let y1 = self.page_height - v[2].parse::<f32>().unwrap();
                    let x2 = v[3].trim().parse::<f32>().unwrap();
                    let line1 = Line {
                        points: vec![
                            (Point::new(Mm(x1), Mm(y1)), false),
                            (Point::new(Mm(x2), Mm(y1)), false),
                        ],
                        is_closed: true,
                    };
                    self.optgraphic.push(Op::DrawLine { line: line1 });
                }
                "LV" => {
                    let x1 = v[1].parse::<f32>().unwrap();
                    let y1 = self.page_height - v[2].parse::<f32>().unwrap();
                    let y2 = self.page_height - v[3].trim().parse::<f32>().unwrap();
                    let line1 = Line {
                        points: vec![
                            (Point::new(Mm(x1), Mm(y1)), false),
                            (Point::new(Mm(x1), Mm(y2)), false),
                        ],
                        is_closed: true,
                    };
                    self.optgraphic.push(Op::DrawLine { line: line1 });
                }
                "L" => {
                    let x1 = v[1].parse::<f32>().unwrap();
                    let y1 = self.page_height - v[2].parse::<f32>().unwrap();
                    let x2 = v[3].parse::<f32>().unwrap();
                    let y2 = self.page_height - v[4].trim().parse::<f32>().unwrap();
                    let line1 = Line {
                        points: vec![
                            (Point::new(Mm(x1), Mm(y1)), false),
                            (Point::new(Mm(x2), Mm(y2)), false),
                        ],
                        is_closed: true,
                    };
                    self.optgraphic.push(Op::DrawLine { line: line1 });
                }
                "R" => {
                    let x1 = v[1].parse::<f32>().unwrap();
                    let y1 = self.page_height - v[2].parse::<f32>().unwrap();
                    let x2 = v[3].parse::<f32>().unwrap();
                    let y2 = self.page_height - v[4].parse::<f32>().unwrap();
                    let fill = v[5].trim();
                    if fill == "Y" {
                        //self.optgraphic.push(Op::SetFillColor {col: Color::Rgb(Rgb::new(0.95,0.95,0.95,None))});
                        self.optgraphic.push(Op::DrawPolygon {
                            polygon: Polygon {
                                rings: vec![vec![
                                    (Point::new(Mm(x1), Mm(y1)), false),
                                    (Point::new(Mm(x1), Mm(y2)), false),
                                    (Point::new(Mm(x2), Mm(y2)), false),
                                    (Point::new(Mm(x2), Mm(y1)), false),
                                ]],
                                mode: PaintMode::FillStroke,
                                winding_order: WindingOrder::NonZero,
                            },
                        });
                    } else {
                        self.opttext.push(Op::DrawPolygon {
                            polygon: Polygon {
                                rings: vec![vec![
                                    (Point::new(Mm(x1), Mm(y1)), false),
                                    (Point::new(Mm(x1), Mm(y2)), false),
                                    (Point::new(Mm(x2), Mm(y2)), false),
                                    (Point::new(Mm(x2), Mm(y1)), false),
                                ]],
                                mode: PaintMode::Stroke,
                                winding_order: WindingOrder::NonZero,
                            },
                        });
                    }
                }
                "I" => {
                    let x = v[1].parse::<f32>().unwrap();
                    let y = self.page_height - v[2].parse::<f32>().unwrap();
                    let w = v[3].parse::<f32>().unwrap();
                    let h = v[4].parse::<f32>().unwrap();
                    let xpt = Pt(x * 72.0 / 25.4);
                    let ypt = Pt(y * 72.0 / 25.4);
                    let wpt = Pt(w* 72.0 / 25.4);
                    let hpt = Pt(h* 72.0 / 25.4);
                    let wptx: PtTo1i32 = From::from(wpt);
                    let hptx: PtTo1i32 = From::from(hpt);
                    let wi32: f32 = wptx.into();
                    let hi32: f32 = hptx.into();
                    let img = v[5].trim();
                    let file_path = img;
                    let (width, height) = get_image_dimensions(file_path).unwrap();
                    let simg = format!("{}", img);
                    let image_bytes = fs::read(simg).expect("Failed to read image file");
                    let image = RawImage::decode_from_bytes(&image_bytes).unwrap();
                    let image_xobject_id = self.doc.add_image(&image);
                    let mut scalex=Some(wi32 / width as f32);
                    let mut scaleY=Some(hi32 / height as f32);
                    self.opttext.push(Op::UseXObject {
                        id: image_xobject_id.clone(),
                        transform: XObjectTransform {
                            translate_x: Some(xpt),
                            translate_y: Some(ypt),
                            scale_x: scalex.clone(),
                            scale_y: scaleY.clone(),
                            dpi:Some(72.0),
                            ..Default::default()
                        },
                    });
                }
                "NP" => self.new_page(),
                _ => {
                    println!("Unknown command: {}", v[0]);
                }
            }
        }
        self.new_page();
    }
}
struct PtTo1i32(Pt);
impl From<Pt> for PtTo1i32 {
    fn from(value: Pt) -> Self {
        PtTo1i32(value)
    }
}

impl Into<f32> for PtTo1i32 {
    fn into(self) -> f32 {
        self.0 .0
    }
}
impl Context {
    pub fn save(&mut self, filename: &str) {
        let pdf_bytes: Vec<u8> = self
            .doc
            .with_pages(self.pages.clone())
            .save(&Default::default());
        fs::write(filename, pdf_bytes).expect("Unable to write file");
    }
    pub fn get_pdf_bytes(&mut self) -> Vec<u8> {
        self.doc
            .with_pages(self.pages.clone())
            .save(&Default::default())
    }
}
fn get_image_dimensions(file_path: &str) -> image::ImageResult<(u32, u32)> {
    // let img = image::open(file_path).expect("Failed to open image file");
    // img.dimensions();
    image::image_dimensions(file_path)
}
impl Context {
    fn create_doc(&mut self, w: f32, h: f32) {
        let doc = PdfDocument::new("PDF");
        self.doc = doc.clone();
        self.page_height = h;
        self.page_width = w;
        self.cur_vpos = 0.0;
        //context.opttext.push(Op::StartTextSection);
    }

    fn new_page(&mut self) {
        //context.opttext.push(Op::EndTextSection);
        let mut ops: Vec<Op> = Vec::new();
        let grapiclayer = self.doc.add_layer(&Layer::new("Grapic content"));
        let textlayer = self.doc.add_layer(&Layer::new("Text content"));
        ops.push(Op::BeginLayer {
            layer_id: grapiclayer.clone(),
        });
        ops.extend(self.optgraphic.clone());
        ops.push(Op::EndLayer {
            layer_id: grapiclayer.clone(),
        });
        ops.push(Op::BeginLayer {
            layer_id: textlayer.clone(),
        });
        ops.extend(self.opttext.clone());
        ops.push(Op::EndLayer {
            layer_id: textlayer.clone(),
        });
        self.pages.push(PdfPage::new(
            Mm(self.page_width),
            Mm(self.page_height),
            ops,
        ));
        self.opttext = Vec::new();
        self.optgraphic = Vec::new();
        // self.opttext.push(Op::StartTextSection);
        self.cur_vpos = 0.0;
        self.page=self.page+1;
        self.page_total=self.page_total+1;
        self.page_report=self.page_report+1;
    }
}
impl Context{
    pub fn exec(&mut self){
       if self.footer_vpos==0.0{
           panic!("Footer position is not set");
       }
        self.page=1;
        self.page_total=1;
        self.page_report=1;
        if self.page_header.len()>0{
                self.page_header[0].Execute(self.clone());
        }
        self.buffer.push("v\tPAGE\t".to_string() + self.page.to_string().as_str() );
        for i in self.cur_line as usize..self.input.len(){
            self.cur_line= i as i32;
            self.exec_detail();
        }
    }
    pub fn exec_detail(&mut self){
        if self.flags.get("NewPageForce"){

        }

    }
}