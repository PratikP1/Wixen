use std::process::Command;

fn main() {
    // Compile Slint UI
    slint_build::compile("MainWindow.slint").unwrap();

    // Run the Python UI test generator before every build
    let status = Command::new("python")
        .arg("tools/generate_ui_tests.py")
        .status()
        .expect("Failed to run UI test generator");
    if !status.success() {
        panic!("UI test generation failed");
    }
}
