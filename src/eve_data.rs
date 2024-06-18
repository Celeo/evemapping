pub enum SystemAffect {
    None,
    Pulsar,
    BlackHole,
    CataVar,
    Magnetar,
    RedGiant,
    WolfRayet,
}

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
}

pub struct StaticWormhole {
    classifier: String,
    source: SystemClassification,
    destination: SystemClassification,
}

pub struct WSpaceData {
    affect: SystemAffect,
    statics: Vec<StaticWormhole>,
}

pub enum SystemType {
    KSpace,
    WSpace(WSpaceData),
}

pub struct System {
    id: u32,
    name: String,
    classification: SystemClassification,
    system_type: SystemType,
}

pub enum WormholeLife {
    Stable,
    EndOfLife,
}

pub enum WormholeMass {
    Stable,
    Destab,
    Critical,
}

pub struct AnomalyId {
    id: String,
    number: u8,
}

pub enum Anomaly {
    Unknown(AnomalyId),
    Combat(AnomalyId, Option<String>),
    Wormhole(AnomalyId, String, String, WormholeLife, WormholeMass),
    Ore(AnomalyId, Option<String>),
    Data(AnomalyId, Option<String>),
    Relic(AnomalyId, Option<String>),
    Gas(AnomalyId, Option<String>),
}
