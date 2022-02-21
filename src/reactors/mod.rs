use std::time::*;

pub enum EnergySource{
    Carbon,
    Gas,
    Water,
    Solar,
    Wind,
    Thermal,
    Fission,
    Fusion
}

pub struct Reactor{
    pub production: u128,
    pub duration: Duration,
    pub source: EnergySource,
    pub state: bool
}

impl Reactor{
    pub fn new(source: EnergySource) -> Reactor{
        let (production, duration ) = match source {
            EnergySource::Carbon => (1, Duration::from_secs(100)),
            EnergySource::Gas => (10, Duration::from_secs(100)),
            EnergySource::Water => (100, Duration::from_secs(200)),
            EnergySource::Solar => (1000, Duration::from_secs(200)),
            EnergySource::Wind => (10_000, Duration::from_secs(200)),
            EnergySource::Thermal => (100_000, Duration::from_secs(300)),
            EnergySource::Fission => (1_000_000, Duration::from_secs(400)),
            EnergySource::Fusion => (10_000_000, Duration::from_secs(500)),
        };

        Reactor { production, duration, source, state: true }
    }
}
