use super::blink;
use super::calibration::EyesCalibration;
use super::eye_state::{Context, EyeState};
use crate::detection::types::{Eye, EyesInput};
use crate::detection::utils::avg_calculator::AverageCalculator;
use crate::middleware::body_info::BodyInfo;
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;

struct EyeStatus {
    context: Context,
    state: EyeState,
    avg_openess_calc: AverageCalculator,
}

struct EyeStates {
    left: EyeStatus,
    right: EyeStatus,
}

pub struct EyesAnalyzer {
    eye_states: TokioMutex<EyeStates>,
    blinking_counter: TokioMutex<u32>,
    calibration: Arc<EyesCalibration>,
    body_info: Arc<BodyInfo>,
}

impl EyesAnalyzer {
    pub fn new(body_info: &Arc<BodyInfo>) -> Self {
        let open_eye_threashold = 50; // TODO from config? anyway calibartion will change it
        let closed_eye_threashold = 30;
        EyesAnalyzer {
            eye_states: TokioMutex::new(EyeStates {
                left: EyeStatus {
                    context: Context::new(open_eye_threashold, closed_eye_threashold),
                    state: EyeState::new(),
                    avg_openess_calc: AverageCalculator::new(),
                },
                right: EyeStatus {
                    context: Context::new(open_eye_threashold, closed_eye_threashold),
                    state: EyeState::new(),
                    avg_openess_calc: AverageCalculator::new(),
                },
            }),
            blinking_counter: TokioMutex::new(0),
            calibration: EyesCalibration::new(),
            body_info: body_info.clone(),
        }
    }

    pub async fn analyze(self: &Self, input: EyesInput) {
        let mut eyes_context_to_state = self.eye_states.lock().await;

        let left_openess_opt = self.calculate_eye_openess(input.left_eye);
        let right_openess_opt = self.calculate_eye_openess(input.right_eye);

        if left_openess_opt.is_none() || right_openess_opt.is_none() {
            return;
        }

        let right_openess = right_openess_opt.unwrap();
        let left_openess = left_openess_opt.unwrap();

        if self.calibration(&mut eyes_context_to_state, right_openess, left_openess) {
            return;
        }

        let prev_right_state = eyes_context_to_state.right.state.clone();
        let prev_left_state = eyes_context_to_state.left.state.clone();
        self.update_eye_state(&mut eyes_context_to_state.right, right_openess);
        self.update_eye_state(&mut eyes_context_to_state.left, left_openess);
        self.blink_analysis(prev_right_state, prev_left_state, &eyes_context_to_state)
            .await;
        self.update_openess(&mut eyes_context_to_state, right_openess, left_openess);
    }

    fn calculate_eye_openess(self: &Self, eye_opt: Option<Eye>) -> Option<u32> {
        if eye_opt.is_some() {
            let eye = eye_opt.unwrap();
            if eye.upper_eyelid.is_some() && eye.lower_eyelid.is_some() {
                return Some(
                    (eye.upper_eyelid.unwrap().y - eye.lower_eyelid.unwrap().y).abs() as u32,
                );
            }
        }
        None
    }

    fn update_eye_state(self: &Self, context_to_eye_state: &mut EyeStatus, input_openess: u32) {
        context_to_eye_state.state = context_to_eye_state
            .state
            .transition(input_openess, &context_to_eye_state.context);
    }

    fn update_openess(
        self: &Self,
        eyes_states: &mut EyeStates,
        right_openess: u32,
        left_openess: u32,
    ) {
        eyes_states.right.context.update_prev_openess(right_openess);
        eyes_states.left.context.update_prev_openess(left_openess);
    }

    async fn blink_analysis(
        self: &Self,
        prev_right: EyeState,
        prev_left: EyeState,
        eyes_states: &EyeStates,
    ) {
        if blink::is_a_blink(
            prev_right,
            prev_left,
            eyes_states.right.state,
            eyes_states.left.state,
        ) {
            let mut counter = self.blinking_counter.lock().await;
            *counter += 1;
            println!("[EyesAnalyzer] Blinking counter: {}", counter);
            self.body_info.send_blinking_counter_update(*counter).await;
        }
    }

    fn calibration(
        self: &Self,
        eyes_states: &mut EyeStates,
        right_openess: u32,
        left_openess: u32,
    ) -> bool {
        if self.calibration.is_calibration() {
            let left_avg_openess = eyes_states
                .left
                .avg_openess_calc
                .calculate(left_openess as f64);
            let right_avg_openess = eyes_states
                .right
                .avg_openess_calc
                .calculate(right_openess as f64);

            println!(
                "[EyesAnalyzer] Average openess of left eye: {}",
                left_avg_openess
            );
            println!(
                "[EyesAnalyzer] Average openess of right eye: {}",
                right_avg_openess
            );

            eyes_states
                .right
                .context
                .update_thresholds(right_avg_openess);
            eyes_states.left.context.update_thresholds(left_avg_openess);

            return true;
        }
        false
    }
}
