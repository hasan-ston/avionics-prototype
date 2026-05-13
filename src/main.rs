mod state;
mod physics;
mod filters;

use rand::distributions::{Distribution, Uniform};
use state::{FlightPhase, transition};
use physics::Rocket;
use filters::WeightedMovingAverage;

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
    let vibrations = Uniform::new_inclusive(-2.0, 2.0);
    
    // Initializing our filter from the module
    let mut filter = WeightedMovingAverage::new(10);

    for movement in 0..100000 {

        let noise = vibrations.sample(&mut rng);
        let noisy_accl = my_rocket.acceleration + noise;
        let mut thrust = 0.0;

        // Using our modular filter
        let filtered_accl = filter.feed(noisy_accl);

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
        
        if next_phase != current_phase {
            println!("Altitude: {} | Velocity: {} | Phase: {:?} -> {:?}", my_rocket.altitude, my_rocket.velocity, current_phase, next_phase);
        }  
        current_phase = next_phase;
    }
}