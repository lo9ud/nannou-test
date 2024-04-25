use nannou::{geom::Point2, App};

pub struct Model {
    pub update_count: i64,
    pub last_click: Option<(Point2, std::time::Instant)>,
}

pub fn model(_app: &App) -> Model {
    _app.main_window().set_resizable(false);
    Model {
        update_count: 0,
        last_click: None,
    }
}