use std::collections::HashMap;
use ratatui::layout::Rect;
use crate::parse::TemplateParserError;
use anyhow::Result;
use ratatui::Frame;
use ratatui::prelude::*;
use crate::widget_wrappers::wrapper_widget::{set_replacements, WrapperWidget};
use crate::widget_wrappers::wrapper_widgets::WrapperWidgets;


#[derive(Default, Debug)]
pub struct WrapperParagraph {
    text: String
}
impl WrapperWidget for WrapperParagraph {
    fn render(&mut self,
              maybe_layout: Option<Rect>,
              replacements: &HashMap<&str, &str>,
              frame: &mut Frame)
        -> Result<()> {

        let text = set_replacements(self.text.clone(), replacements);
        let this = ratatui::widgets::Paragraph::new(text);
        if let Some(layout) = maybe_layout {
            this.render(layout, frame.buffer_mut());
        } else {
            frame.render_widget(this, frame.size());
        }
        Ok(())
    }
    fn load_value(&mut self, k: &str, v: &str, line: &usize) -> Result<(), TemplateParserError>{

        match k {
            "text" => self.text = v.to_string(),
            _ => return Err(TemplateParserError::InvalidWidgetAttributeName(*line, "Paragraph".to_string(), k.to_string()))
        }
        Ok(())
    }
    fn add_child(&mut self, _: WrapperWidgets, line_no: &usize) -> Result<(), TemplateParserError> {
       Err(TemplateParserError::AddWidgetToNonContainer(*line_no))
    }
}
