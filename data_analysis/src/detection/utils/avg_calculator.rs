use std::sync::Mutex;
struct SumToCount(f64, i32);
pub struct AverageCalculator {
    sum_to_count: Mutex<SumToCount>,
}

impl AverageCalculator {
    pub fn new() -> Self {
        AverageCalculator {
            sum_to_count: Mutex::new(SumToCount(0.0, 1)),
        }
    }

    pub fn calculate(self: &Self, new_value: f64) -> f64 {
        let mut sum_to_count = self.sum_to_count.lock().unwrap();
        sum_to_count.0 += (new_value - sum_to_count.0) / sum_to_count.1 as f64;
        sum_to_count.1 += 1;
        sum_to_count.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn average_calculator() {
        let calculator = AverageCalculator::new();

        assert_relative_eq!(calculator.calculate(3.0), 3.0);
        assert_relative_eq!(calculator.calculate(0.0), 1.5);
        assert_relative_eq!(calculator.calculate(6.0), 3.0);
        assert_relative_eq!(calculator.calculate(100.0), 27.25);
        assert_relative_eq!(calculator.calculate(100.0), 41.8);
        assert_relative_eq!(calculator.calculate(60.0), 44.83, epsilon = 0.01);
        assert_relative_eq!(calculator.calculate(1.0), 38.57, epsilon = 0.01);
    }
}
