use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use regex::Regex;

fn main() {
    let src_file = "src/main.rs";
    let test_dir = "../tests/";
    let test_file = format!("{}auto_ui_test.rs", test_dir);
    let src = fs::read_to_string(src_file).expect("Failed to read src/main.rs");
    let re = Regex::new(r"Role::(MenuBar|MenuItem|List|ListItem)").unwrap();
    let mut components = vec![];
    for cap in re.captures_iter(&src) {
        components.push(cap[1].to_string());
    }
    fs::create_dir_all(test_dir).unwrap();
    let mut file = File::create(&test_file).unwrap();
    writeln!(file, "use crate::build_accessibility_tree;\nuse crate::AppState;\nuse crate::FocusArea;\n").unwrap();
    for comp in components {
        writeln!(file, "#[test]\ndef test_{}_accessible() {{\n    // TODO: Set up AppState for {}\n    let state = AppState::default();\n    let tree = build_accessibility_tree(&state);\n    let found = tree.nodes.iter().any(|(_, node)| node.role() == accesskit::Role::{});\n    assert!(found, \"{} should be accessible\");\n}}\n", comp.to_lowercase(), comp, comp, comp).unwrap();
    }
    println!("Generated tests for: {}", components.join(", "));
}
