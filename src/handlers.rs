pub struct MyActivationHandler;
impl accesskit::ActivationHandler for MyActivationHandler {
    fn request_initial_tree(&mut self) -> Option<accesskit::TreeUpdate> {
        // Return None for now; can be updated to return Some(tree) if needed
        None
    }
}
// src/handlers.rs
// Custom accessibility handlers/adapters

pub struct MyActionHandler;
impl accesskit::ActionHandler for MyActionHandler {
    fn do_action(&mut self, _request: accesskit::ActionRequest) {}
}

pub struct MyDeactivationHandler;
impl accesskit::DeactivationHandler for MyDeactivationHandler {
    fn deactivate_accessibility(&mut self) {}
}
