#[cfg(test)]
mod simple1 {
    use printpdf::*;
    use rustreport::PageOrientation::{Landscape, Portrait};
    use rustreport::PageSize::A4;
    use num_format::*; // for formatting floats
    use rustreport::*;
    use std::fs::File;
    use std::io::BufWriter;
    use genpdfrev::render::Renderer;

    #[test]
    fn simple1() {
        let mut context = rustreport::Context::new();
        let txt = std::fs::read_to_string("test\\sales1.txt")
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
        context.set_page(A4, Landscape);
        context.set_font_dir_and_name(
            "C:\\a\\wkrust\\rustpdf\\assets\\fonts\\Noto_Serif_JP\\static",
            "NotoSerifJP",
        );
        context.set_font_size(12);
        context.cur_vpos = 20.0;
        context.footer_vpos =190.0;
        context.cur_line = 0;
        context.sum_work.insert("amountcum=".to_string(), serde_json::Value::from(0.0 as f64));
        context.detail.push(Box::new(DetailS1 {}));
        context.page_header.push(Box::new(PageHeaderS1 {}));
        context.report_summary.push(Box::new(ReportSummaryS1 {}));
        context.exec();
        context.convert();
        context.save("temp/simple1.pdf");
        context.write_buffer("temp/simple1.txt");

    }


    pub struct DetailS1;

    impl rustreport::exec::Detail for DetailS1 {
        fn GetHeight(&self, context:&mut  rustreport::Context) -> f32 {
            10.0
        }
        fn Execute(&self, context:&mut  rustreport::Context) {
            context.set_font_size(12.0 as i32);
            context.write_text(
                15.0,
                context.cur_vpos+8.0,
                &context.input[context.cur_line as usize][0].clone(),
            );
            context.write_text(
                30.0,
                context.cur_vpos+8.0,
                &context.input[context.cur_line as usize][1].clone(),
            );
            context.write_text(
                60.0,
                context.cur_vpos+8.0,
                &context.input[context.cur_line as usize][2].clone(),
            );
            context.write_text(
                90.0,
                context.cur_vpos+8.0,
                &context.input[context.cur_line as usize][3].clone(),
            );
            context.write_text(
                120.0,
                context.cur_vpos+8.0,
                &context.input[context.cur_line as usize][4].clone(),
            );
            context.write_text_right(
                160.0,
                context.cur_vpos+8.0,
                &context.input[context.cur_line as usize][5].clone(),
            );
            context.write_text_right(
                180.0,
                context.cur_vpos+8.0,
                &context.input[context.cur_line as usize][6].clone(),
            );
            let amt=context.input[context.cur_line as usize][5].parse::<f32>().unwrap()*
                context.input[context.cur_line as usize][6].parse::<f32>().unwrap();
            let rounded_amt = (amt * 100.0).round() / 100.0;
            context.sum_work.insert("amountcum=".to_string(), serde_json::Value::from(
                context.sum_work.get("amountcum=").unwrap().as_f64().unwrap() + rounded_amt as f64,
            ));
            // let formatted_amt = format!("{:.2}", rounded_amt);
            context.write_text_right(
                210.0,
                context.cur_vpos+8.0,
                rustreport::exec::f64_to_string_commad2(amt as f64).as_str(),
            );
            context.cur_vpos=context.cur_vpos+10.0;
        }
    }
    pub struct PageHeaderS1;
    impl rustreport::exec::PageHeader for PageHeaderS1 {
        fn Execute(&self, context:&mut  rustreport::Context) {
            context.set_font_size(14.0 as i32)
            ;
            context.write_text(50.0, 20.0, "Sales Report");
            context.write_text(240.0, 28.0, "Page");
            context.write_text(260.0, 28.0, &format!("{}",context.page));
            context.write_text(15.0, 28.0, "D No");
            context.write_text(30.0, 28.0, "Dept");
            context.write_text(60.0, 28.0, "Order");
            context.write_text(90.0, 28.0, "Stock");
            context.write_text(120.0, 28.0, "Name");
            context.write_text_right(160.0, 28.0, "Unit Price");
            context.write_text_right(180.0, 28.0, "Qty");
            context.write_text_right(210.0, 28.0, "Amount");
            context.cur_vpos=30.0;
        }
    }
    pub struct ReportSummaryS1;
    impl rustreport::exec::ReportSummary for ReportSummaryS1 {
        fn GetHeight(&self, context:&mut  rustreport::Context) -> f32 {
            10.0
        }
        fn Execute(&self, context:&mut  rustreport::Context) {
            context.set_font_size(12.0 as i32);
            context.write_text(160.0, context.cur_vpos+8.0, "Total");
            context.write_text_right(210.0, context.cur_vpos+8.0, rustreport::exec::f64_to_string_commad2(
                context.sum_work.get("amountcum=").unwrap().as_f64().unwrap()).as_str());
            context.cur_vpos=context.cur_vpos+10.0;
        }
    }
}
