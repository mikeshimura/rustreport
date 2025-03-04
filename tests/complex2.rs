#[cfg(test)]
mod complex2 {
    use num_format::Locale::ar_AE;
    use num_format::*; // for formatting floats
    use printpdf::*;
    use rustreport::exec::{Detail, PageHeader, Summary};
    use rustreport::PageOrientation::{Landscape, Portrait};
    use rustreport::PageSize::A4;
    use rustreport::*;
    use std::fs::File;
    use std::io::BufWriter;

    #[test]
    fn complex2() {
        let mut context = rustreport::Context::new();
        let txt = std::fs::read_to_string("C:\\a\\wkrust\\rustpdf\\tests\\examples\\invoice2.txt")
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
        context.set_page(A4, Portrait);
        context.set_font_dir_and_name(
            "C:\\a\\wkrust\\rustpdf\\assets\\fonts\\Noto_Sans_JP\\static\\bold",
            "NotoSansJPBold",
        );
        context.set_font_dir_and_name(
            "C:\\a\\wkrust\\rustpdf\\assets\\fonts\\Noto_Sans_JP\\static",
            "NotoSansJP",
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
            .insert("g1item".to_string(), serde_json::Value::from(0.0 as f64));
        context
            .sum_work
            .insert("g2item".to_string(), serde_json::Value::from(0.0 as f64));
        context.detail.push(Box::new(DetailC2 {}));
        context.page_header.push(Box::new(PageHeaderC2 {}));
        context.summary.push(Box::new(SummaryC2G1 {}));
        context.summary.push(Box::new(SummaryC2G2 {}));
        context.footor.push(Box::new(FooterC2 {}));
        context.max_level = 2;
        context.exec();
        context.convert();
        context.save("temp/complex2.pdf");
        context.write_buffer("temp/complex2.txt");

        struct DetailC2 {}
        impl Detail for DetailC2 {
            fn GetHeight(&self, context: &mut Context) -> f32 {
                5.0
            }
            fn Execute(&self, context: &mut Context) {
                let mut slipshow = true;
                if context.sum_work.get("g1item").unwrap().as_f64().unwrap() > 0.0 {
                    if context.input[context.cur_line as usize][4] == context.input[context.cur_line as usize - 1][4] &&
                        //改ページの場合表示する
                        context.cur_vpos > 20.0
                    {
                        slipshow = false;
                    }
                }
                let x = 25.0;
                let y = context.cur_vpos;
                context.set_outline_thickness(0.3);
                context.write_rect(x , y, x + 160.0, y + 5.0, false);
                context.write_line_vertical(x, y, y + 5.0);
                context.write_line_vertical(x + 23.0, y, y + 5.0);
                context.write_line_vertical(x + 41.0, y, y + 5.0);
                context.write_line_vertical(x + 61.0, y, y + 5.0);
                context.write_line_vertical(x + 99.0, y, y + 5.0);
                context.write_line_vertical(x + 116.0, y, y + 5.0);
                context.write_line_vertical(x + 135.0, y, y + 5.0);
                //ページ最終行はラインを引く
                if context.footer_vpos - context.cur_vpos <= 5.0 {
                    context.write_line_horizontal(x, y + 5.0, x + 41.0);
                };
                context.set_font("NotoSansJP");
                context.set_font_size(9.0 as i32);
                if slipshow {
                    let s1 = context.input[context.cur_line as usize][3].clone();
                    context.write_text(x + 1.0, y + 4.0, &s1);
                    let s2 = context.input[context.cur_line as usize][4].clone();
                    context.write_text(x + 24.0, y + 4.0, &s2);
                }
                let s3 = context.input[context.cur_line as usize][5].clone();
                context.write_text(x + 42.0, y + 4.0, &s3);
                let s4 = context.input[context.cur_line as usize][6].clone();
                context.write_text(x + 62.0, y + 4.0, &s4);
                let s5 = context.input[context.cur_line as usize][7].clone();
                context.write_text_right(
                    x + 115.0,
                    y + 4.0,
                    exec::i64_to_string_comma(s5.as_str().parse::<i64>().unwrap()).as_str(),
                );

                context.write_text_right(
                    x + 134.0,
                    y + 4.0,
                    &(exec::get_yen_string()
                        + &exec::i64_to_string_comma(
                        context.input[context.cur_line as usize][8]
                            .as_str()
                            .parse::<i64>()
                            .unwrap(),
                    )),
                );
                context.write_text_right(
                    x + 159.0,
                    y + 4.0,
                    &exec::i64_to_string_comma(
                        context.input[context.cur_line as usize][9]
                            .as_str()
                            .parse::<i64>()
                            .unwrap(),
                    ),
                );
                let amt = context.input[context.cur_line as usize][9]
                    .as_str()
                    .parse::<f64>()
                    .unwrap();
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
                    "g1item".to_string(),
                    serde_json::Value::from(
                        context.sum_work.get("g1item").unwrap().as_f64().unwrap() + 1.0,
                    ),
                );
                context.sum_work.insert(
                    "g2item".to_string(),
                    serde_json::Value::from(
                        context.sum_work.get("g2item").unwrap().as_f64().unwrap() + 1.0,
                    ),
                );
                context.cur_vpos += 5.0;
            }


            fn BreakCheckBefore(&self, context: &mut Context) -> i32 {
                if context.cur_line == 0 {
                    return 2 as i32;
                }
                if context.input[context.cur_line as usize][0]
                    != context.input[context.cur_line as usize - 1][0]
                {
                    return 2 as i32;
                }
                if context.input[context.cur_line as usize][4]
                    != context.input[context.cur_line as usize - 1][4]
                {
                    return 1 as i32;
                }
                0 as i32
            }
            fn BreakCheckAfter(&self, context: &mut Context) -> i32 {
                if context.cur_line == context.input.len() as i32 - 1 {
                    return 2 as i32;
                }
                if context.input[context.cur_line as usize][0]
                    != context.input[context.cur_line as usize + 1][0]
                {
                    return 2 as i32;
                }
                if context.input[context.cur_line as usize][4]
                    != context.input[context.cur_line as usize + 1][4]
                {
                    return 1 as i32;
                }
                0 as i32
            }
        }

        struct PageHeaderC2 {}
        impl PageHeader for PageHeaderC2 {
            fn Execute(&self, context: &mut Context) {
                let x = 25.0;
                let mut y = 32.0;
                if context.sum_work.get("g2item").unwrap().as_f64().unwrap() == 0.0 {
                    context.set_font_size(10);
                    context.set_font("NotoSansJP");
                    //今日の日付を2006年01月02日の形式で取得
                    let now = chrono::Local::now();
                    context.write_text_right(
                        182.0,
                        13.0,
                        now.format("%Y年%m月%d日").to_string().as_str(),
                    );
                    context.write_text_right(
                        182.0,
                        18.0,
                        &("請求書番号:".to_owned()
                            + context.input[context.cur_line as usize][2].as_str()),
                    );
                    context.set_font("NotoSansJPBold");
                    context.set_font_size(16);
                    context.write_text(92.0, 30.0, "請求書");
                    context.set_font_size(11);
                    let x = 118.0;
                    context.write_text(x, 46.0, "サンプル商事株式会社");
                    context.write_text(x, 51.0, "山田太郎");
                    context.set_font("NotoSansJP");
                    context.set_font_size(9);
                    context.write_text(x, 58.0, "〒181-0001");
                    context.write_text(x, 62.0, "東京都三鷹市井の頭5-12-12");
                    context.write_text(x, 72.0, "TEL:0422-22-2222");
                    context.write_text(x, 76.0, "FAX:0422-22-2223");
                    context.write_text(x, 80.0, "info@mitakashoji.jp");
                    let x = 25.0;
                    context.set_font("NotoSansJPBold");
                    context.set_font_size(12);
                    let company = context
                        .input
                        .get(context.cur_line as usize)
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .to_owned()
                        + "御中";
                    //この幅を取得する
                    //let companywidth = context.getWidthOfString(company.clone());
                    context.write_text(x, 46.0, company.as_str());
                    context.set_outline_thickness(0.5);
                    context.set_outline_greyScale(50);
                    //context.write_line_horizontal(x, 50.0, x + companywidth);
                    context.write_line_horizontal_strlen(x, 50.0, company, 0.0, 0.0);
                    context.set_outline_greyScale(0);
                    context.set_font("NotoSansJP");
                    context.set_font_size(9);
                    context.write_text(x, 58.0, "下記のとおりご請求申し上げます。");
                    context.set_font("NotoSansJPBold");
                    context.set_font_size(12);
                    context.write_text(x, 70.0, "ご請求金額");
                    context.write_text_right(
                        x + 72.0,
                        70.0,
                        "!!!TOTAL!!!",
                    );
                    context.write_line_horizontal(x, 74.0, x + 74.0);
                    context.set_outline_greyScale(0);
                    y = 85.0;
                }
                context.set_outline_greyScale(85);
                context.set_fill_greyScale(85);
                context.write_rect(x, y, x + 160.0, y + 5.0, true);
                context.set_outline_greyScale(0);
                context.set_outline_thickness(0.3);
                context.write_rect(x , y, x + 160.0, y + 5.0, false);
                context.write_line_vertical(x + 23.0, y, y + 5.0);
                context.write_line_vertical(x + 41.0, y, y + 5.0);
                context.write_line_vertical(x + 61.0, y, y + 5.0);
                context.write_line_vertical(x + 99.0, y, y + 5.0);
                context.write_line_vertical(x + 116.0, y, y + 5.0);
                context.write_line_vertical(x + 135.0, y, y + 5.0);
                context.set_font("NotoSansJP");
                context.set_font_size(10);
                let yadd = 4.0;
                context.write_text(x + 5.0, y + yadd, "年月日");
                context.write_text(x + 28.0, y + yadd, "伝票");
                context.write_text(x + 47.0, y + yadd, "品番");
                context.write_text(x + 76.0, y + yadd, "品名");
                context.write_text(x + 104.0, y + yadd, "数量");
                context.write_text(x + 122.0, y + yadd, "単価");
                context.write_text(x + 144.0, y + yadd, "金額");
                context
                    .sum_work
                    .insert("g1item".to_string(), serde_json::Value::from(0.0 as f64));
                context
                    .sum_work
                    .insert("g2item".to_string(), serde_json::Value::from(1.0 as f64));
                context.cur_vpos = y + 5.0;
            }
        }

        struct FooterC2 {}
        impl exec::Footer for FooterC2 {
            fn Execute(&self, context: &mut Context) {
                context.write_text(100.0, context.footer_vpos + 12.0, "Page");
                context.write_text(
                    112.0,
                    context.footer_vpos + 12.0,
                    context.page.to_string().as_str(),
                )
            }
        }
        struct SummaryC2G1 {}
        impl Summary for SummaryC2G1 {
            fn GetHeight(&self, context: &mut Context) -> f32 {
                5.0
            }
            fn Execute(&self, context: &mut Context) {
                let x = 25.0;
                let y = context.cur_vpos ;
                context.set_outline_greyScale(85);
                context.set_fill_greyScale(85);
                context.write_rect(x, y, x + 160.0, y + 5.0, true);
                context.set_outline_thickness(0.3);
                context.set_outline_greyScale(0);
                context.write_rect(x, y, x + 160.0, y + 5.0, false);
                context.set_font("NotoSansJP");
                context.set_font_size(10);
                context.write_text_right(
                    x + 159.0,
                    y + 4.0,
                    &(exec::get_yen_string()
                        + exec::i64_to_string_comma(
                        context.sum_work.get("g1amtcum").unwrap().as_f64().unwrap() as i64,
                    )
                        .as_str()),
                );
                context.sum_work.insert("g1amtcum".to_string(), serde_json::Value::from(0.0 as f64));
                context.sum_work.insert("g1item".to_string(), serde_json::Value::from(0.0 as f64));
                context.write_text(x + 117.0, y + 4.0, "伝票合計");
                context.cur_vpos += 5.0;
            }
        }
        struct SummaryC2G2 {}
        impl Summary for SummaryC2G2 {
            fn GetHeight(&self, context: &mut Context) -> f32 {
                20.0
            }
            fn Execute(&self, context: &mut Context) {
                let x = 25.0;
                let y = context.cur_vpos ;
                context.set_outline_greyScale(85);
                context.set_fill_greyScale(85);
                context.write_rect(x + 116.0, y, x + 160.0, y + 15.0, true);
                context.set_outline_thickness(0.3);
                context.set_outline_greyScale(0);
                context.write_rect(x + 116.0, y, x + 160.0, y + 5.0, false);
                context.write_rect(x + 116.0, y + 5.0, x + 160.0, y + 10.0, false);
                context.write_rect(x + 116.0, y + 10.0, x + 160.0, y + 15.0, false);
                context.set_font("NotoSansJP");
                context.set_font_size(10);
                // context.write_text_right(
                //     x + 159.0,
                //     y + .0,
                //     &(exec::get_yen_string()
                //         + &exec::i64_to_string_comma(
                //         context.sum_work.get("g2amtcum").unwrap().as_f64().unwrap() as i64,
                //     )),
                // );
                context.write_text(x + 117.0, y+4.0, "合計");
                let amt = context.sum_work.get("g2amtcum").unwrap().as_f64().unwrap() as i64;
                context.write_text_right(
                    x + 159.0,
                    y +4.0,
                    &(exec::get_yen_string() + exec::i64_to_string_comma(amt).as_str()),
                );
                let cons = amt * 10 / 100;
                context.write_text(x + 117.0, y + 9.0, "消費税");
                context.write_text_right(
                    x + 159.0,
                    y + 9.0,
                    &(exec::get_yen_string() + exec::i64_to_string_comma(cons).as_str()),
                );
                context.write_text(x + 117.0, y + 14.0, "請求金額");
                let total=exec::get_yen_string() + exec::i64_to_string_comma(amt + cons).as_str();
                context.write_text_right(
                    x + 159.0,
                    y + 14.0,
                    total.as_str().clone(),
                );
                context.sum_work.insert("!!!TOTAL!!!".to_string(),serde_json::Value::from(total));
                // context
                //     .sum_work
                //     .insert("g2item".to_string(), serde_json::Value::from(0.0 as f64));
                // context
                //     .sum_work
                //     .insert("g2amtcum".to_string(), serde_json::Value::from(0.0 as f64));
                if context.cur_line< context.input.len() as i32 - 1 {
                    context.page_break(true)
                }
            }
        }
    }
}
