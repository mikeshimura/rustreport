#[cfg(test)]
mod medium1 {
    use num_format::Locale::ar_AE;
    use num_format::*; // for formatting floats
    use printpdf::*;
    use rustreport::PageOrientation::{Landscape, Portrait};
    use rustreport::PageSize::A4;
    use rustreport::*;
    use std::fs::File;
    use std::io::BufWriter;

    #[test]
    fn medium1() {
        let mut context = rustreport::Context::new();
        let txt = std::fs::read_to_string("tests\\sales1.txt")
            .expect("Unable to read file");

        // Split the input into lines and then split each line by tabs
        context.input = txt
            .lines()
            .map(|line| {
                line.trim()
                    .split("\t")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();
        context.flags.insert("__pagetotal__".to_string(), true);
        context.set_page(A4, Landscape);
        context.set_font_dir_and_name(
            "assets\\fonts\\Noto_Serif_JP\\static",
            "NotoSerifJP",
        );
        context.set_font("NotoSerifJP");
        context.set_font_size(12);
        context.cur_vpos = 0.0;
        context.footer_vpos = 190.0;
        context.cur_line = 0;
        context
            .sum_work
            .insert("amountcum".to_string(), serde_json::Value::from(0.0 as f64));
        context
            .sum_work
            .insert("g1item".to_string(), serde_json::Value::from(0.0 as f64));
        context
            .sum_work
            .insert("g1cum".to_string(), serde_json::Value::from(0.0 as f64));
        context
            .sum_work
            .insert("g2cum".to_string(), serde_json::Value::from(0.0 as f64));
        context.detail.push(Box::new(DetailM1 {}));
        context.page_header.push(Box::new(PageHeaderM1 {}));
        context.report_summary.push(Box::new(ReportSummaryM1 {}));
        context.footor.push(Box::new(FooterM1 {}));
        context.summary.push(Box::new(GroupSummaryM11 {}));
        context.summary.push(Box::new(GroupSummaryM12 {}));
        context.max_level = 2;
        context.exec();
        context.convert();
        context.save("temp/medium1.pdf");
        context.write_buffer("temp/medium1.txt");
    }
    struct DetailM1;
    impl exec::Detail for DetailM1 {
        fn GetHeight(&self, context: &mut Context) -> f32 {
            10.0
        }
        fn Execute(&self, context: &mut Context) {
            context.set_font_size(12.0 as i32);
            context.write_text(
                15.0,
                context.cur_vpos + 8.0,
                &context.input[context.cur_line as usize][0].clone(),
            );
            context.write_text(
                30.0,
                context.cur_vpos + 8.0,
                &context.input[context.cur_line as usize][1].clone(),
            );
            context.write_text(
                60.0,
                context.cur_vpos + 8.0,
                &context.input[context.cur_line as usize][2].clone(),
            );
            context.write_text(
                90.0,
                context.cur_vpos + 8.0,
                &context.input[context.cur_line as usize][3].clone(),
            );
            context.write_text(
                120.0,
                context.cur_vpos + 8.0,
                &context.input[context.cur_line as usize][4].clone(),
            );
            context.write_text(
                135.0,
                context.cur_vpos + 8.0,
                &context.input[context.cur_line as usize][5].clone(),
            );
            context.write_text(
                160.0,
                context.cur_vpos + 8.0,
                &context.input[context.cur_line as usize][6].clone(),
            );
          let amount = context.input[context.cur_line as usize][6].parse::<f64>().unwrap()*
                context.input[context.cur_line as usize][5].parse::<f64>().unwrap();
            context.write_text_right(
                210.0,
                context.cur_vpos + 8.0,
                rustreport::exec::f64_to_string_commad2(amount).as_str(),
            );
            context.sum_work.insert(
                "amountcum".to_string(),
                serde_json::Value::from(
                    context
                        .sum_work
                        .get("amountcum")
                        .unwrap()
                        .as_f64()
                        .unwrap()
                        + amount,
                ),
            );
            context.sum_work.insert(
                "g1cum".to_string(),
                serde_json::Value::from(
                    context
                        .sum_work
                        .get("g1cum")
                        .unwrap()
                        .as_f64()
                        .unwrap()
                        + amount,
                ),
            );
            context.sum_work.insert(
                "g1item".to_string(),
                serde_json::Value::from(
                    context
                        .sum_work
                        .get("g1item")
                        .unwrap()
                        .as_f64()
                        .unwrap()
                        + 1.0,
                ),
            );
            context.sum_work.insert(
                "g2cum".to_string(),
                serde_json::Value::from(
                    context
                        .sum_work
                        .get("g2cum")
                        .unwrap()
                        .as_f64()
                        .unwrap()
                        + amount,
                ),
            );
            context.cur_vpos += 10.0;
        }
        fn BreakCheckBefore(&self, context: &mut Context) -> i32 {
            if context.cur_line == 0 {
                return 2 as i32;
            }
            let curr = context.input[context.cur_line as usize].clone();
            let prev = context.input[(context.cur_line - 1) as usize].clone();
            if curr[0] != prev[0] {
                return 2 as i32;
            }
            if curr[2] != prev[2] {
                return 1 as i32;
            }
            0 as i32
        }
        fn BreakCheckAfter(&self, context: &mut Context) -> i32 {
            if context.cur_line == context.input.len() as i32 - 1 {
                return 2 as i32;
            }
            let curr = context.input[context.cur_line as usize].clone();
            let next = context.input[(context.cur_line + 1) as usize].clone();
            if curr[0] != next[0] {
                return 2 as i32;
            }
            if curr[2] != next[2] {
                return 1 as i32;
            }
            0 as i32
        }
    }
    struct PageHeaderM1;
    impl exec::PageHeader for PageHeaderM1 {
        fn Execute(&self, context: &mut Context) {
            context.set_font_size(14.0 as i32);
            context.set_outline_thickness(1.8);
            context.set_fill_greyScale(90);
            context.write_rect(48.0, 13.0, 81.0, 21.0, true);
            context.write_text(50.0, 19.0, "Sales Report");
            context.write_text(245.0, 20.0, "page");
            context.write_text_right(263.0, 20.0, context.page.to_string().as_str());
            context.write_text(264.0, 20.0, "of");
            context.write_text_right(278.0, 20.0, "&#PAGETOTAL&#");
            context.write_text(15.0, 28.0, "D No");
            context.write_text(30.0, 28.0, "Dept");
            context.write_text(60.0, 28.0, "Order");
            context.write_text(90.0, 28.0, "Stock");
            context.write_text(120.0, 28.0, "Name");
            context.write_text_right(160.0, 28.0, "Unit Price");
            context.write_text_right(180.0, 28.0, "Qty");
            context.write_text_right(210.0, 28.0, "Amount");
            context.write_image(
                220.0,
                40.0,
                20.0,
                20.0,
                "C:\\a\\github\\wkgithub\\goreport\\example\\apple.jpg",
            );
            context.cur_vpos = 30.0;
        }
    }
    struct GroupSummaryM11;
    impl exec::Summary for GroupSummaryM11 {
        fn GetHeight(&self, context: &mut Context) -> f32 {
            if context.sum_work.get("g1item").unwrap().as_f64().unwrap() > 1.0 {
                10.0
            } else {
                0.0
            }
        }
        fn Execute(&self, context: &mut Context) {
            if context.input[context.cur_line as usize][2]=="8100002"{
                println!("8100002");
            }
            if context.sum_work.get("g1item").unwrap().as_f64().unwrap() > 1.0 {
                context.set_font_size(12.0 as i32);
                context.write_text(80.0, context.cur_vpos + 8.0, "Item");
                context.write_text_right(
                    110.0,
                    context.cur_vpos + 8.0,
                    context
                        .sum_work
                        .get("g1item")
                        .unwrap()
                        .as_f64()
                        .unwrap()
                        .to_string()
                        .as_str(),
                );
                context.write_text(150.0, context.cur_vpos + 8.0, "Order Total");
                context.write_text_right(
                    210.0,
                    context.cur_vpos + 8.0,
                    rustreport::exec::f64_to_string_commad2(
                        context
                            .sum_work
                            .get("g1cum")
                            .unwrap()
                            .as_f64()
                            .unwrap(),
                    )
                        .as_str(),
                );
            }
            context.set_outline_thickness(1.0);


            if context.sum_work.get("g1item").unwrap().as_f64().unwrap() > 1.0 {
                context.write_line_horizontal(15.0, context.cur_vpos + 10.0, 220.0);
                context.cur_vpos += 10.0;
            } else{
                //itemがひとつのときはすでにcur_vposが10.0増加しているので、ここで増加しない
                context.write_line_horizontal(15.0, context.cur_vpos , 220.0);
            }
            context.sum_work.insert(
                "g1cum".to_string(),
                serde_json::Value::from(0.0 as f64),
            );
            context.sum_work.insert(
                "g1item".to_string(),
                serde_json::Value::from(0.0 as f64),
            );
        }
    }
    struct GroupSummaryM12;
    impl exec::Summary for GroupSummaryM12 {
        fn GetHeight(&self, context: &mut Context) -> f32 { 10.0 }
        fn Execute(&self, context: &mut Context) {
            context.set_font_size(12.0 as i32);
            context.write_text(150.0, context.cur_vpos + 8.0, "Dept Total");
            context.write_text_right(
                210.0,
                context.cur_vpos + 8.0,
                rustreport::exec::f64_to_string_commad2(
                    context
                        .sum_work
                        .get("g2cum")
                        .unwrap()
                        .as_f64()
                        .unwrap(),
                )
                    .as_str(),
            );
            context.sum_work.insert("g2cum".to_string(), serde_json::Value::from(0.0 as f64));
            context.page_break(false);
            //Page breakの時は vpos がリセットされるので、ここで設定不要
         }
    }
    struct ReportSummaryM1;
    impl exec::ReportSummary for ReportSummaryM1 {
        fn GetHeight(&self, context: &mut Context) -> f32 { 10.0 }
        fn Execute(&self, context: &mut Context) {
            context.set_font_size(12.0 as i32);
            context.write_text(160.0, context.cur_vpos + 8.0, "Total");
            context.write_text_right(
                210.0,
                context.cur_vpos + 8.0,
                rustreport::exec::f64_to_string_commad2(
                    context
                        .sum_work
                        .get("amountcum")
                        .unwrap()
                        .as_f64()
                        .unwrap(),
                )
                    .as_str(),
            );
            context.cur_vpos += 10.0;
        }
    }
    struct FooterM1;
    impl exec::Footer for FooterM1 {
        fn Execute(&self, context: &mut Context) {
            context.set_font_size(12.0 as i32);
            context.write_text(160.0, 200.0, "Footer Sample");
        }
    }
}

