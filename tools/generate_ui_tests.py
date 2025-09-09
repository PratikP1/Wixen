import os
import re

SRC_FILE = 'src/main.rs'
TEST_DIR = '../tests/'
TEST_FILE = os.path.join(TEST_DIR, 'auto_ui_test.rs')

COMPONENT_PATTERN = re.compile(r'Role::(MenuBar|MenuItem|List|ListItem)')

def find_components():
    with open(SRC_FILE, 'r', encoding='utf-8') as f:
        src = f.read()
    return set(COMPONENT_PATTERN.findall(src))

def write_tests(components):
    with open(TEST_FILE, 'w', encoding='utf-8') as f:
        f.write('use crate::build_accessibility_tree;\nuse crate::AppState;\nuse crate::FocusArea;\n\n')
        for comp in components:
            f.write(f"#[test]\ndef test_{comp.lower()}_accessible() {{\n    // TODO: Set up AppState for {comp}\n    let state = AppState::default();\n    let tree = build_accessibility_tree(&state);\n    let found = tree.nodes.iter().any(|(_, node)| node.role() == accesskit::Role::{comp});\n    assert!(found, \"{comp} should be accessible\");\n}}\n\n")

if __name__ == '__main__':
    os.makedirs(TEST_DIR, exist_ok=True)
    components = find_components()
    write_tests(components)
    print(f"Generated tests for: {', '.join(components)}")
