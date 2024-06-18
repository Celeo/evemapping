#![allow(unused)]

use crate::eve_data::{
    Anomaly, AnomalyId, AnomalyType, AnomalyWormhole, WormholeLife, WormholeMass,
};
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub enum ViewMode {
    Normal,
    Adding(Anomaly),
    Editing(Anomaly),
}

// App state.
pub struct App {
    pub current_system: Option<String>,
    pub system_data: HashMap<String, Vec<Anomaly>>,

    pub data_index: usize,

    pub view: ViewMode,
}

impl App {
    pub fn new() -> Self {
        let mut system_data = HashMap::new();
        system_data.insert(
            "J123456".to_owned(),
            vec![
                Anomaly::new(
                    "ABC",
                    123,
                    AnomalyType::Combat(Some("Some Combat Site".to_owned())),
                ),
                Anomaly::new(
                    "DEF",
                    456,
                    AnomalyType::Wormhole(AnomalyWormhole {
                        wh_type: Some("K162".to_owned()),
                        destination: None,
                        life: WormholeLife::Stable,
                        mass: WormholeMass::Stable,
                    }),
                ),
            ],
        );

        Self {
            current_system: Some("J123456".to_owned()),
            system_data,

            data_index: 0,

            view: ViewMode::Normal,
        }
    }

    pub fn system_anomalies(&self) -> Vec<&Anomaly> {
        if let Some(current_system) = self.current_system.as_ref() {
            if let Some(data) = self.system_data.get(current_system) {
                return data.iter().collect();
            }
        }
        Vec::new()
    }
}
