pub struct ShoulderCoords {
    pub x: Option<i32>,
    pub y: Option<i32>,
}

pub struct ShouldersCoordinatesInput {
    pub left_shoulder: ShoulderCoords,
    pub right_shoulder: ShoulderCoords,
}
