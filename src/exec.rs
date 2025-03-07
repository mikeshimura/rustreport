use crate::Context;
use num_format::{Locale, ToFormattedString};
pub trait Detail {
    fn GetHeight(&self, context: &mut Context) -> f32;
    fn Execute(&self,context:&mut Context);
    fn BreakCheckBefore(&self,context:&mut Context)->i32{
        0
    }
    fn BreakCheckAfter(&self,context:&mut Context)->i32{
        0
    }
}
pub trait PageHeader {
    fn Execute(&self,context:&mut Context);
}
pub trait GroupHeader {
    fn GetHeight(&self,context:&mut Context)->f32;
    fn Execute(&self,context:&mut Context);
}
pub trait Summary {
    fn GetHeight(&self,context:&mut Context)->f32;
    fn Execute(&self,context:&mut Context);
}
pub trait Footer {
    fn Execute(&self,context:&mut Context);
}
pub trait ReportSummary {
    fn GetHeight(&self,context:&mut Context)->f32;
    fn Execute(&self,context:&mut Context);
}
pub fn f64_roundedd2(f:f64)->f64{
    (f * 100.0).round() / 100.0
}
pub fn f64_to_string_commad2(f:f64)->String{
    let f_rounded = (f * 100.0).round() / 100.0;
    let integer_part = f_rounded.trunc() as i64;
    let decimal_part = (f_rounded.fract() * 100.0).round() as i64;
    let formatted_amt = format!(
        "{}.{:02}",
        integer_part.to_formatted_string(&Locale::en),
        decimal_part
    );
    formatted_amt
}
pub fn f64_to_string_commad1(f:f64)->String{
    let f_rounded = (f * 10.0).round() / 10.0;
    let integer_part = f_rounded.trunc() as i64;
    let decimal_part = (f_rounded.fract() * 10.0).round() as i64;
    let formatted_amt = format!(
        "{}.{:01}",
        integer_part.to_formatted_string(&Locale::en),
        decimal_part
    );
    formatted_amt
}
pub fn i64_to_string_comma(i:i64)->String{
     let formatted_amt = format!(
        "{}",
        i.to_formatted_string(&Locale::en),
     );
    formatted_amt
}
pub fn get_yen_string()->String{
    let code_point = 0x00A5;
    let yen_code =
        std::char::from_u32(code_point).expect("Invalid Unicode code point");
    let yen_str = yen_code.to_string();
    yen_str
}