use crate::detection::utils::timer::Timer;
use std::sync::{Arc, Mutex, Once};
use std::time::Duration;
pub struct EyesCalibration {
    is_calibration: Mutex<bool>,
    calibration_duration_sec: u64,
    once: Once,
    timer: Mutex<Option<Arc<Timer>>>,
}

impl EyesCalibration {
    pub fn new() -> Arc<Self> {
        Arc::new(EyesCalibration {
            is_calibration: Mutex::new(false),
            calibration_duration_sec: 5,
            once: Once::new(),
            timer: Mutex::new(None),
        })
    }

    pub fn is_calibration(self: &Arc<Self>) -> bool {
        self.once.call_once(|| {
            let mut is_calibration = self.is_calibration.lock().unwrap();
            *is_calibration = true;
            let self_clone = self.clone();
            let mut timer_opt = self.timer.lock().unwrap();
            *timer_opt = Some(Timer::new(
                Duration::from_secs(self.calibration_duration_sec),
                Arc::new(move || {
                    let mut is_calibration = self_clone.is_calibration.lock().unwrap();
                    *is_calibration = false;
                }),
            ));
            if let Some(timer) = &*timer_opt {
                Timer::start(timer);
            }
        });
        *self.is_calibration.lock().unwrap()
    }
}
