pub struct Context {
    open_threashold: u32,
    closed_threashold: u32,
    prev_openess: u32,
}

impl Context {
    pub fn new(open_th: u32, closed_th: u32) -> Self {
        Context {
            open_threashold: open_th,
            closed_threashold: closed_th,
            prev_openess: u32::MAX,
        }
    }
    pub fn update_thresholds(&mut self, avg_openess: f64) {
        let correction = avg_openess * 0.1;
        self.update_open_threshold((avg_openess + correction * 2.0) as u32);
        self.update_closed_threshold((avg_openess - correction * 3.0) as u32);
    }
    fn update_open_threshold(&mut self, open_th: u32) {
        self.open_threashold = open_th;
    }
    fn update_closed_threshold(&mut self, closed_th: u32) {
        self.closed_threashold = closed_th;
    }
    pub fn update_prev_openess(&mut self, input: u32) {
        self.prev_openess = input;
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum EyeState {
    Open,
    Closed,
    Closing,
    Opening,
}

impl EyeState {
    pub fn new() -> Self {
        EyeState::Open
    }

    pub fn transition(&self, input: u32, context: &Context) -> Self {
        fn passed_below_open_threashold(input: u32, context: &Context) -> bool {
            input < context.open_threashold
        }
        fn passed_above_open_threashold(input: u32, context: &Context) -> bool {
            input > context.open_threashold
        }
        fn passed_below_closed_threashold(input: u32, context: &Context) -> bool {
            input < context.closed_threashold
        }
        fn passed_above_closed_threashold(input: u32, context: &Context) -> bool {
            input > context.closed_threashold
        }
        fn is_closing(input: u32, context: &Context) -> bool {
            input < context.prev_openess
        }

        match self {
            EyeState::Open => {
                if is_closing(input, context) && passed_below_open_threashold(input, context) {
                    if passed_below_closed_threashold(input, context) {
                        return EyeState::Closed;
                    }
                    return EyeState::Closing;
                }
                EyeState::Open
            }
            EyeState::Closed => {
                if !is_closing(input, context) {
                    if passed_above_closed_threashold(input, context) {
                        if passed_above_open_threashold(input, context) {
                            return EyeState::Open;
                        }
                        return EyeState::Opening;
                    }
                }
                EyeState::Closed
            }
            EyeState::Closing => {
                if is_closing(input, context) && passed_below_closed_threashold(input, context) {
                    return EyeState::Closed;
                } else if !is_closing(input, context) {
                    if passed_above_open_threashold(input, context) {
                        return EyeState::Open;
                    }
                    return EyeState::Opening;
                }
                EyeState::Closing
            }
            EyeState::Opening => {
                if !is_closing(input, context) && passed_above_open_threashold(input, context) {
                    return EyeState::Open;
                } else if is_closing(input, context) {
                    if passed_below_closed_threashold(input, context) {
                        return EyeState::Closed;
                    }
                    return EyeState::Closing;
                }
                EyeState::Opening
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn transition_and_update_prev(state: &mut EyeState, input: u32, context: &mut Context) {
        *state = state.transition(input, context);
        context.update_prev_openess(input);
    }
    #[test]
    fn start_from_open_to_closing() {
        let mut context = Context::new(30, 20);
        let mut state = EyeState::new();
        state = state.transition(25, &mut context);
        assert_eq!(state, EyeState::Closing);
    }
    #[test]
    fn start_from_open_to_closed() {
        let mut context = Context::new(30, 20);
        let mut state = EyeState::new();
        state = state.transition(19, &mut context);
        assert_eq!(state, EyeState::Closed);
    }
    #[test]
    fn start_state_stays_open() {
        let mut context = Context::new(30, 20);
        let mut state = EyeState::Open;
        state = state.transition(12312, &mut context);
        assert_eq!(state, EyeState::Open);
    }

    #[test]
    fn closing_to_opening_without_close() {
        let mut context = Context::new(30, 20);
        let mut state: EyeState = EyeState::new();
        transition_and_update_prev(&mut state, 25, &mut context);
        assert_eq!(state, EyeState::Closing);
        transition_and_update_prev(&mut state, 24, &mut context);
        assert_eq!(state, EyeState::Closing);
        transition_and_update_prev(&mut state, 25, &mut context);
        assert_eq!(state, EyeState::Opening);
    }

    #[test]
    fn close_to_open_without_closing() {
        let mut context = Context::new(30, 20);
        let mut state: EyeState = EyeState::new();
        transition_and_update_prev(&mut state, 25, &mut context);
        assert_eq!(state, EyeState::Closing);
        transition_and_update_prev(&mut state, 24, &mut context);
        assert_eq!(state, EyeState::Closing);
        transition_and_update_prev(&mut state, 25, &mut context);
        assert_eq!(state, EyeState::Opening);
    }
}
