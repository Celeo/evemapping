#![allow(unused)]

use std::fmt;

#[derive(Debug, Clone)]
pub enum SystemAffect {
    None,
    Pulsar,
    BlackHole,
    CataVar,
    Magnetar,
    RedGiant,
    WolfRayet,
}

#[derive(Debug, Clone)]
pub enum SystemClassification {
    Unknown,
    HighSec,
    LowSec,
    NullSec,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C13,
    Thera,
}

#[derive(Debug, Clone)]
pub struct StaticWormhole {
    classifier: String,
    source: SystemClassification,
    destination: SystemClassification,
}

#[derive(Debug, Clone)]
pub struct WSpaceData {
    affect: SystemAffect,
    statics: Vec<StaticWormhole>,
}

#[derive(Debug, Clone)]
pub enum SystemType {
    KSpace,
    WSpace(WSpaceData),
}

/// Represents a system.
#[derive(Debug, Clone)]
pub struct System {
    id: u32,
    name: String,
    classification: SystemClassification,
    system_type: SystemType,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct AnomalyId {
    pub id: String,
    pub number: u16,
}

impl AnomalyId {
    pub fn new(id: &str, number: u16) -> Self {
        Self {
            id: id.to_owned(),
            number,
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum AnomalyType {
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
#[derive(Debug, Clone)]
pub struct Anomaly {
    identifier: AnomalyId,
    anomaly_type: AnomalyType,
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
