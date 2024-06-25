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
                    "123",
                    SignatureType::Combat(Some("Some Combat Site".to_owned())),
                ),
                Signature::new(
                    "DEF",
                    "456",
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
            let existing_ids: Vec<_> = self
                .system_data
                .get(current_system)
                .unwrap()
                .iter()
                .map(|sig| sig.identifier.clone())
                .collect();

            // update existing data
            let existing = self.system_data.get_mut(current_system).unwrap();
            for signature in existing {
                let id: String = format!("{}", signature.identifier);
                if let Some(check) = new_data.iter().find(|d| d.id == id) {
                    let (_new_id, new_type) = check.into();
                    match new_type {
                        SignatureType::Unknown => {
                            // no new information; leave it
                        }
                        SignatureType::Wormhole(_) => {
                            match &signature.signature_type {
                                SignatureType::Wormhole(signature_wh) => {
                                    // existing signature is a wormhole, so nothing to do
                                    // since the scanner doesn't give any additional info
                                }
                                _ => {
                                    // existing signature is something else (likely unknown),
                                    // so overwrite with a default wormhole
                                    signature.signature_type =
                                        SignatureType::Wormhole(SignatureWormhole::default());
                                }
                            }
                        }
                        _ => {
                            if new_type.has_name() {
                                // overwrite with the new data since the new data has the same
                                signature.signature_type = new_type;
                            } else if signature.signature_type.has_name() {
                                // existing has a name; do nothing
                            } else {
                                // neither has the name, so overwrite in case the classifier updated
                                signature.signature_type = new_type;
                            }
                        }
                    }
                }
            }

            // insert any new items
            let existing = self.system_data.get_mut(current_system).unwrap();
            for clipboard_item in new_data {
                let (new_sig_id, new_sig_type) = clipboard_item.into();
                if !existing_ids.contains(&new_sig_id) {
                    existing.push(Signature {
                        identifier: new_sig_id,
                        signature_type: new_sig_type,
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::App;
    use crate::eve_data::{
        ClipboardItem, Signature, SignatureType, SignatureWormhole, WormholeLife,
    };

    #[test]
    fn test_app_merge_in_empty_empty() {
        let mut app = App::new();
        app.current_system = Some("Thera".to_owned());
        app.merge_in(&[]);

        assert!(app.system_data.contains_key("Thera"));
        assert!(app.system_data.get("Thera").unwrap().is_empty());
    }

    #[test]
    fn test_app_merge_in_some_empty() {
        let mut app = App::new();
        app.current_system = Some("Thera".to_owned());
        app.system_data.insert(
            "Thera".to_owned(),
            vec![Signature::new(
                "ABC",
                "123",
                crate::eve_data::SignatureType::Data(Some("Foobar".to_owned())),
            )],
        );
        app.merge_in(&[]);

        assert_eq!(app.system_data.get("Thera").unwrap().len(), 1);
    }

    #[test]
    fn test_app_merge_in_update_type() {
        let mut app = App::new();
        app.current_system = Some("Thera".to_owned());
        app.system_data.insert(
            "Thera".to_owned(),
            vec![Signature::new(
                "ABC",
                "123",
                crate::eve_data::SignatureType::Unknown,
            )],
        );

        app.merge_in(&[ClipboardItem::new("ABC-123", "Relic", "Foobar")]);

        assert_eq!(app.system_data.get("Thera").unwrap().len(), 1);
        let sig = app.system_data.get("Thera").unwrap().get(0).unwrap();
        assert_eq!(sig.identifier.id, "ABC".to_owned());
        assert_eq!(sig.identifier.number, "123");
        match sig.signature_type {
            crate::eve_data::SignatureType::Relic(ref name) => {
                assert_eq!(name, &Some("Foobar".to_owned()));
            }
            _ => {
                panic!("Should be a relic sig");
            }
        }
    }

    #[test]
    fn test_app_merge_in_add_name() {
        let mut app = App::new();
        app.current_system = Some("Thera".to_owned());
        app.system_data.insert(
            "Thera".to_owned(),
            vec![Signature::new(
                "ABC",
                "123",
                crate::eve_data::SignatureType::Relic(None),
            )],
        );

        app.merge_in(&[ClipboardItem::new("ABC-123", "Relic", "Foobar")]);

        assert_eq!(app.system_data.get("Thera").unwrap().len(), 1);
        let sig = app.system_data.get("Thera").unwrap().get(0).unwrap();
        assert_eq!(sig.identifier.id, "ABC".to_owned());
        assert_eq!(sig.identifier.number, "123");
        match sig.signature_type {
            crate::eve_data::SignatureType::Relic(ref name) => {
                assert_eq!(name, &Some("Foobar".to_owned()));
            }
            _ => {
                panic!("Should be a relic sig");
            }
        }
    }

    #[test]
    fn test_app_merge_in_no_overwrite_name() {
        let mut app = App::new();
        app.current_system = Some("Thera".to_owned());
        app.system_data.insert(
            "Thera".to_owned(),
            vec![Signature::new(
                "ABC",
                "123",
                crate::eve_data::SignatureType::Relic(Some("Foobar".to_owned())),
            )],
        );

        app.merge_in(&[ClipboardItem::new("ABC-123", "Relic", "")]);

        assert_eq!(app.system_data.get("Thera").unwrap().len(), 1);
        let sig = app.system_data.get("Thera").unwrap().get(0).unwrap();
        assert_eq!(sig.identifier.id, "ABC".to_owned());
        assert_eq!(sig.identifier.number, "123");
        match sig.signature_type {
            crate::eve_data::SignatureType::Relic(ref name) => {
                assert_eq!(name, &Some("Foobar".to_owned()));
            }
            _ => {
                panic!("Should be a relic sig");
            }
        }
    }

    #[test]
    fn test_app_merge_in_no_wormhole_data_overwrite() {
        let mut app = App::new();
        app.current_system = Some("Thera".to_owned());
        let wh = {
            let mut wh = SignatureWormhole::default();
            wh.life = WormholeLife::EndOfLife;
            wh.wh_type = Some("A239".to_owned());
            wh
        };
        app.system_data.insert(
            "Thera".to_owned(),
            vec![Signature::new(
                "ABC",
                "123",
                crate::eve_data::SignatureType::Wormhole(wh),
            )],
        );

        app.merge_in(&[ClipboardItem::new("ABC-123", "Wormhole", "")]);

        assert_eq!(app.system_data.get("Thera").unwrap().len(), 1);
        let sig = app.system_data.get("Thera").unwrap().get(0).unwrap();
        assert_eq!(sig.identifier.id, "ABC".to_owned());
        assert_eq!(sig.identifier.number, "123");
        match sig.signature_type {
            SignatureType::Wormhole(ref data) => {
                assert_eq!(data.wh_type, Some("A239".to_owned()));
                assert_eq!(data.life, WormholeLife::EndOfLife);
            }
            _ => {
                panic!("Should be a wormhole sig");
            }
        }
    }
}
