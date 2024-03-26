use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;
use strum::EnumString;
use thiserror::Error;

type ArbitraryYaml = Vec<HashMap<String, serde_yaml::Value>>;


pub fn parse_template_files(path: &str) {
    let mut file = std::fs::File::open(path).unwrap_or_else(
        |e| panic!("Could not open root template file '{}': {}", path, e)
    );
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap_or_else(
        |e| panic!("Could not read root template file '{}': {}", path, e)
    );
    parse_template_file(text.as_str());
}


fn parse_template_file(template: &str) -> Result<(), TemplateParserError>{

    Ok(())
}

fn parse_text(text: &str) {

    let line_no = 0;
    let mut current_indentation = 0;

    for line in text.lines() {

        let trimmed_line = line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue // Comment or empty line
        }

        let mut line_indentation = 0;
        for char in line {
            if char != ' ' {
                break
            }
            line_indentation += 1;
        }
        if line_indentation.abs_diff(current_indentation) > 4 || line_indentation % 4 != 0{
            return Err(TemplateParserError::InvalidIndentation(line_no, current_indentation, line))
        }

        if context.is_none() {
            if trimmed_line.ends_with(':') {
                parse_widget(trimmed_line, line_no)?;
            } else if trimmed_line.contains(':') {
                parse_value(trimmed_line, line_no)?
            } else {
                return Err(TemplateParserError::InvalidContext(line_no, line))
            }
        } else {
            match context {
                ParseContext::Widget => parse_widget(trimmed_line, line_no)?,
                ParseContext::Value => parse_value(trimmed_line, line_no)?,
            }
        }
    }
}

fn parse_widget(line: &str, line_number: usize) -> Result<(Widget), TemplateParserError>{

    if !line.ends_with(':') {
        return Err(TemplateParserError::InvalidWidgetDeclaration(line_number, line.to_string()))
    }
    let widget_name = line.strip_suffix(":").unwrap();
    if let Some(widget) = widget_name.into() {
        Ok(widget)
    } else {
        return Err(TemplateParserError::InvalidWidgetType(line_number, widget_name.to_string()))
    }
}

fn parse_value(line: &str, line_number: usize) -> Result<(Widget), TemplateParserError> {
    if !line.contains(':') {
        return Err(TemplateParserError::InvalidWidgetDeclaration(line_number, line.to_string()))
    }
    let widget_name = line.strip_suffix(":").unwrap();
    if let Some(widget) = widget_name.into() {
        Ok(widget)
    } else {
        return Err(TemplateParserError::InvalidWidgetType(line_number, widget_name.to_string()))
    }
}


#[derive(EnumString)]
pub enum Widget {
    BoxLayout,
    Label,
}

#[derive(Error, Debug, PartialOrd, PartialEq)]
pub enum TemplateParserError {
    #[error("Root template must contain exactly one root widget.")]
    NoRoots,
    #[error("Root template must contain exactly one root widget.")]
    TooManyRoots(String),
    #[error("Line {}: Expected indention level '{}', but found this instead: {}")]
    InvalidIndentation(usize, usize, String),
    #[error("Line {}: Expected widget declaration ending with a ':', but found this instead: {}")]
    InvalidWidgetDeclaration(usize, String),
    #[error("Line {}: Invalid widget type '{}'")]
    InvalidWidgetType(usize, String),
    #[error("Line {}: Expected a widget declaration ending ':', or a value declaration 'foo: bar', but found this instead: '{}'")]
    InvalidContext(usize, String),
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
