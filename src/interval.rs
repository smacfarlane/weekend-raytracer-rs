#[derive(Copy, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Interval {
            min: -f64::INFINITY,
            max: f64::INFINITY,
        }
    }
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    // TODO: Cleanup
    // pub fn contains(&self, x: f64) -> bool {
    //     self.min <= x && x <= self.max
    // }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    // TODO: Cleanup
    // pub fn clamp(&self, x: f64) -> f64 {
    //     if x < self.min {
    //         self.min
    //     } else if x > self.max {
    //         self.max
    //     } else {
    //         x
    //     }
    // }
}
