use super::types::ShouldersCoordinatesInput;
use crate::alerts_reporter::{AlertType, AlertsReporter};
use std::sync::{Arc, Mutex};

struct SumToCount(f64, i32);
struct AverageCalculator {
    sum_to_count: Mutex<SumToCount>,
}

impl AverageCalculator {
    fn new() -> Self {
        AverageCalculator {
            sum_to_count: Mutex::new(SumToCount(0.0, 1)),
        }
    }

    fn calculate(self: &Self, new_value: f64) -> f64 {
        let mut sum_to_count = self.sum_to_count.lock().unwrap();
        sum_to_count.0 += (new_value - sum_to_count.0) / sum_to_count.1 as f64;
        sum_to_count.1 += 1;
        sum_to_count.0
    }
}

pub struct ShouldersAnalyzer {
    alerts_reporter: Arc<AlertsReporter>,
    avg_calculator: AverageCalculator,
    base_threashold: f64, // TODO: remove when pre-calibration implemented
}

impl ShouldersAnalyzer {
    pub fn new(alerts_reporter: &Arc<AlertsReporter>) -> Self {
        ShouldersAnalyzer {
            alerts_reporter: alerts_reporter.clone(),
            avg_calculator: AverageCalculator::new(),
            base_threashold: 20.0,
        }
    }

    pub async fn analyze(self: &Self, input: ShouldersCoordinatesInput) {
        if !self.both_shoulders_detected(&input) {
            return;
        }

        let height_dff: f64 =
            ((input.left_shoulder.y.unwrap() - input.right_shoulder.y.unwrap()).abs()) as f64;

        let avg = self.avg_calculator.calculate(height_dff);
        println!("Shoulders avg height diff: {}", avg); // TODO: should be avg from calibration with correct pose

        let threshold: f64 = self.get_threshold(avg);

        if height_dff > threshold {
            self.alerts_reporter
                .start_alert(AlertType::ShoulderPose)
                .await
        } else {
            self.alerts_reporter
                .stop_alert(AlertType::ShoulderPose)
                .await
        }
    }

    fn both_shoulders_detected(self: &Self, input: &ShouldersCoordinatesInput) -> bool {
        if input.left_shoulder.x == None || input.left_shoulder.y == None {
            println!("[ShouldersAnalyzer] Left shoulder not detected!");
            return false;
        } else if input.right_shoulder.x == None || input.right_shoulder.y == None {
            println!("[ShouldersAnalyzer] Right shoulder not detected!");
            return false;
        }
        println!("[ShouldersAnalyzer] Both shoulders detected. Going to analyze.");
        true
    }

    // TODO: replace by calibration result
    fn get_threshold(self: &Self, avg: f64) -> f64 {
        if avg < self.base_threashold {
            return self.base_threashold - (avg / 2.0);
        }
        self.base_threashold + (avg / 2.0)
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
