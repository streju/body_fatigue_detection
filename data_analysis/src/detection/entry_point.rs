use super::shoulders_analyzer::ShouldersAnalyzer;
use super::types::ShouldersCoordinatesInput;
use crate::alerts_reporter::AlertsReporter;
use std::sync::Arc;
pub struct DetectionEntryPoint {
    shoulders_analyzer: ShouldersAnalyzer,
}

impl DetectionEntryPoint {
    pub fn new(alerts_reporter: &Arc<AlertsReporter>) -> Arc<Self> {
        Arc::new(DetectionEntryPoint {
            shoulders_analyzer: ShouldersAnalyzer::new(&alerts_reporter),
        })
    }

    pub async fn start_shoulders_analysis(
        self: &Self,
        shoulders_coordinates: ShouldersCoordinatesInput,
    ) {
        // TODO: consider spawn and drop new tokio runtime task here
        self.shoulders_analyzer.analyze(shoulders_coordinates).await;
    }
}
