#[derive(Debug, Clone, PartialEq)]
pub enum FlightPhase {
    PreLaunch,
    PoweredAscentStage1,
    Meco,
    StageSeparation,
    PoweredAscentStage2,
    OrbitReached,
}

pub fn transition(current: &FlightPhase, velocity: f32, accel: f32) -> FlightPhase {
    match current {
        FlightPhase::PreLaunch => {
            if accel > 2.0 { FlightPhase::PoweredAscentStage1}
            else {FlightPhase::PreLaunch}
        }

        FlightPhase::PoweredAscentStage1 => {
            if accel < 0.5 {FlightPhase::Meco}
            else {FlightPhase::PoweredAscentStage1}
        }

        FlightPhase::Meco => FlightPhase::StageSeparation, // seperating arms
        FlightPhase::StageSeparation => FlightPhase::PoweredAscentStage2,

        FlightPhase::PoweredAscentStage2 => {
            if velocity > 7800.0 { FlightPhase::OrbitReached}
            else {FlightPhase::PoweredAscentStage2}
        }

        FlightPhase::OrbitReached => FlightPhase::OrbitReached,
    }
}