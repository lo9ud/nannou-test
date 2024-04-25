pub mod bubble {

    struct Bubble {
        radius: f64,
        x: f64,
        y: f64,
    }

    impl Bubble {
        fn new(radius: f64, x: f64, y: f64) -> Bubble {
            Bubble { radius, x, y }
        }

        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }

        fn intersects(&self, other: &Bubble) -> bool {
            let distance = ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt();
            distance < self.radius + other.radius
        }

        fn collapse(&mut self, other: &Bubble) -> Bubble {
            let total_area = self.area() + other.area();
            let r = total_area / std::f64::consts::PI;
            let x = (self.x * self.area() + other.x * other.area()) / 2.0;
            let y = (self.y * self.area() + other.y * other.area()) / 2.0;
            Bubble::new(x, y, r)
        }

        fn draw(&self, draw: &nannou::draw::Draw) {
            draw.ellipse()
                .x_y(self.x, self.y)
                .radius(self.radius)
                .color(nannou::color::rgb(0.0, 0.0, 1.0));
        }
    }
}
