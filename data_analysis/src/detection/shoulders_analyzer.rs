use super::types::ShouldersCoordinatesInput;
use super::utils::avg_calculator::AverageCalculator;
use crate::alerts_reporter::{AlertType, AlertsReporter};
use std::sync::Arc;

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
            ((input.left_shoulder.unwrap().y - input.right_shoulder.unwrap().y).abs()) as f64;

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
        if input.left_shoulder.is_none() {
            println!("[ShouldersAnalyzer] Left shoulder not detected!");
            return false;
        } else if input.right_shoulder.is_none() {
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
