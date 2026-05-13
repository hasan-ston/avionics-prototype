use std::collections::VecDeque;

pub struct WeightedMovingAverage { // employing encapsulation
    buffer: VecDeque<f32>,
}

impl WeightedMovingAverage {
    pub fn new(size: usize) -> Self {
        Self {
            buffer: VecDeque::from(vec![0.0; size]),
        }
    }

    pub fn feed(&mut self, sample: f32) -> f32 {
        self.buffer.pop_front();
        self.buffer.push_back(sample);

        let mut weighted_sum = 0.0;
        let mut weighted_total = 0.0;

        for (index, value) in self.buffer.iter().enumerate() {
            let weight = (index + 1) as f32;
            weighted_sum += value * weight;
            weighted_total += weight;
        }
        
        weighted_sum / weighted_total
    }
}