use ratatui_declare::parse;
use ratatui_declare::run;
fn main() {
    let root = parse::parse_template_files("C:\\Users\\daand\\RustroverProjects\\ratatui-declare\\src\\ratatui-declare-test\\src\\templates\\basic.rat");
    run::render(root).unwrap()
}
