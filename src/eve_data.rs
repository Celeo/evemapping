#![allow(unused)]

use anyhow::Result;
use log::info;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SignatureId {
    pub id: String,
    pub number: u16,
}

impl fmt::Display for SignatureId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.id, self.number)
    }
}

impl SignatureId {
    pub fn new(id: &str, number: u16) -> Self {
        Self {
            id: id.to_owned(),
            number,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SignatureWormhole {
    pub wh_type: Option<String>,
    pub destination: Option<String>,
    pub life: WormholeLife,
    pub mass: WormholeMass,
}

impl Default for SignatureWormhole {
    fn default() -> Self {
        Self {
            wh_type: None,
            destination: None,
            life: WormholeLife::Stable,
            mass: WormholeMass::Stable,
        }
    }
}

impl SignatureWormhole {
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

#[derive(Debug, Clone, PartialEq, Default)]
pub enum SignatureType {
    #[default]
    Unknown,
    /// Fields: name
    Combat(Option<String>),
    Wormhole(SignatureWormhole),
    /// Fields: name
    Ore(Option<String>),
    /// Fields: name
    Data(Option<String>),
    /// Fields: name
    Relic(Option<String>),
    /// Fields: name
    Gas(Option<String>),
}

impl fmt::Display for SignatureType {
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
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Signature {
    pub identifier: SignatureId,
    pub signature_type: SignatureType,
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}      {}",
            self.identifier.id, self.identifier.number, self.signature_type
        )
    }
}

impl Signature {
    pub fn new(id: &str, number: u16, ty: SignatureType) -> Self {
        Self {
            identifier: SignatureId::new(id, number),
            signature_type: ty,
        }
    }
}

#[derive(Deserialize)]
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
    serde_json::from_str(raw).unwrap()
});

/// Data about a single system.
#[derive(Deserialize)]
pub struct SystemData {
    pub security: f32,
    pub class: Option<u8>,
    pub effect: Option<String>,
    pub statics: Vec<String>,
}

pub enum SystemClassification {
    HighSec,
    LowSec,
    NullSec,
    WSpace(u8),
}

impl SystemClassification {
    pub fn as_str(&self) -> String {
        match self {
            Self::HighSec => String::from("High-Sec"),
            Self::LowSec => String::from("Low-Sec"),
            Self::NullSec => String::from("Null-Sec"),
            Self::WSpace(class) => format!("Class-{class}"),
        }
    }
}

impl SystemData {
    /// Typical system security classification options.
    pub fn classification(&self) -> SystemClassification {
        if let Some(c) = self.class {
            SystemClassification::WSpace(c)
        } else if self.security >= 0.5 {
            SystemClassification::HighSec
        } else if self.security >= 0.1 {
            SystemClassification::LowSec
        } else {
            SystemClassification::NullSec
        }
    }
}

/// All systems in the game, K-space and W-space.
pub static ALL_SYSTEMS: Lazy<HashMap<String, SystemData>> = Lazy::new(|| {
    let raw = include_str!("../static/systems.json");
    serde_json::from_str(raw).unwrap()
});

#[derive(Debug)]
pub struct ClipboardItem {
    pub id: String,
    pub sig_type: String,
    pub sig_name: String,
}

impl ClipboardItem {
    fn new(
        id: impl Into<String>,
        sig_type: impl Into<String>,
        sig_name: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            sig_type: sig_type.into(),
            sig_name: sig_name.into(),
        }
    }
}

impl From<&ClipboardItem> for (SignatureId, SignatureType) {
    fn from(val: &ClipboardItem) -> Self {
        let mut id_parts = val.id.split(':');
        let id = SignatureId::new(
            id_parts.next().unwrap(),
            id_parts.next().unwrap().parse().unwrap(),
        );

        let name = if val.sig_name.is_empty() {
            None
        } else {
            Some(val.sig_type.clone())
        };
        let st = if val.sig_type == "Wormhole" {
            SignatureType::Wormhole(SignatureWormhole::default())
        } else if val.sig_type == "Gas" {
            SignatureType::Gas(name)
        } else if val.sig_type == "Relic" {
            SignatureType::Relic(name)
        } else if val.sig_type == "Data" {
            SignatureType::Data(name)
        } else if val.sig_type == "Combat" {
            SignatureType::Combat(name)
        } else {
            SignatureType::Unknown
        };

        (id, st)
    }
}

/// Parse clipboard data to extract any cosmic signature data.
pub fn parse_paste(text: &str) -> Vec<ClipboardItem> {
    if text.trim().is_empty() {
        return Vec::new();
    }
    let mut findings: Vec<ClipboardItem> = Vec::new();
    for line in text.split_terminator('\n') {
        let id: String = line.chars().take(6).collect();
        let parts = line.split('\t').skip(2).collect::<Vec<_>>();
        if parts.is_empty() {
            continue;
        }
        if parts[0] == "Wormhole" {
            findings.push(ClipboardItem::new(id, "", ""));
        } else if parts[0] == "Gas Site"
            || parts[0] == "Relic Site"
            || parts[0] == "Data Site"
            || parts[0] == "Combat Site"
        {
            let name = match parts.get(1) {
                Some(s) => s,
                None => "",
            };
            findings.push(ClipboardItem::new(id, parts[0].replace(" Site", ""), name));
        } else {
            findings.push(ClipboardItem::new(id, "", ""));
        }
    }
    findings
}

#[cfg(test)]
mod tests {
    use super::{parse_paste, ClipboardItem};

    #[test]
    fn test_parse_paste() {
        let text = r#"OEB-892	Cosmic Signature	Wormhole	Unstable Wormhole	100.0%	4.99 AU
YQS-184	Cosmic Signature	Wormhole	Unstable Wormhole	100.0%	2.93 AU
OVD-328	Cosmic Signature	Wormhole	Unstable Wormhole	100.0%	71 km
WIV-940	Cosmic Signature	Relic Site	Ruined Blood Raider Temple Site	100.0%	8.98 AU
ROZ-580	Cosmic Signature	Relic Site	Ruined Angel Temple Site	100.0%	2.07 AU
MJK-752	Cosmic Signature	Wormhole	Unstable Wormhole	100.0%	6.89 AU
ZYP-580	Cosmic Signature			10.4%	5.77 AU
LHB-560	Cosmic Signature			8.8%	2.95 AU
WYT-700	Cosmic Signature	Gas Site		5.2%	4.02 AU"#;
        let results = parse_paste(text);
        let expected: Vec<ClipboardItem> = vec![
            ClipboardItem::new("OEB-892", "Wormhole", ""),
            ClipboardItem::new("YQS-184", "Wormhole", ""),
            ClipboardItem::new("OVD-328", "Wormhole", ""),
            ClipboardItem::new("WIV-940", "Relic", "Ruined Blood Raider Temple Site"),
            ClipboardItem::new("ROZ-580", "Relic", "Ruined Angel Temple Site"),
            ClipboardItem::new("MJK-752", "Wormhole", ""),
            ClipboardItem::new("ZYP-580", "", ""),
            ClipboardItem::new("LHB-560", "", ""),
            ClipboardItem::new("WYT-700", "Gas", ""),
        ];
    }
}
