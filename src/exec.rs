use crate::Context;

pub trait Detail {
    fn GetHeight(&self,context:Context)->f32;
    fn Execute(&self,context:Context);
    fn BreakCheckBefore(&self,context:Context)->i32{
        0
    }
    fn BreakCheckAfter(&self,context:Context)->i32{
        0
    }
}
pub trait PageHeader {
    fn Execute(&self,context:Context);
}
pub trait Summary {
    fn GetHeight(&self,context:Context)->f32;
    fn Execute(&self,context:Context);
}
pub trait Footer {
    fn GetHeight(&self,context:Context)->f32;
    fn Execute(&self,context:Context);
}
pub trait ReportSummary {
    fn GetHeight(&self,context:Context)->f32;
    fn Execute(&self,context:Context);
}