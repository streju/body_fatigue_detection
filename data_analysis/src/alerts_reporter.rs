use crate::middleware::translation::to_proto_alert;
use crate::middleware::types::VisualizationClient;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum AlertType {
    ShoulderPose,
}

pub struct AlertsReporter {
    visualization_client: Arc<Mutex<dyn VisualizationClient>>,
    alerts_state: Mutex<HashMap<AlertType, bool>>,
}

impl AlertsReporter {
    pub fn new(visualization: &Arc<Mutex<dyn VisualizationClient>>) -> Arc<Self> {
        Arc::new(AlertsReporter {
            visualization_client: visualization.clone(),
            alerts_state: Mutex::new(HashMap::from([(AlertType::ShoulderPose, false)])),
        })
    }

    pub async fn start_alert(self: &Self, alert_type: AlertType) {
        self.alert_handling(&alert_type, true).await
    }

    pub async fn stop_alert(self: &Self, alert_type: AlertType) {
        self.alert_handling(&alert_type, false).await
    }

    async fn alert_handling(self: &Self, alert_type: &AlertType, state: bool) {
        println!("[AlertsReporter] alert_handling");
        let mut alerts_state = self.alerts_state.lock().await;
        if *alerts_state.get(alert_type).unwrap() != state {
            self.visualization_client
                .lock()
                .await
                .send_alarm_info(to_proto_alert(alert_type, state))
                .await
                .unwrap();
            alerts_state.insert(*alert_type, state);
        }
    }
}
