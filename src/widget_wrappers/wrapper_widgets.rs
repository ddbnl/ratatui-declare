use crate::widget_wrappers::wrapper_layout::WrapperLayout;
use crate::widget_wrappers::wrapper_paragraph::WrapperParagraph;
use crate::widget_wrappers::wrapper_widget::WrapperWidget;


#[derive(Debug)]
pub enum WrapperWidgets {
    Layout(WrapperLayout),
    Paragraph(WrapperParagraph),
}
impl WrapperWidgets {
    pub fn to_box(&mut self) -> Box<&mut dyn WrapperWidget> {
        match self {
            WrapperWidgets::Layout(layout) => Box::new(layout),
            WrapperWidgets::Paragraph(paragraph) => Box::new(paragraph),
        }
    }
}
impl From<&str> for WrapperWidgets {
    fn from(value: &str) -> Self {
       match value {
           "Layout" => WrapperWidgets::Layout(WrapperLayout::default()),
           "Paragraph" => WrapperWidgets::Paragraph(WrapperParagraph::default()),
           _ => panic!("Widget '{}' not a layout", value)
       }
    }
}