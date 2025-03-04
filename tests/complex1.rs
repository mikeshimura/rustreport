#[cfg(test)]
mod complex1 {
    use num_format::Locale::ar_AE;
    use num_format::*; // for formatting floats
    use printpdf::*;
    use rustreport::PageOrientation::{Landscape, Portrait};
    use rustreport::PageSize::A4;
    use rustreport::*;
    use std::fs::File;
    use std::io::BufWriter;

    #[test]
    fn complex1() {
        let mut context = rustreport::Context::new();
        let txt = std::fs::read_to_string("C:\\a\\wkrust\\rustpdf\\tests\\examples\\invoice.txt")
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
        context.set_page(A4, Portrait);
        context.set_font_dir_and_name(
            "C:\\a\\wkrust\\rustpdf\\assets\\fonts\\Roboto\\static",
            "Roboto_Condensed",
        );
        context.set_font_dir_and_name(
            "C:\\a\\wkrust\\rustpdf\\assets\\fonts\\Roboto\\static\\bold",
            "Roboto_CondensedBold",
        );
        context.set_font_size(12);
        context.cur_vpos = 0.0;
        context.footer_vpos = 265.0;
        context.cur_line = 0;
        context
            .sum_work
            .insert("g1amtcum".to_string(), serde_json::Value::from(0.0 as f64));
        context
            .sum_work
            .insert("g2amtcum".to_string(), serde_json::Value::from(0.0 as f64));
        context
            .sum_work
            .insert("g1hrcum".to_string(), serde_json::Value::from(0.0 as f64));
        context
            .sum_work
            .insert("g2hrcum".to_string(), serde_json::Value::from(0.0 as f64));
        context
            .sum_work
            .insert("g2item".to_string(), serde_json::Value::from(0.0 as f64));
        context.detail.push(Box::new(DetailC1 {}));
        context.page_header.push(Box::new(PageHeaderC1 {}));
        context.group_header.push(Box::new(GroupHeaderC1 {}));
        context.summary.push(Box::new(SummaryC1 {}));
        context.summary.push(Box::new(SummaryC2 {}));
        context.footor.push(Box::new(FooterC1 {}));
        context.max_level = 2;
        context.exec();
        context.convert();
        context.save("temp/complex1.pdf");
        context.write_buffer("temp/complex1.txt");
        struct DetailC1;
        impl exec::Detail for DetailC1 {
            fn GetHeight(&self, context: &mut Context) -> f32 {
                6.0
            }
            fn Execute(&self, context: &mut Context) {
                context.set_font("Roboto_Condensed");
                context.set_font_size(10.0 as i32);
                context.write_text(
                    14.0,
                    context.cur_vpos + 4.0,
                    &context.input[context.cur_line as usize][5].clone(),
                );
                context.write_text(
                    40.0,
                    context.cur_vpos + 4.0,
                    &context.input[context.cur_line as usize][6].clone(),
                );
                let hr = context.input[context.cur_line as usize][7]
                    .parse::<f64>()
                    .unwrap();
                context.write_text_right(
                    170.0,
                    context.cur_vpos + 4.0,
                    rustreport::exec::f64_to_string_commad1(hr).as_str(),
                );
                let amt = context.input[context.cur_line as usize][8]
                    .parse::<f64>()
                    .unwrap();
                context.write_text_right(
                    196.0,
                    context.cur_vpos + 4.0,
                    rustreport::exec::f64_to_string_commad2(amt).as_str(),
                );
                context.sum_work.insert(
                    "g1amtcum".to_string(),
                    serde_json::Value::from(
                        context.sum_work.get("g1amtcum").unwrap().as_f64().unwrap() + amt,
                    ),
                );
                context.sum_work.insert(
                    "g2amtcum".to_string(),
                    serde_json::Value::from(
                        context.sum_work.get("g2amtcum").unwrap().as_f64().unwrap() + amt,
                    ),
                );
                context.sum_work.insert(
                    "g1hrcum".to_string(),
                    serde_json::Value::from(
                        context.sum_work.get("g1hrcum").unwrap().as_f64().unwrap() + hr,
                    ),
                );
                context.sum_work.insert(
                    "g2hrcum".to_string(),
                    serde_json::Value::from(
                        context.sum_work.get("g2hrcum").unwrap().as_f64().unwrap() + hr,
                    ),
                );
                context.cur_vpos = context.cur_vpos + 6.0;
            }
            fn BreakCheckBefore(&self, context: &mut Context) -> i32 {
                if context.cur_line == 0 {
                    return 2;
                }
                if context.input[context.cur_line as usize][0]
                    != context.input[(context.cur_line - 1) as usize][0]
                {
                    return 2;
                }
                if context.input[context.cur_line as usize][4]
                    != context.input[(context.cur_line - 1) as usize][4]
                {
                    return 1;
                }
                0
            }
            fn BreakCheckAfter(&self, context: &mut Context) -> i32 {
                if context.cur_line == context.input.len() as i32 - 1 {
                    return 2;
                }
                if context.input[context.cur_line as usize][0]
                    != context.input[(context.cur_line + 1) as usize][0]
                {
                    return 2;
                }
                if context.input[context.cur_line as usize][4]
                    != context.input[(context.cur_line + 1) as usize][4]
                {
                    return 1;
                }
                0
            }
        }
        struct PageHeaderC1;
        impl exec::PageHeader for PageHeaderC1 {
            fn Execute(&self, context: &mut Context) {
                let mut y = 32.0;
                if context.sum_work.get("g2item").unwrap().as_f64().unwrap() == 0.0 {
                    context.write_image(
                        20.0,
                        50.0,
                        15.0,
                        15.0,
                        "C:\\a\\wkrust\\rustpdf\\assets\\apple.jpg",
                    );
                    context.set_font("Roboto_CondensedBold");
                    context.set_font_size(18.0 as i32);
                    context.set_fill_greyScale(90);
                    context.set_outline_greyScale(90);
                    context.write_rect(49.0, 70.0, 49.5, 85.0, true);
                    context.write_rect(150.0, 40.0, 150.5, 64.0, true);
                    context.write_rect(150.0, 69.0, 150.5, 93.0, true);
                    context.set_fill_greyScale(0);
                    context.write_text(145.0, 33.0, "TAX INVOICE");
                    context.set_font_size(9.0 as i32);
                    context.write_text(153.0, 45.0, "Test Consulting Corp.");
                    context.write_text(153.0, 51.0, "123 Hyde Street");
                    context.write_text(153.0, 57.0, "San Francisco, Calfornia");
                    context.write_text(153.0, 63.0, "USA");
                    context.write_text(139.0, 74.0, "To");
                    context.write_text(
                        153.0,
                        74.0,
                        context.input[context.cur_line as usize][0].clone().as_str(),
                    );
                    context.write_text(
                        153.0,
                        80.0,
                        context.input[context.cur_line as usize][1].clone().as_str(),
                    );
                    context.write_text(
                        153.0,
                        86.0,
                        context.input[context.cur_line as usize][2].clone().as_str(),
                    );
                    context.write_text(
                        153.0,
                        92.0,
                        context.input[context.cur_line as usize][3].clone().as_str(),
                    );
                    context.write_text(14.0, 73.0, "Tax Invoice No:");
                    context.write_text(14.0, 79.0, "Tax Invoice Date:");
                    context.write_text(14.0, 85.0, "Payment Due Date:");
                    context.write_text(
                        52.0,
                        73.0,
                        context.input[context.cur_line as usize][9].clone().as_str(),
                    );
                    context.write_text(
                        52.0,
                        79.0,
                        context.input[context.cur_line as usize][10]
                            .clone()
                            .as_str(),
                    );
                    context.write_text(
                        52.0,
                        85.0,
                        context.input[context.cur_line as usize][11]
                            .clone()
                            .as_str(),
                    );
                    y = 110.0;
                }
                context.set_outline_greyScale(90);
                context.set_fill_greyScale(90);
                context.write_rect(11.0, y - 4.0, 199.0, y + 3.5, true);
                context.set_outline_greyScale(0);
                context.write_text(14.0, y, "Type");
                context.write_text(40.0, y, "Description");
                context.write_text(161.0, y, "Hours");
                context.write_text(184.0, y, "Amount");

                if context.sum_work.get("g2item").unwrap().as_f64().unwrap() == 0.0 {
                    context.cur_vpos = 116.0;
                } else {
                    context.cur_vpos = 38.0;
                }
                context
                    .sum_work
                    .insert("g2item".to_string(), serde_json::Value::from(1.0 as f64));
            }
        }
        struct GroupHeaderC1;
        impl exec::GroupHeader for GroupHeaderC1 {
            fn GetHeight(&self, context: &mut Context) -> f32 {
                8.0
            }
            fn Execute(&self, context: &mut Context) {
                context.set_font("Roboto_CondensedBold");
                context.set_font_size(10.0 as i32);
                context.write_text(14.0, context.cur_vpos + 4.0, "SUB-TASK");
                context.write_text(
                    40.0,
                    context.cur_vpos + 4.0,
                    context.input[context.cur_line as usize][4].clone().as_str(),
                );

                context.set_fill_greyScale(90);
                context.set_outline_greyScale(90);
                context.write_rect(
                    11.0,
                    context.cur_vpos + 7.0,
                    199.0,
                    context.cur_vpos + 7.3,
                    true,
                );
                context.set_outline_greyScale(0);
                context.cur_vpos = context.cur_vpos + 8.0;
            }
        }
        struct SummaryC1;
        impl exec::Summary for SummaryC1 {
            fn GetHeight(&self, context: &mut Context) -> f32 {
                7.0
            }
            fn Execute(&self, context: &mut Context) {
                context.set_outline_thickness(0.2);
                context.set_outline_greyScale(90);
                context.write_line_horizontal(11.0, context.cur_vpos, 199.0);
                context.set_outline_greyScale(0);
                context.set_font("Roboto_CondensedBold");
                context.set_font_size(10.0 as i32);
                context.write_text_right(
                    170.0,
                    context.cur_vpos + 4.0,
                    &(rustreport::exec::f64_to_string_commad1(
                        context.sum_work.get("g1hrcum").unwrap().as_f64().unwrap(),
                    )
                    .as_str()
                    .to_owned()
                        + " Hrs"),
                );
                context.write_text_right(
                    196.0,
                    context.cur_vpos + 4.0,
                    &(rustreport::exec::f64_to_string_commad2(
                        context.sum_work.get("g1amtcum").unwrap().as_f64().unwrap(),
                    )
                    .as_str()
                    .to_owned()
                        + " USD"),
                );

                context
                    .sum_work
                    .insert("g1amtcum".to_string(), serde_json::Value::from(0.0 as f64));
                context
                    .sum_work
                    .insert("g1hrcum".to_string(), serde_json::Value::from(0.0 as f64));
                context.cur_vpos = context.cur_vpos + 7.0;
            }
        }
        struct SummaryC2;
        impl exec::Summary for SummaryC2 {
            fn GetHeight(&self, context: &mut Context) -> f32 {
                50.0
            }
            fn Execute(&self, context: &mut Context) {
                context.set_font("Roboto_CondensedBold");
                context.set_font_size(10.0 as i32);
                context.write_text_right(143.0, context.cur_vpos + 22.0, "Total:");
                context.write_text_right(
                    170.0,
                    context.cur_vpos + 22.0,
                    &(rustreport::exec::f64_to_string_commad1(
                        context.sum_work.get("g2hrcum").unwrap().as_f64().unwrap(),
                    )
                    .as_str()
                    .to_owned()
                        + " Hrs"),
                );
                context.write_text_right(
                    196.0,
                    context.cur_vpos + 22.0,
                    &(rustreport::exec::f64_to_string_commad2(
                        context.sum_work.get("g2amtcum").unwrap().as_f64().unwrap(),
                    )
                    .as_str()
                    .to_owned()
                        + " USD"),
                );

                context.set_outline_thickness(0.2);
                context.set_outline_greyScale(90);
                context.write_line_horizontal(11.0, context.cur_vpos, 199.0);
                context.set_outline_greyScale(0);

                context.set_outline_thickness(0.2);
                context.set_outline_greyScale(90);
                context.write_line_horizontal(11.0, context.cur_vpos + 7.0, 199.0);
                context.set_outline_greyScale(0);
                context.set_font("Roboto_CondensedBold");
                context.set_font_size(10.0 as i32);
                context.write_text_right(
                    170.0,
                    context.cur_vpos + 11.0,
                    &(rustreport::exec::f64_to_string_commad1(
                        context.sum_work.get("g1hrcum").unwrap().as_f64().unwrap(),
                    )
                    .as_str()
                    .to_owned()
                        + " Hrs"),
                );
                context.write_text_right(
                    196.0,
                    context.cur_vpos + 11.0,
                    &(rustreport::exec::f64_to_string_commad2(
                        context.sum_work.get("g1amtcum").unwrap().as_f64().unwrap(),
                    )
                    .as_str()
                    .to_owned()
                        + " USD"),
                );
                context.write_text_right(143.0, context.cur_vpos + 28.0, "Tax:");
                context.write_text_right(170.0, context.cur_vpos + 28.0, "7.75%");
                let tax = context.sum_work.get("g2amtcum").unwrap().as_f64().unwrap() * 0.0775;
                context.write_text_right(
                    196.0,
                    context.cur_vpos + 28.0,
                    &(rustreport::exec::f64_to_string_commad2(tax)
                        .as_str()
                        .to_owned()
                        + " USD"),
                );
                context.set_outline_thickness(0.3);
                context.write_line_horizontal(170.0, context.cur_vpos + 33.0, 199.0);
                context.set_font("Roboto_CondensedBold");
                context.set_font_size(12.0 as i32);
                context.write_text_right(143.0, context.cur_vpos + 42.0, "AMOUT DUE:");
                let amt_due = context.sum_work.get("g2amtcum").unwrap().as_f64().unwrap() + tax;
                context.write_text_right(
                    196.0,
                    context.cur_vpos + 42.0,
                    &(rustreport::exec::f64_to_string_commad2(amt_due)
                        .as_str()
                        .to_owned()
                        + " USD"),
                );
                if context.cur_line < context.input.len() as i32 - 1 {
                    context.page_break(true);
                }
                context
                    .sum_work
                    .insert("g2item".to_string(), serde_json::Value::from(0.0 as f64));
                context
                    .sum_work
                    .insert("g2amtcum".to_string(), serde_json::Value::from(0.0 as f64));
                context
                    .sum_work
                    .insert("g2hrcum".to_string(), serde_json::Value::from(0.0 as f64));
            }
        }
        struct FooterC1;
        impl exec::Footer for FooterC1 {
            fn Execute(&self, context: &mut Context) {
                context.set_font("Roboto_Condensed");
                context.set_font_size(10.0 as i32);
                context.write_text(100.0, 280.0, "Page:");
                context.write_text_right(112.0, 280.0, context.page.to_string().as_str());
            }
        }
    }
}
