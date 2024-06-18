#![allow(unused)]

use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{collections::HashMap, fmt};

#[derive(Clone, PartialEq)]
pub enum WormholeLife {
    Stable,
    EndOfLife,
}

impl WormholeLife {
    pub fn as_str(&self) -> &'static str {
        match self {
            WormholeLife::Stable => "Stable",
            WormholeLife::EndOfLife => "EOL",
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum WormholeMass {
    Stable,
    Destab,
    Critical,
}

impl WormholeMass {
    pub fn as_str(&self) -> &'static str {
        match self {
            WormholeMass::Stable => "Stable",
            WormholeMass::Destab => "Destab",
            WormholeMass::Critical => "Critical",
        }
    }
}

#[derive(Clone, PartialEq, Default)]
pub struct AnomalyId {
    pub id: String,
    pub number: u16,
}

impl fmt::Display for AnomalyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.id, self.number)
    }
}

impl AnomalyId {
    pub fn new(id: &str, number: u16) -> Self {
        Self {
            id: id.to_owned(),
            number,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct AnomalyWormhole {
    pub wh_type: Option<String>,
    pub destination: Option<String>,
    pub life: WormholeLife,
    pub mass: WormholeMass,
}

impl AnomalyWormhole {
    pub fn new(
        wh_type: Option<String>,
        destination: Option<String>,
        life: WormholeLife,
        mass: WormholeMass,
    ) -> Self {
        Self {
            wh_type,
            destination,
            life,
            mass,
        }
    }
}

#[derive(Clone, PartialEq, Default)]
pub enum AnomalyType {
    #[default]
    Unknown,
    /// Fields: name
    Combat(Option<String>),
    Wormhole(AnomalyWormhole),
    /// Fields: name
    Ore(Option<String>),
    /// Fields: name
    Data(Option<String>),
    /// Fields: name
    Relic(Option<String>),
    /// Fields: name
    Gas(Option<String>),
}

impl fmt::Display for AnomalyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => {
                write!(f, "Unknown")
            }
            Self::Combat(name) => match name {
                Some(n) => write!(f, "Combat   {n}"),
                None => write!(f, "Combat"),
            },
            Self::Wormhole(data) => {
                write!(
                    f,
                    "WH       {} -> {}      {}      {}",
                    match data.wh_type.as_ref() {
                        Some(s) => s,
                        None => "?",
                    },
                    match data.destination.as_ref() {
                        Some(d) => d,
                        None => "?",
                    },
                    data.life.as_str(),
                    data.mass.as_str(),
                )
            }
            Self::Ore(name) => match name {
                Some(n) => write!(f, "Ore      {n}"),
                None => write!(f, "Ore"),
            },
            Self::Data(name) => match name {
                Some(n) => write!(f, "Data     {n}"),
                None => write!(f, "Data"),
            },
            Self::Relic(name) => match name {
                Some(n) => write!(f, "Relic    {n}"),
                None => write!(f, "Relic"),
            },
            Self::Gas(name) => match name {
                Some(n) => write!(f, "Gas      {n}"),
                None => write!(f, "Gas"),
            },
        }
    }
}

/// Represents a scannable item in space.
#[derive(Clone, PartialEq, Default)]
pub struct Anomaly {
    pub identifier: AnomalyId,
    pub anomaly_type: AnomalyType,
}

impl fmt::Display for Anomaly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}      {}",
            self.identifier.id, self.identifier.number, self.anomaly_type
        )
    }
}

impl Anomaly {
    pub fn new(id: &str, number: u16, ty: AnomalyType) -> Self {
        Self {
            identifier: AnomalyId::new(id, number),
            anomaly_type: ty,
        }
    }
}

#[derive(Clone, Deserialize)]
pub struct WormholeInfo {
    pub life: String,
    pub from: Vec<String>,
    #[serde(rename = "leadsTo")]
    pub leads_to: String,
    pub mass: u64,
    pub jump: u64,
}

/// All wormhole types in a map of identifier to data.
pub static WORMHOLE_TYPES: Lazy<HashMap<String, WormholeInfo>> = Lazy::new(|| {
    let raw = include_str!("../static/wormhole_types.json");
    let parsed = serde_json::from_str(raw).unwrap();
    parsed
});

/// Data about a single system.
#[derive(Clone, Deserialize)]
pub struct SystemData {
    pub security: String,
    pub class: Option<u8>,
    pub effect: Option<String>,
    pub statics: Vec<String>,
}

pub enum SystemClassification {
    HighSec,
    LowSec,
    NullSec,
    WSpace(String),
    Thera,
}

impl SystemData {
    pub fn classification(&self) -> SystemClassification {
        todo!()
    }
}

/// All systems in the game, K-space and W-space.
pub static ALL_SYSTEMS: Lazy<HashMap<String, SystemData>> = Lazy::new(|| {
    let raw = include_str!("../static/systems.json");
    let parsed = serde_json::from_str(raw).unwrap();
    parsed
});

/// Lookup system data.
pub fn get_system_data(name: &str) -> Option<&SystemData> {
    ALL_SYSTEMS.get(name)
}
