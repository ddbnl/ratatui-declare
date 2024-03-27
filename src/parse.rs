use std::io::Read;
use std::str::FromStr;
use strum::EnumString;
use thiserror::Error;
use crate::widgets::Widget;


pub fn parse_template_files(path: &str) {
    let mut file = std::fs::File::open(path).unwrap_or_else(
        |e| panic!("Could not open root template file '{}': {}", path, e)
    );
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap_or_else(
        |e| panic!("Could not read root template file '{}': {}", path, e)
    );
    if let Err(e) = parse_text(text.as_str()) {
        panic!("Could not parse template file '{}': {}", path, e)
    }
}


fn parse_text(text: &str) -> Result<(), TemplateParserError> {

    let line_no = 0;
    let mut current_indentation = 0;

    for line in text.lines() {

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
        if line_indentation.abs_diff(current_indentation) > 4 || line_indentation % 4 != 0{
            return Err(TemplateParserError::InvalidIndentation(line_no, current_indentation, line.to_string()))
        }

        if trimmed_line.ends_with(':') {
            current_widget = Some(parse_widget(trimmed_line, line_no)?);
        } else if trimmed_line.contains(':') {
            if let Some(widget) = current_widget.as_mut() {
                parse_value(trimmed_line, widget, line_no)?;
            } else {
                return Err(TemplateParserError::DanglingValue(line_no, trimmed_line.to_string()))
            }
        } else {
            return Err(TemplateParserError::InvalidContext(line_no, line.to_string()))
        }
        current_indentation = line_indentation;
    }
    Ok(())
}

fn parse_widget(line: &str, line_number: usize) -> Result<Box<dyn Widget>, TemplateParserError> {


    if !line.contains(':') {
        return Err(TemplateParserError::InvalidWidgetDeclaration(line_number, line.to_string()))
    }
    let widget_name = line.strip_suffix(":").unwrap();
    if let Ok(widget) = WidgetTypes::from_str(widget_name) {
        Ok(widget.into())
    } else {
        return Err(TemplateParserError::InvalidWidgetType(line_number, widget_name.to_string()))
    }
}

fn parse_value(line: &str, widget: &mut Box<dyn Widget>, line_number: usize) -> Result<(), TemplateParserError> {

    if !line.contains(':') {
        return Err(TemplateParserError::InvalidWidgetDeclaration(line_number, line.to_string()))
    }
    let (k, v) = line.split_once(':').unwrap();
    widget.load_value(k, v, line_number)?;
    Ok(())
}


#[derive(EnumString)]
pub enum WidgetTypes {
    Layout,
    Paragraph,
}

#[derive(Error, Debug, PartialOrd, PartialEq)]
pub enum TemplateParserError {
    #[error("Root template must contain exactly one root widget.")]
    NoRoots,
    #[error("Root template must contain exactly one root widget.")]
    TooManyRoots(String),
    #[error("Line {0}: Expected indention level '{1}', but found this instead: {2}")]
    InvalidIndentation(usize, usize, String),
    #[error("Line {0}: Expected widget declaration ending with a ':', but found this instead: {1}")]
    InvalidWidgetDeclaration(usize, String),
    #[error("Line {0}: Invalid widget type '{1}'")]
    InvalidWidgetType(usize, String),
    #[error("Line {0}: Expected a widget declaration ending ':', or a value declaration 'foo: bar', but found this instead: '{1}'")]
    InvalidContext(usize, String),
    #[error("Line {0}: Widget '{1}' has no attribute '{2}'")]
    InvalidWidgetAttributeName(usize, String, String),
    #[error("Line {0}: Attribute '{1}' for widget '{2}' has an invalid value '{3}'")]
    InvalidWidgetAttributeValue(usize, String, String, String),
    #[error("Line {0}: Value '{1}' does not belong to any widget")]
    DanglingValue(usize, String),
}


#[test]
fn test_template_empty() {
    let test = r#""#;
    let e = parse_template_file(test).unwrap_err();
    assert_eq!(e, TemplateParserError::NoRoots);
}

#[test]
fn test_template_too_many_roots() {
    let test = r#"
    BoxLayout:
        Label:
            text: "Hello!"
    BoxLayout:
        Label:
            text: "Hello!"
    "#;
    let e = parse_template_file(test).unwrap_err();
    assert_eq!(e, TemplateParserError::TooManyRoots("BoxLayout, BoxLayout".to_string()));
}
