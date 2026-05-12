#[derive(Debug, Clone, PartialEq)]
enum FlightPhase {
    PreLaunch,
    PoweredAscentStage1,
    Meco,
    StageSeperation,
    PoweredAscentStage2,
    OrbitReached,
}

fn transition(current: &FlightPhase, velocity: f32, accel: f32) -> FlightPhase {
    match current {
        FlightPhase::PreLaunch => {
            if accel > 2.0 { FlightPhase::PoweredAscentStage1}
            else {FlightPhase::PreLaunch}
        }

        FlightPhase::PoweredAscentStage1 => {
            if accel < 0.5 {FlightPhase::Meco}
            else {FlightPhase::PoweredAscentStage1}
        }

        FlightPhase::Meco => FlightPhase::StageSeperation, // seperating arms
        FlightPhase::StageSeperation => FlightPhase::PoweredAscentStage2,

        FlightPhase::PoweredAscentStage2 => {
            if velocity > 7800.0 { FlightPhase::OrbitReached}
            else {FlightPhase::PoweredAscentStage2}
        }

        FlightPhase::OrbitReached => FlightPhase::OrbitReached,
    }
}

struct Rocket {
    altitude: f32,
    velocity: f32,
    acceleration: f32,
    mass: f32,
}

fn main () {
    // creating an instance of the struct
    let mut my_rocket = Rocket {
        altitude : 0.00,
        velocity : 0.00,
        acceleration : 0.00,
        mass : 500.0,
    };

    let mut current_phase = FlightPhase::PreLaunch;

    let dt = 0.1;

    for movement in 0..30000 {

        let mut thrust = 0.0;

        // The engine fires if we are in PreLaunch (at T+10) OR if we are in an ascent phase
        if current_phase == FlightPhase::PreLaunch && movement >= 10 {
            thrust = 8000.0;
        } else if current_phase == FlightPhase::PoweredAscentStage1 && movement < 5000 {
            thrust = 8000.0;
        } else if current_phase == FlightPhase::PoweredAscentStage2 && movement < 25000 {
            thrust = 6500.0;
        }

        if my_rocket.altitude <= 0.0 && thrust < (my_rocket.mass * 9.81) {
            my_rocket.acceleration = 0.0;
        } else {
        my_rocket.acceleration = (thrust / my_rocket.mass) - 9.81;
        }

        // Altitude = current altitude + (velocity * dt) + (0.5 * accel * dt * dt)
        let displacement = (my_rocket.velocity * dt) + (0.5 * my_rocket.acceleration * dt * dt);
        my_rocket.altitude += displacement; // adding to current altitude

        my_rocket.velocity += my_rocket.acceleration * dt;

        let next_phase = transition(&current_phase, my_rocket.velocity, my_rocket.acceleration);
        if next_phase != current_phase{
            println!("Altitude: {} | Velocity: {} | Phase: {:?} -> {:?}", my_rocket.altitude, my_rocket.velocity, current_phase, next_phase);
        }  

        current_phase = next_phase;
    }
}

// TODO: Handle noises
