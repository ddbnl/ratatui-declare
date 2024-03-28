use std::collections::HashMap;
use ratatui_declare::parse;
use ratatui_declare::run;
fn main() {
    let root = parse::parse_template_files("C:\\Users\\daand\\RustroverProjects\\ratatui-declare\\src\\ratatui-declare-test\\src\\templates\\basic.rat");

    let replacements = HashMap::from([
        ("hello", "Hello, "),
        ("world", "World!")
    ]);
    
    run::render(root, &replacements).unwrap()
}
