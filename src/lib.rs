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
use serde_json::Value;
use std::collections::HashMap;
use std::{fmt, fs};

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
    pub cur_line: i32,
    pub cur_vpos: f64,
    pub footer_vpos: f64,
    pub sum_work: HashMap<String, Value>,
    pub vers: HashMap<String, String>,
    pub page: i32,
    pub page_report: i32,
    pub page_total: i32,
    pub flags: HashMap<String, bool>,
    pub detail: Vec<Box<dyn exec::Detail>>,
    pub page_header: Vec<Box<dyn exec::PageHeader>>,
    pub group_header: Vec<Box<dyn exec::GroupHeader>>,
    pub max_level: i32,
    pub footor: Vec<Box<dyn exec::Footer>>,
    pub summary: Vec<Box<dyn exec::Summary>>,
    pub report_summary: Vec<Box<dyn exec::ReportSummary>>,
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
            group_header: Vec::new(),
            max_level: 0,
            footor: Vec::new(),
            summary: Vec::new(),
            report_summary: Vec::new(),
        }
    }
}
#[derive(PartialEq, Clone)]
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

#[derive(PartialEq, Clone)]
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
            cur_line: 0,
            cur_vpos: 0.0,
            footer_vpos: 0.0,
            buffer: Vec::new(),
            doc: PdfDocument::new("PDF"),
            input: Vec::new(),
            sum_work: HashMap::new(),
            vers: HashMap::new(),
            page: 0,
            page_report: 0,
            page_total: 0,
            flags: HashMap::new(),
            detail: vec![],
            page_header: vec![],
            group_header: vec![],
            max_level: 0,
            footor: vec![],
            summary: vec![],
            report_summary: vec![],
        }
    }
}
impl Context {
    pub fn set_page(&mut self, ps: PageSize, pt: PageOrientation) {
        let (w, h) = match (ps.clone(), pt.clone()) {
            (PageSize::A4, PageOrientation::Portrait) => (210.0, 297.0),
            (PageSize::A4, PageOrientation::Landscape) => (297.0, 210.0),
            (PageSize::Letter, PageOrientation::Portrait) => (216.0, 279.0),
            (PageSize::Letter, PageOrientation::Landscape) => (279.0, 216.0),
        };
        self.page_height = h as f32;
        self.page_width = w as f32;
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
    pub fn set_font_dir_and_name(&mut self, fontDir: &str, fontName: &str) {
        self.font_name = String::from(fontName);
        self.buffer.push(format!("FF\t{}\t{}\n", fontName, fontDir));
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
    pub fn set_outline_thickness(&mut self, t: f32) {
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
    pub fn write_line_horizontal_strlen(
        &mut self,
        x1: f64,
        y1: f64,
        s: String,
        ofsettl: f64,
        ofsettr: f64,
    ) {
        self.buffer.push(format!(
            "LHS\t{}\t{}\t{}\t{}\t{}\n",
            x1, y1, s, ofsettl, ofsettr
        ));
    }
    pub fn write_line_horizontal_right_strlen(
        &mut self,
        x1: f64,
        y1: f64,
        s: String,
        ofsettl: f64,
        ofsettr: f64,
    ) {
        self.buffer.push(format!(
            "LHSR\t{}\t{}\t{}\t{}\t{}\n",
            x1, y1, s, ofsettl, ofsettr
        ));
    }
    pub fn write_line_vertical(&mut self, x1: f64, y1: f64, y2: f64) {
        self.buffer.push(format!("LV\t{}\t{}\t{}\n", x1, y1, y2));
    }
    pub fn write_line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        self.buffer
            .push(format!("L\t{}\t{}\t{}\t{}\n", x1, y1, x2, y2));
    }
    pub fn write_rect(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, fill: bool) {
        if fill {
            self.buffer
                .push(format!("R\t{}\t{}\t{}\t{}\t{}\n", x1, y1, x2, y2, "Y"));
        } else {
            self.buffer
                .push(format!("R\t{}\t{}\t{}\t{}\t{}\n", x1, y1, x2, y2, "N"));
        }
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
    pub fn getWidthOfString(&self, text: String) -> f64 {
        println!("{}", &self.font_name);
        println!("{:?}", self.genpdffonts);
        let width: genpdfrev::Mm = self
            .genpdffonts
            .get(&self.font_name)
            .unwrap()
            .regular
            .str_width(
                &self.font_cache.clone().unwrap(),
                text.as_str(),
                self.font_size as u8,
            )
            .into();
        let f64_width: f64 = width.into();
        return f64_width;
    }
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
                    let pt = match v[2].trim() {
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
                    let g = v[1].trim().parse::<f32>().unwrap();
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
                    // let width: genpdfrev::Mm = self
                    //     .genpdffonts
                    //     .get(&self.font_name)
                    //     .unwrap()
                    //     .regular
                    //     .str_width(
                    //         &self.font_cache.clone().unwrap(),
                    //         text,
                    //         self.font_size as u8,
                    //     )
                    //     .into();
                    // let f64_width: f64 = width.into();
                    let f64_width = self.getWidthOfString(text.to_string());
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
                "LHS" => {
                    let x1 = v[1].parse::<f32>().unwrap();
                    let y1 = self.page_height - v[2].parse::<f32>().unwrap();
                    let s: String = v[3].to_string();
                    let o1 = v[4].trim().parse::<f32>().unwrap();
                    let o2 = v[5].trim().parse::<f32>().unwrap();
                    let width = self.getWidthOfString(s.clone());
                    let line1 = Line {
                        points: vec![
                            (Point::new(Mm(x1 + o1), Mm(y1)), false),
                            (Point::new(Mm(x1 + width as f32 + o2), Mm(y1)), false),
                        ],
                        is_closed: true,
                    };
                    self.optgraphic.push(Op::DrawLine { line: line1 });
                }
                "LHSR" => {
                    let x1 = v[1].parse::<f32>().unwrap();
                    let y1 = self.page_height - v[2].parse::<f32>().unwrap();
                    let s: String = v[3].to_string();
                    let o1 = v[4].trim().parse::<f32>().unwrap();
                    let o2 = v[5].trim().parse::<f32>().unwrap();
                    let width = self.getWidthOfString(s.clone());
                    let line1 = Line {
                        points: vec![
                            (Point::new(Mm(x1 - width as f32 + o1), Mm(y1)), false),
                            (Point::new(Mm(x1 + o2), Mm(y1)), false),
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
                    let wpt = Pt(w * 72.0 / 25.4);
                    let hpt = Pt(h * 72.0 / 25.4);
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
                    let mut scalex = Some(wi32 / width as f32);
                    let mut scaleY = Some(hi32 / height as f32);
                    self.opttext.push(Op::UseXObject {
                        id: image_xobject_id.clone(),
                        transform: XObjectTransform {
                            translate_x: Some(xpt),
                            translate_y: Some(ypt),
                            scale_x: scalex.clone(),
                            scale_y: scaleY.clone(),
                            dpi: Some(72.0),
                            ..Default::default()
                        },
                    });
                }
                "NP" => self.new_page_draw(),
                _ => {
                    println!("Unknown command: {}", v[0]);
                }
            }
        }
        self.new_page_draw();
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

    fn new_page_draw(&mut self) {
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
        self.pages
            .push(PdfPage::new(Mm(self.page_width), Mm(self.page_height), ops));
        self.opttext = Vec::new();
        self.optgraphic = Vec::new();
        // self.opttext.push(Op::StartTextSection);
    }
}
impl Context {
    pub fn exec(&mut self) {
        if self.footer_vpos == 0.0 {
            panic!("Footer position is not set");
        }
        self.page = 1;
        self.page_total = 1;
        self.page_report = 1;
        self.buffer.push(
            "V\tPAGE\t".to_string()
                + self.page.to_string().as_str()
                + "\tTOTALPAGE\t"
                + self.page_total.to_string().as_str()
                + "\n",
        );
        if self.page_header.len() > 0 {
            // 1. Remove the PageHeader from the vector.
            let header = self.page_header.remove(0);
            // 2. Execute the header.
            header.Execute(self);
            //3. insert page_header
            self.page_header.insert(0, header);
        }

        for i in self.cur_line as usize..self.input.len() {
            self.cur_line = i as i32;
            self.exec_detail();
        }
        if self.report_summary.len() > 0 {
            let report_summary = self.report_summary.remove(0 as usize);
            let height = report_summary.GetHeight(self);
            self.page_break_check(height);
            report_summary.Execute(self);
            self.report_summary.insert(0 as usize, report_summary);
            self.cur_vpos = self.cur_vpos + height as f64;
        }
        if self.footor.len() > 0 {
            let footer = self.footor.remove(0);
            footer.Execute(self);
            self.footor.insert(0, footer);
        }
        self.execute_replace_pagetotal();
    }
    pub fn execute_replace_pagetotal(&mut self) {
        let mut total_page: String = String::new(); // Initialize as an empty String

        // First pass: find TOTALPAGE
        for i in (0..self.buffer.len()).rev() {
            let mut txt = self.buffer.remove(i);
            // Create a new Vector<String> instead of Vec<&str>
            let v: Vec<String> = txt.split("\t").map(|s| s.to_string()).collect();

            if v[0] == "V" {
                let page_temp = &v[2]; // Now a String, not a &str
                let total_page_temp = v[4].clone(); // Now a String, not a &str

                if total_page.is_empty() {
                    total_page = total_page_temp; // Clone into total_page
                }

                //txt = txt.replace("TOTALPAGE", &total_page);
                let parts: Vec<&str> = txt.split("\t").collect();
                let mut new_txt = String::new();
                for (i, part) in parts.iter().enumerate() {
                    if i == 4 {
                        new_txt.push_str(&total_page);
                    } else {
                        new_txt.push_str(part);
                    }
                    if i < parts.len() - 1 {
                        new_txt.push('\t');
                    }
                }
                txt = new_txt;

                if *page_temp == "1" {
                    total_page.clear(); // Clear the String
                }
            }
            self.buffer.insert(i, txt);
        }

        // Second pass: replace ＆＃TOTALPAGE&#
        let mut total_page_value = "".to_string();
        for i in 0..self.buffer.len() {
            let mut txt = self.buffer.remove(i); // Remove and take ownership
                                                 //parts now owns its String data, no more borrow from txt.
            let parts: Vec<String> = txt.split("\t").map(|s| s.to_string()).collect();
            let findtotal = r#"&#PAGETOTAL&#"#.to_string();
            if txt.contains("PAGETOTAL") {
                println!("found");
            }
            if parts.len() >= 3 && txt.contains(&findtotal) {
                // if let Some(total_page_value) = total_pages.get(&current_page) {
                txt = txt.replace(&findtotal, &total_page_value);
                //}
            }
            if parts[0] == "V" && parts.len() >= 4 && parts[2] == "1" {
                total_page_value = parts[4].clone();
            }
            if txt.contains("!!!") {
                for i in 0..parts.len() {
                    if parts[i].contains("!!!") {
                        let  mut found = parts[i].clone().trim().to_string();
                        print!("found {:?} {}", found, i);

                        // HashMap::get() を使用して値を取得し、Option<&Value> を返す
                        if let Some(work) = self.sum_work.get(&found) {
                            println!("work {:?}", work);

                            // work が数値の場合（total が数値だと仮定）
                            if let Some(num) = work.as_f64() {
                                let work_str = num.to_string();
                                let mut new_txt = String::new();
                                for (j, part) in parts.iter().enumerate() {
                                    if j == i {
                                        new_txt.push_str(&work_str);
                                    } else {
                                        new_txt.push_str(part);
                                    }
                                    if j < parts.len() - 1 {
                                        new_txt.push('\t');
                                    }
                                }
                                txt = new_txt;
                            }
                            // 文字列として扱いたい場合
                            else if let Some(work_str) = work.as_str() {
                                let mut new_txt = String::new();
                                for (j, part) in parts.iter().enumerate() {
                                    if j == i {
                                        new_txt.push_str(work_str);
                                    } else {
                                        new_txt.push_str(part);
                                    }
                                    if j < parts.len() - 1 {
                                        new_txt.push('\t');
                                    }
                                }
                                txt = new_txt;
                            }
                        }
                    }
                }
            }
            self.buffer.insert(i, txt);
        }
        // for i in 0..self.buffer.len() {
        //     let mut txt = self.buffer.remove(i); // Remove and take ownership
        //     let parts: Vec<String> = txt.split("\t").map(|s| s.to_string()).collect();
        //     if parts[0] == "V" && parts.len() >= 4 && parts[3] == "TOTALPAGE" {
        //         if let Some(total_page_value) = total_pages.get(&current_page) {
        //             let mut new_txt = String::new();
        //             for (i, part) in parts.iter().enumerate() {
        //                 if i == 4 {
        //                     new_txt.push_str(&total_page_value);
        //                 } else {
        //                     new_txt.push_str(part);
        //                 }
        //                 if i < parts.len() - 1 {
        //                     new_txt.push('\t');
        //                 }
        //             }
        //             txt = new_txt;
        //         }
        //     }
        //     self.buffer.insert(i, txt);
        //}
    }
    // ... (rest of your Context implementation) ...

    pub fn exec_detail(&mut self) {
        let mut detail = self.detail.remove(0);
        if self.flags.get("NewPageForce").is_some()
            && (*self.flags.get("NewPageForce").unwrap() == true)
        {
            self.page_break(*self.flags.get("ResetPageNo").unwrap());
            self.flags.insert("NewPageForce".to_string(), false);
        }
        if self.max_level > 0 {
            let bfr = detail.BreakCheckBefore(self);
            if bfr > 0 {
                self.execute_group_header(bfr);
            }
        }
        let height = detail.GetHeight(self);
        self.page_break_check(height);
        detail.Execute(self);
        if self.max_level > 0 {
            let afr = detail.BreakCheckAfter(self);
            if afr > 0 {
                self.execute_group_summary(afr);
            }
        }

        self.detail.insert(0, detail);
    }

    pub fn page_break(&mut self, reset_page_no: bool) {
        if self.footor.len() > 0 {
            let footer = self.footor.remove(0);
            footer.Execute(self);
            self.footor.insert(0, footer);
        }
        self.new_page();
        if reset_page_no {
            self.page = 1;
            self.page_total = 1;
        } else {
            self.page = self.page + 1;
            self.page_total = self.page;
        }
        self.page_report = self.page_report + 1;
        self.cur_vpos = 0.0;
        self.buffer.push(
            "V\tPAGE\t".to_string()
                + self.page.to_string().as_str()
                + "\tTOTALPAGE\t"
                + self.page_total.to_string().as_str()
                + "\n",
        );
        if (self.page_header.len() > 0) {
            let header = self.page_header.remove(0);
            header.Execute(self);
            self.page_header.insert(0, header);
        }
        if self.page == 3 {
            println!("page 3");
        }
    }

    pub fn execute_group_header(&mut self, level: i32) {
        let mut execlevl = level;
        if execlevl > self.group_header.len() as i32 {
            execlevl = self.group_header.len() as i32;
        }
        if self.group_header.len() > 0 {
            for i in 0..execlevl {
                let group_header = self.group_header.remove(i as usize);
                let height = group_header.GetHeight(self);
                self.page_break_check(height);
                group_header.Execute(self);
                self.group_header.insert(i as usize, group_header);
                self.cur_vpos = self.cur_vpos + height as f64;
            }
        }
    }
    pub fn execute_group_summary(&mut self, level: i32) {
        let mut execlevl = level;
        if execlevl > self.summary.len() as i32 {
            execlevl = self.summary.len() as i32;
        }

        for i in 0..execlevl {
            let summary = self.summary.remove(i as usize);
            let height = summary.GetHeight(self);
            self.page_break_check(height);
            summary.Execute(self);
            self.summary.insert(i as usize, summary);
            // self.cur_vpos = self.cur_vpos + height as f64;
        }
    }
    pub fn page_break_check(&mut self, height: f32) {
        if self.cur_vpos + height as f64 > self.footer_vpos {
            self.page_break(false);
        }
    }
}
