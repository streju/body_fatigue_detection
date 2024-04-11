use super::eye_state::EyeState;

pub fn is_a_blink(
    prev_right: EyeState,
    prev_left: EyeState,
    curr_right: EyeState,
    curr_left: EyeState,
) -> bool {
    let is_an_eye_blink = |prev, curr| -> bool { prev != curr && curr == EyeState::Closed };

    let remains_closed = |prev, curr| -> bool { prev == curr && curr == EyeState::Closed };

    let left_blink = is_an_eye_blink(prev_left, curr_left);
    let left_stays_closed = remains_closed(prev_left, curr_left);
    let right_blink = is_an_eye_blink(prev_right, curr_right);
    let right_stays_closed = remains_closed(prev_right, curr_right);

    (left_blink && right_blink)
        || (left_blink && right_stays_closed)
        || (left_stays_closed && right_blink)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn both_eyes_blinking() {
        let prev_right = EyeState::Closing;
        let prev_left = EyeState::Open;
        let curr_right = EyeState::Closed;
        let curr_left = EyeState::Closed;

        assert_eq!(
            is_a_blink(prev_right, prev_left, curr_right, curr_left),
            true
        );
    }

    #[test]
    fn left_eye_blink_right_remains_closed() {
        let prev_right = EyeState::Closed;
        let prev_left = EyeState::Opening;
        let curr_right = EyeState::Closed;
        let curr_left = EyeState::Closed;

        assert_eq!(
            is_a_blink(prev_right, prev_left, curr_right, curr_left),
            true
        );
    }

    #[test]
    fn right_eye_blink_left_remains_closed() {
        let prev_right = EyeState::Closing;
        let prev_left = EyeState::Closed;
        let curr_right = EyeState::Closed;
        let curr_left = EyeState::Closed;

        assert_eq!(
            is_a_blink(prev_right, prev_left, curr_right, curr_left),
            true
        );
    }

    #[test]
    fn not_a_blink_cases() {
        let mut prev_right = EyeState::Open;
        let mut prev_left = EyeState::Open;
        let mut curr_right = EyeState::Open;
        let mut curr_left = EyeState::Open;
        assert_eq!(
            is_a_blink(prev_right, prev_left, curr_right, curr_left),
            false
        );

        prev_right = EyeState::Open;
        prev_left = EyeState::Open;
        curr_right = EyeState::Closing;
        curr_left = EyeState::Closing;
        assert_eq!(
            is_a_blink(prev_right, prev_left, curr_right, curr_left),
            false
        );

        prev_right = EyeState::Opening;
        prev_left = EyeState::Opening;
        curr_right = EyeState::Closing;
        curr_left = EyeState::Closing;
        assert_eq!(
            is_a_blink(prev_right, prev_left, curr_right, curr_left),
            false
        );

        prev_right = EyeState::Closed;
        prev_left = EyeState::Closed;
        curr_right = EyeState::Closed;
        curr_left = EyeState::Closed;
        assert_eq!(
            is_a_blink(prev_right, prev_left, curr_right, curr_left),
            false
        );

        prev_right = EyeState::Closed;
        prev_left = EyeState::Closed;
        curr_right = EyeState::Closing;
        curr_left = EyeState::Closing;
        assert_eq!(
            is_a_blink(prev_right, prev_left, curr_right, curr_left),
            false
        );

        prev_right = EyeState::Open;
        prev_left = EyeState::Closing;
        curr_right = EyeState::Closing;
        curr_left = EyeState::Closed;
        assert_eq!(
            is_a_blink(prev_right, prev_left, curr_right, curr_left),
            false
        );

        prev_right = EyeState::Closed;
        prev_left = EyeState::Open;
        curr_right = EyeState::Closed;
        curr_left = EyeState::Opening;
        assert_eq!(
            is_a_blink(prev_right, prev_left, curr_right, curr_left),
            false
        );
    }
}
