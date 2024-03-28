use ratatui::layout::{Constraint, Direction, Rect};
use crate::parse::TemplateParserError;
use anyhow::Result;
use ratatui::Frame;
use ratatui::layout::Layout as RatLayout;
use ratatui::prelude::*;


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

pub trait WrapperWidget {

    fn render(&mut self, maybe_layout: Option<Rect>, frame: &mut Frame) -> Result<()>;
    fn load_value(&mut self, k: &str, v: &str, line_no: usize) -> Result<(), TemplateParserError>;
    fn add_child(&mut self, child: WrapperWidgets, line_no: usize) -> Result<(), TemplateParserError>;
    fn load_config(&mut self,
                   lines: &mut Vec<&str>,
                   mut line_no: usize,
                   indent: usize)
        -> Result<(), TemplateParserError> {

        let mut widget_was_added= false;
        while !lines.is_empty() {

            let line = lines.remove(0);
            let trimmed_line = line.trim();
            if line.is_empty() || line.starts_with("//") {
                continue // Comment or empty line
            }

            let mut line_indentation: usize = 0;
            for char in line.chars() {
                if char != ' ' {
                    break
                }
                line_indentation += 1;
            }
            if line_indentation < indent {
                return Ok(())
            } else if line_indentation as isize - indent as isize > 4 || line_indentation % 4 != 0 {
                return Err(TemplateParserError::InvalidIndentation(line_no, indent, line.to_string()))
            }

            if trimmed_line.ends_with(':') {
                let widget_name = line.strip_suffix(":").unwrap().trim();
                let mut widget: WrapperWidgets = widget_name.into();
                widget.to_box().load_config(lines, line_no, indent + 4)?;
                self.add_child(widget, line_no)?;
                widget_was_added = true;
            } else if trimmed_line.contains(':') {
                if widget_was_added {
                    return Err(TemplateParserError::LateAttribute(line_no, line.to_string()))
                }
                let (k, v) = line.split_once(':').unwrap();
                self.load_value(k.trim(), v.trim(), line_no)?;
            } else {
                return Err(TemplateParserError::InvalidContext(line_no, line.to_string()))
            }
            line_no += 1;
        }

        Ok(())
    }

}


#[derive(Default)]
pub struct WrapperLayout {
    direction: Direction,
    children: Vec<WrapperWidgets>
}
impl WrapperWidget for WrapperLayout {
    fn render(&mut self, maybe_layout: Option<Rect>, frame: &mut Frame) -> Result<()> {

        let mut constraints = Vec::new();
        for _ in 0..self.children.len() {
            constraints.push(Constraint::Max(10));
        }
        let this = RatLayout::default().direction(self.direction).constraints(constraints.as_slice());
        let rects = if let Some(layout) = maybe_layout {
            this.split(layout)
        } else {
            this.split(frame.size())
        };
        for (i, child) in self.children.iter_mut().enumerate() {
            child.to_box().render(Some(rects[i]), frame)?;
        }
        Ok(())
    }
    fn load_value(&mut self, k: &str, v: &str, line: usize) -> Result<(), TemplateParserError>{

        match k {
            "direction" => {
                match v {
                    "horizontal" => self.direction = Direction::Horizontal,
                    "vertical" => self.direction = Direction::Vertical,
                    _ => return Err(TemplateParserError::InvalidWidgetAttributeValue(line, "Layout".to_string(), k.to_string(), v.to_string()))
                }
            }
            _ => return Err(TemplateParserError::InvalidWidgetAttributeName(line, "Layout".to_string(), k.to_string()))
        }
        Ok(())
    }
    fn add_child(&mut self, child: WrapperWidgets, _: usize) -> Result<(), TemplateParserError> {
        self.children.push(child);
        Ok(())
    }
}


#[derive(Default, Debug)]
pub struct WrapperParagraph {
    text: String
}
impl WrapperWidget for WrapperParagraph {
    fn render(&mut self, maybe_layout: Option<Rect>, frame: &mut Frame) -> Result<()> {

        let this = ratatui::widgets::Paragraph::new(self.text.clone());
        if let Some(layout) = maybe_layout {
            this.render(layout, frame.buffer_mut());
        } else {
            frame.render_widget(this, frame.size());
        }
        Ok(())
    }
    fn load_value(&mut self, k: &str, v: &str, line: usize) -> Result<(), TemplateParserError>{

        match k {
            "text" => self.text = v.to_string(),
            _ => return Err(TemplateParserError::InvalidWidgetAttributeName(line, "Paragraph".to_string(), k.to_string()))
        }
        Ok(())
    }
    fn add_child(&mut self, _: WrapperWidgets, line_no: usize) -> Result<(), TemplateParserError> {
       Err(TemplateParserError::AddWidgetToNonContainer(line_no))
    }
}
