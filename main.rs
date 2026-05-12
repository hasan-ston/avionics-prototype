use rand::distributions::{Distribution, Uniform};
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
enum FlightPhase {
    PreLaunch,
    PoweredAscentStage1,
    Meco,
    StageSeparation,
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

        FlightPhase::Meco => FlightPhase::StageSeparation, // seperating arms
        FlightPhase::StageSeparation => FlightPhase::PoweredAscentStage2,

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
    let mut rng = rand::thread_rng();
    let vibrations = Uniform::new_inclusive(-2.0,2.0);
    let mut accel_buffer: VecDeque<f32> = VecDeque::from(vec![0.0; 10]);

    for movement in 0..100000 {

        let noise = vibrations.sample(&mut rng);
        let noisy_accl = my_rocket.acceleration + noise;
        let mut thrust = 0.0;

        accel_buffer.pop_front();
        accel_buffer.push_back(noisy_accl);

        let mut weighted_sum = 0.0;
        let mut weighted_total = 0.0;

        for (index, value) in accel_buffer.iter().enumerate() {
            let weight = (index + 1) as f32;
            weighted_sum += value * weight;
            weighted_total += weight;
        }
        let filtered_accl = weighted_sum/weighted_total;

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

        let next_phase = transition(&current_phase, my_rocket.velocity, filtered_accl);
        if movement % 10 == 0 {
            println!("T: {:>4.1}s | Raw Accel: {:>6.2} | Filtered: {:>6.2}", movement as f32 * dt, noisy_accl, filtered_accl);
        }
        if next_phase != current_phase{
            println!("Altitude: {} | Velocity: {} | Phase: {:?} -> {:?}", my_rocket.altitude, my_rocket.velocity, current_phase, next_phase);
        }  
        current_phase = next_phase;
    }
}
