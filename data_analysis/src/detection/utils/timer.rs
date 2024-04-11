use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

type TimerArc = Arc<Timer>;

/// Passed function callback should be thread_safe, i.e. implements Send and Sync traits
pub fn create_timer_callback<F>(cb: F) -> Arc<dyn Fn() + Send + Sync>
where
    F: Fn() + Send + Sync + 'static,
{
    Arc::new(cb)
}

pub struct Timer {
    duration: Duration,
    callback: Mutex<Option<Arc<dyn Fn() + Send + Sync>>>,
    expired: Mutex<bool>,
}

impl Timer {
    pub fn new(duration: Duration, cb: Arc<dyn Fn() + Send + Sync>) -> TimerArc {
        Arc::new(Timer {
            duration: duration,
            callback: Mutex::new(Some(Arc::clone(&cb))),
            expired: Mutex::new(false),
        })
    }

    pub fn start(timer_instance: &TimerArc) {
        let timer_clone = Arc::clone(&timer_instance);
        tokio::task::spawn(async move {
            sleep(timer_clone.duration).await;
            let mut cb_opt = timer_clone.callback.lock().unwrap();
            let cb = cb_opt.clone().unwrap();
            let mut expired = timer_clone.expired.lock().unwrap();
            *expired = true;
            *cb_opt = None;

            (cb)();
        });
    }

    pub fn is_expired(self: &Self) -> bool {
        *self.expired.lock().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Dummy {
        val: Mutex<bool>,
        timer: Mutex<Option<Arc<Timer>>>,
    }

    impl Dummy {
        fn start_timer(self: &Arc<Self>) {
            let mut timer_opt = self.timer.lock().unwrap();
            let self_clone = self.clone();
            *timer_opt = Some(Timer::new(
                Duration::from_secs(2),
                Arc::new(move || {
                    self_clone.react_to_expiration();
                }),
            ));

            if let Some(timer) = &*timer_opt {
                Timer::start(timer);
            }
        }

        fn react_to_expiration(self: &Arc<Self>) {
            let mut v = self.val.lock().unwrap();
            *v = true;
        }

        fn drop_timer(self: &Arc<Self>) {
            let mut timer_opt = self.timer.lock().unwrap();
            *timer_opt = None;
        }
    }
    async fn wait_for_timer_expiration(timer: &TimerArc) {
        while !timer.is_expired() {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    #[tokio::test]
    async fn basic_closure_timer_callback() {
        let val = Arc::new(Mutex::new(33));
        let val_clone = val.clone();
        let to_do = move || {
            let mut v = val_clone.lock().unwrap();
            *v = 44;
        };
        let timer = Timer::new(Duration::from_secs(3), create_timer_callback(to_do));
        Timer::start(&timer);
        assert_eq!(*val.lock().unwrap(), 33);
        assert_eq!(timer.is_expired(), false);

        wait_for_timer_expiration(&timer).await;

        assert_eq!(timer.is_expired(), true);
        assert_eq!(*val.lock().unwrap(), 44);
    }

    #[tokio::test]
    async fn struct_that_use_timer() {
        let dummy = Arc::new(Dummy {
            val: Mutex::new(false),
            timer: Mutex::new(None),
        });

        dummy.start_timer();
        {
            let timer_opt = dummy.timer.lock().unwrap();
            if let Some(timer) = &*timer_opt {
                wait_for_timer_expiration(&timer).await;
                assert_eq!(timer.is_expired(), true);
                assert_eq!(*dummy.val.lock().unwrap(), true);
            } else {
                panic!("Cannot get timer, test failed!");
            }
        }
        dummy.drop_timer();
    }
}
