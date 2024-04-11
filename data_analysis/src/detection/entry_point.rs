use super::eyes::eyes_analyzer::EyesAnalyzer;
use super::shoulders_analyzer::ShouldersAnalyzer;
use super::types::{EyesInput, ShouldersCoordinatesInput};
use crate::alerts_reporter::AlertsReporter;
use crate::middleware::body_info::BodyInfo;
use std::sync::Arc;

pub struct DetectionEntryPoint {
    shoulders_analyzer: ShouldersAnalyzer,
    eyes_analyzer: EyesAnalyzer,
}

impl DetectionEntryPoint {
    pub fn new(alerts_reporter: &Arc<AlertsReporter>, body_info: &Arc<BodyInfo>) -> Arc<Self> {
        Arc::new(DetectionEntryPoint {
            shoulders_analyzer: ShouldersAnalyzer::new(&alerts_reporter),
            eyes_analyzer: EyesAnalyzer::new(&body_info),
        })
    }

    pub async fn start_shoulders_analysis(
        self: &Self,
        shoulders_coordinates: ShouldersCoordinatesInput,
    ) {
        // TODO: consider spawn and drop new tokio runtime task here
        self.shoulders_analyzer.analyze(shoulders_coordinates).await;
    }

    pub async fn start_eyes_analysis(self: &Self, eyes: EyesInput) {
        // TODO: consider spawn and drop new tokio runtime task here
        self.eyes_analyzer.analyze(eyes).await;
    }
}
