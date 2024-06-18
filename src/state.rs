use crate::eve_data::Anomaly;
use std::collections::HashMap;

// App state.
pub struct App {
    pub current_system: Option<String>,
    pub system_data: HashMap<String, Vec<Anomaly>>,

    pub data_index: Option<u16>,

    pub is_adding: bool,
    pub is_editing: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_system: None,
            system_data: HashMap::new(),

            data_index: None,

            is_adding: false,
            is_editing: false,
        }
    }
}
