use ratatui::layout::Direction;
use crate::parse::{TemplateParserError, WidgetTypes};

pub trait Widget {
    fn load_value(&mut self, k: &str, v: &str, line: usize) -> Result<(), TemplateParserError>;
}

impl From<WidgetTypes> for Box<dyn Widget> {
    fn from(value: WidgetTypes) -> Self {
       match value {
           WidgetTypes::Layout => Box::new(Layout::default()),
           WidgetTypes::Paragraph => Box::new(Paragraph::default())
       } 
    }
}


#[derive(Default)]
pub struct Layout {
    direction: Direction,
    children: Vec<Box<dyn Widget>>
}
impl Widget for Layout {
    fn load_value(&mut self, k: &str, v: &str, line: usize) -> Result<(), TemplateParserError>{
        
        match k {
            "Direction" => {
                match v {
                    "Horizontal" => self.direction = Direction::Horizontal,
                    "Vertical" => self.direction = Direction::Vertical,
                    _ => return Err(TemplateParserError::InvalidWidgetAttributeValue(line, "Layout".to_string(), k.to_string(), v.to_string()))
                }
            }
            _ => return Err(TemplateParserError::InvalidWidgetAttributeName(line, "Layout".to_string(), k.to_string()))
        } 
        Ok(())
    }
}


#[derive(Default, Debug)]
pub struct Paragraph {
    text: String
}
impl Widget for Paragraph {
    fn load_value(&mut self, k: &str, v: &str, line: usize) -> Result<(), TemplateParserError>{

        match k {
            "Text" => self.text = v.to_string(),
            _ => return Err(TemplateParserError::InvalidWidgetAttributeName(line, "Layout".to_string(), k.to_string()))
        }
        Ok(())
    }
}
