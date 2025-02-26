use super::lseg::LSeg;

#[derive(Clone, Copy)]
pub enum FaceType {
    Floor,
    Wall,
    Ceiling,
}

pub static MAPS: &'static [&'static [(LSeg, FaceType)]] = &[
    &[
        (LSeg::new_xy(0.0, 450.0, 1200.0, 450.0), FaceType::Floor),
        (LSeg::new_xy(1200.0, 450.0, 1600.0, 200.0), FaceType::Floor),
    ],
    &[
        (LSeg::new_xy(0.0, 600.0, 1600.0, 600.0), FaceType::Floor),
        (LSeg::new_xy(0.0, 400.0, 400.0, 400.0), FaceType::Ceiling),
        (
            LSeg::new_xy(1200.0, 300.0, 1600.0, 400.0),
            FaceType::Ceiling,
        ),
    ],
    &[
        (LSeg::new_xy(400.0, 600.0, 1200.0, 600.0), FaceType::Floor),
        (LSeg::new_xy(400.0, 600.0, 400.0, 0.0), FaceType::Wall),
        (LSeg::new_xy(1200.0, 600.0, 1200.0, 0.0), FaceType::Wall),
    ],
    &[
        (LSeg::new_xy(0.0, 400.0, 800.0, 400.0), FaceType::Floor),
        (LSeg::new_xy(1000.0, 400.0, 1600.0, 400.0), FaceType::Floor),
        (LSeg::new_xy(800.0, 600.0, 800.0, 400.0), FaceType::Wall),
        (LSeg::new_xy(1000.0, 800.0, 1000.0, 400.0), FaceType::Wall),
        (LSeg::new_xy(0.0, 600.0, 800.0, 600.0), FaceType::Ceiling),
        (LSeg::new_xy(0.0, 800.0, 1000.0, 800.0), FaceType::Floor),
    ],
];
