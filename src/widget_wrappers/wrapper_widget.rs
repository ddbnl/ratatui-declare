use std::collections::HashMap;
use ratatui::layout::Rect;
use crate::parse::TemplateParserError;
use anyhow::Result;
use ratatui::Frame;
use ratatui::prelude::*;
use ratatui::widgets::GraphType::Scatter;
use crate::widget_wrappers::wrapper_widgets::WrapperWidgets;


pub trait WrapperWidget {

    fn render(&mut self,
              maybe_layout: Option<Rect>,
              replacements: &HashMap<&str, &str>,
              frame: &mut Frame)
        -> Result<()>;
    fn load_value(&mut self, k: &str, v: &str, line_no: &usize) -> Result<(), TemplateParserError>;
    fn add_child(&mut self, child: WrapperWidgets, line_no: &usize) -> Result<(), TemplateParserError>;
    fn load_config(&mut self,
                   lines: &mut Vec<&str>,
                   line_no: &mut usize,
                   indent: usize)
        -> Result<(), TemplateParserError> {

        let mut widget_was_added= false;
        while !lines.is_empty() {

            let line = lines.remove(0);
            let trimmed_line = line.trim();
            if line.is_empty() || line.starts_with("//") {
                continue // Comment or empty line
            }

            // Validate indention
            let line_indentation= get_indentation_level(line);
            if line_indentation < indent {
                lines.insert(0, line);
                return Ok(()) // Lower indentation level, this widget has finished loading
            } else if line_indentation == 0 {
                return Err(TemplateParserError::TooManyRoots(*line_no, line.to_string()))
            } else if line_indentation as isize - indent as isize > 4 || line_indentation % 4 != 0 {
                return Err(TemplateParserError::InvalidIndentation(*line_no, indent, line.to_string()))
            }

            self.load_line(trimmed_line, lines, line_no, indent, &mut widget_was_added)?;

            *line_no += 1;
        }
        Ok(())
    }

    fn load_line(&mut self,
                 line: &str,
                 remaining_lines: &mut Vec<&str>,
                 line_no: &mut usize,
                 indent: usize,
                 widget_was_added: &mut bool)
        -> Result<(), TemplateParserError> {

        if line.ends_with(':') {  // Widgets end with ':'
            let widget_name = line.strip_suffix(":").unwrap().trim();
            let mut widget: WrapperWidgets = widget_name.into();
            widget.to_box().load_config(remaining_lines, line_no, indent + 4)?;
            self.add_child(widget, line_no)?;
            *widget_was_added = true;
        } else if line.contains(':') {  // Values have 'k: v'
            if *widget_was_added {
                return Err(TemplateParserError::LateAttribute(*line_no, line.to_string()))
            }
            let (k, v) = line.split_once(':').unwrap();
            self.load_value(k.trim(), v.trim(), line_no)?;
        } else {  // No other allowed lines except for empty lines and comments
            return Err(TemplateParserError::InvalidContext(*line_no, line.to_string()))
        }
        Ok(())
    }
}

fn get_indentation_level(line: &str) -> usize {

    let mut line_indentation: usize = 0;
    for char in line.chars() {
        if char != ' ' {
            break
        }
        line_indentation += 1;
    }
    line_indentation
}


pub fn set_replacements(mut line:String, replacements: &HashMap<&str, &str>) -> String {

    for (k, v) in replacements {
        let replace = format!("{{{{{}}}}}", k);
        line = line.replace(replace.as_str(), *v);
    }
    line.strip_prefix('"').unwrap().strip_suffix('"').unwrap().to_string()
}