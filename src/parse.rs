use std::io::Read;
use thiserror::Error;
use crate::widget_wrappers::wrapper_widgets::WrapperWidgets;


pub fn parse_template_files(path: &str) -> WrapperWidgets {
    
    let mut file = std::fs::File::open(path).unwrap_or_else(
        |e| panic!("Could not open root template file '{}': {}", path, e)
    );
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap_or_else(
        |e| panic!("Could not read root template file '{}': {}", path, e)
    );
    match parse_text(text.as_str()) {
        Ok(widget) => widget,
        Err(e) => panic!("Could not parse template file '{}': {}", path, e)
    }
}


fn parse_text(text: &str) -> Result<(WrapperWidgets), TemplateParserError> {

    let mut line_no = 0;
    let mut lines: Vec<&str> = text.lines().collect();
    
    while !lines.is_empty() {
        let line = lines.remove(0);
        if line.trim().is_empty() || line.trim().starts_with("//") {
            line_no += 1;
            continue
        }
        if line != "Layout:" {
            return Err(TemplateParserError::RootWidget(line_no, line.to_string()))
        }
        let mut root = WrapperWidgets::from(line.strip_suffix(':').unwrap());
        root.to_box().load_config(&mut lines, &mut line_no, 4)?;
        return Ok(root)
    }
    Err(TemplateParserError::NoRoots)
}


#[derive(Error, Debug, PartialOrd, PartialEq)]
pub enum TemplateParserError {

    #[error("Line {0}: First declaration must be a layout, but found this instead: '{1}'.")]
    RootWidget(usize, String),
    
    #[error("Root template must contain exactly one root widget.")]
    NoRoots,
    
    #[error("Line {0}: Root template must contain exactly one root level declaration, but found a second: {1}")]
    TooManyRoots(usize, String),
    
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
    
    #[error("Line {0}: Attribute '{1}' was set after a child-widget declaration. Attributes must be set first. ")]
    LateAttribute(usize, String),
    
    #[error("Line {0}: Widgets must be added to layouts, not other widgets. ")]
    AddWidgetToNonContainer(usize),
}


#[test]
fn test_template_empty() {
    let test = r#""#;
    let e = parse_text(test).unwrap_err();
    assert_eq!(e, TemplateParserError::NoRoots);
}

#[test]
fn test_template_too_many_roots() {
    let test = r#"
    Layout:
        Paragraph:
            text: "Hello!"
    Layout:
        Paragraph:
            text: "Hello!"
    "#;
    let e = parse_text(test).unwrap_err();
    assert_eq!(e, TemplateParserError::TooManyRoots("BoxLayout, BoxLayout".to_string()));
}
