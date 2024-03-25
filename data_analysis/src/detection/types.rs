pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

pub struct ShouldersCoordinatesInput {
    pub left_shoulder: Option<Coordinates>,
    pub right_shoulder: Option<Coordinates>,
}

pub struct Iris {
    pub top: Option<Coordinates>,
    pub bottom: Option<Coordinates>,
    pub external: Option<Coordinates>,
    pub interior: Option<Coordinates>,
}

pub struct Eye {
    pub upper_eyelid: Option<Coordinates>,
    pub lower_eyelid: Option<Coordinates>,
    pub iris: Option<Iris>,
}

pub struct EyesInput {
    pub right_eye: Option<Eye>,
    pub left_eye: Option<Eye>,
}
