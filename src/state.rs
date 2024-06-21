#![allow(unused)]

use cli_clipboard::x11_clipboard::Clipboard;

use crate::eve_data::{
    ClipboardItem, Signature, SignatureId, SignatureType, SignatureWormhole, WormholeLife,
    WormholeMass,
};
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub enum ViewMode {
    Normal,
    Adding(Signature),
    Editing(Signature),
}

// App state.
pub struct App {
    pub current_system: Option<String>,
    pub system_data: HashMap<String, Vec<Signature>>,

    pub data_index: usize,

    pub view: ViewMode,
}

impl App {
    pub fn new() -> Self {
        let mut system_data = HashMap::new();
        system_data.insert(
            "J173213".to_owned(),
            vec![
                Signature::new(
                    "ABC",
                    123,
                    SignatureType::Combat(Some("Some Combat Site".to_owned())),
                ),
                Signature::new(
                    "DEF",
                    456,
                    SignatureType::Wormhole(SignatureWormhole {
                        wh_type: Some("K162".to_owned()),
                        destination: None,
                        life: WormholeLife::Stable,
                        mass: WormholeMass::Stable,
                    }),
                ),
            ],
        );

        Self {
            current_system: Some("J173213".to_owned()),
            system_data,

            data_index: 0,

            view: ViewMode::Normal,
        }
    }

    pub fn system_signatures(&self) -> Vec<&Signature> {
        if let Some(current_system) = self.current_system.as_ref() {
            if let Some(data) = self.system_data.get(current_system) {
                return data.iter().collect();
            }
        }
        Vec::new()
    }

    /// Merge data from a paste into the existing system data.
    pub fn merge_in(&mut self, new_data: &[ClipboardItem]) {
        if let Some(current_system) = self.current_system.as_ref() {
            if !self.system_data.contains_key(current_system) {
                self.system_data
                    .insert(current_system.to_owned(), Vec::new());
            }
            let existing = self.system_data.get_mut(current_system).unwrap();
            for signature in existing {
                let id: String = format!("{}", signature.identifier);
                if let Some(check) = new_data.iter().find(|d| d.id == id) {
                    let (_new_id, new_type) = check.into();
                    match new_type {
                        SignatureType::Wormhole(existing_wh) => {
                            // TODO
                        }
                        _ => {
                            signature.signature_type = new_type;
                        }
                    }
                }
            }
            // TODO new items
        }
    }
}
