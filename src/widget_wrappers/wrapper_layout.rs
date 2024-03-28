use std::collections::HashMap;
use ratatui::layout::{Constraint, Direction, Rect};
use crate::parse::TemplateParserError;
use anyhow::Result;
use ratatui::Frame;
use ratatui::layout::Layout as RatLayout;
use ratatui::prelude::*;
use crate::widget_wrappers::wrapper_widget::WrapperWidget;
use crate::widget_wrappers::wrapper_widgets::WrapperWidgets;


#[derive(Default, Debug)]
pub struct WrapperLayout {
    direction: Direction,
    children: Vec<WrapperWidgets>
}
impl WrapperWidget for WrapperLayout {
    fn render(&mut self,
              maybe_layout: Option<Rect>,
              replacements: &HashMap<&str, &str>,
              frame: &mut Frame) 
        -> Result<()> {

        let mut constraints = Vec::new();
        for _ in 0..self.children.len() {
            constraints.push(Constraint::Max(10));
        }
        let this = RatLayout::default()
            .direction(self.direction)
            .constraints(constraints.as_slice());
        let rects = if let Some(layout) = maybe_layout {
            this.split(layout)
        } else {
            this.split(frame.size())
        };
        for (i, child) in self.children.iter_mut().enumerate() {
            child.to_box().render(Some(rects[i]), replacements, frame)?;
        }
        Ok(())
    }
    fn load_value(&mut self, k: &str, v: &str, line: &usize) -> Result<(), TemplateParserError>{

        match k {
            "direction" => {
                match v {
                    "horizontal" => self.direction = Direction::Horizontal,
                    "vertical" => self.direction = Direction::Vertical,
                    _ => return Err(TemplateParserError::InvalidWidgetAttributeValue(
                        *line, "Layout".to_string(), k.to_string(), v.to_string()))
                }
            }
            _ => return Err(TemplateParserError::InvalidWidgetAttributeName(
                *line, "Layout".to_string(), k.to_string()))
        }
        Ok(())
    }
    fn add_child(&mut self, child: WrapperWidgets, _: &usize) -> Result<(), TemplateParserError> {
        self.children.push(child);
        Ok(())
    }
}