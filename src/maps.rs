use super::{lseg::LSeg, vec2::Vec2};

use super::map::Map;

pub static MAPS: [&'static [LSeg]; 2] = [
    &[
        LSeg::new_xy(0.0, 450.0, 1200.0, 450.0),
        LSeg::new_xy(1200.0, 450.0, 1600.0, 0.0),
    ],
    &[LSeg::new_xy(400.0, 600.0, 1200.0, 600.0)],
];
