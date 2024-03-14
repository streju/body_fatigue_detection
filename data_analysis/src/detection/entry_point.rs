use super::shoulders_analyzer::ShouldersAnalyzer;
use super::types::ShouldersCoordinatesInput;
use crate::middleware::types::VisualizationClient;
use std::sync::Arc;
pub struct DetectionEntryPoint {
    workers: rayon::ThreadPool,
    visualization_client: Arc<dyn VisualizationClient>,
    shoulders_analyzer: Arc<ShouldersAnalyzer>,
}

impl DetectionEntryPoint {
    pub fn new(workers_nr: usize, visualization: &Arc<dyn VisualizationClient>) -> Arc<Self> {
        Arc::new(DetectionEntryPoint {
            workers: rayon::ThreadPoolBuilder::new()
                .num_threads(workers_nr)
                .build()
                .unwrap(),
            visualization_client: Arc::clone(visualization),
            shoulders_analyzer: Arc::new(ShouldersAnalyzer {}),
        })
    }

    pub fn start_shoulders_analysis(self: &Self, shoulders_coordinates: ShouldersCoordinatesInput) {
        // start workers
        self.workers.install(|| {
            self.shoulders_analyzer
                .as_ref()
                .analyze(shoulders_coordinates)
        })
    }
}
