use nannou::prelude::*;

#[derive(Debug, Clone)]
pub struct SlimePoint {
    pub position: Vec2,
    pub velocity: Vec2,
}

impl From<Vec2> for SlimePoint {
    fn from(v: Vec2) -> Self {
        Self {
            position: v,
            velocity: vec2(0.0, 0.0),
        }
    }
}

impl From<&Vec2> for SlimePoint {
    fn from(v: &Vec2) -> Self {
        Self {
            position: v.clone(),
            velocity: vec2(0.0, 0.0),
        }
    }
}

impl Into<Vec2> for SlimePoint {
    fn into(self) -> Vec2 {
        self.position
    }
}

#[derive(Debug, Clone)]
pub struct Slime {
    pub slime_props: SlimeProps,
    pub body: Vec<SlimePoint>,
    pub volume: f32,
}

#[derive(Debug, Clone)]
pub struct SlimeProps {
    pub cohesion: f32,
    pub min_cohesion_distance: f32,
    pub pressure: f32,
    pub color: Rgba,

    pub split_threshold: f32,
    pub split_chance: f32,
    pub split_force: f32,

    pub merge_threshold: f32,
    pub merge_chance: f32,
    pub merge_force: f32,

    pub vision_radius: f32,

    pub velocity_decay: f32,
}

impl Slime {
    pub fn blob(props: SlimeProps, n: usize, radius: f32, x: f32, y: f32) -> Self {
        let mut points: Vec<SlimePoint> = Vec::with_capacity(n);
        for i in 0..n {
            let angle = i as f32 * 2.0 * PI / n as f32;
            points.push(SlimePoint {
                position: vec2(x + angle.cos() * radius, y + angle.sin() * radius).into(),
                velocity: vec2(random_range(-1.0, 1.0), random_range(-1.0, 1.0)),
            });
        }
        Self {
            slime_props: props,
            body: points,
            volume: radius * radius,
        }
    }

    pub fn max_radius(&self) -> f32 {
        self.body
            .iter()
            .map(|point| (self.centroid() - point.position).length())
            .max_by(|a, b| a.total_cmp(b))
            .or(Some(0.0))
            .unwrap()
    }

    pub fn contains(&self, point: Vec2) -> bool {
        if (self.centroid() - point).length() < self.max_radius() * 1.1 {
            return true;
        }
        return false;
    }

    pub fn area(&self) -> f32 {
        let mut area = 0.0;
        for i in 0..self.body.len() {
            let j = (i + 1) % self.body.len();
            area += self.body[i].position.x * self.body[j].position.y;
            area -= self.body[j].position.x * self.body[i].position.y;
        }
        area / 2.0
    }

    fn volume(&self) -> f32 {
        self.volume
    }

    fn perimeter(&self) -> f32 {
        let mut perimeter = 0.0;
        for i in 0..self.body.len() {
            let j = (i + 1) % self.body.len();
            perimeter += (self.body[i].position - self.body[j].position).length();
        }
        perimeter
    }

    pub fn centroid(&self) -> Vec2 {
        let mut cx = 0.0;
        let mut cy = 0.0;
        let mut a = 0.0;
        for i in 0..self.body.len() {
            let j = (i + 1) % self.body.len();
            let cross = self.body[i].position.x * self.body[j].position.y
                - self.body[j].position.x * self.body[i].position.y;
            a += cross;
            cx += (self.body[i].position.x + self.body[j].position.x) * cross;
            cy += (self.body[i].position.y + self.body[j].position.y) * cross;
        }
        a *= 3.0;
        vec2(cx / a, cy / a)
    }

    pub fn core_radius(&self) -> f32 {
        self.volume().sqrt() / 6.0
    }

    pub fn get_targets(&self, slimes: &Vec<Slime>, mouse_pos: Vec2) -> Vec<Vec2> {
        return vec![mouse_pos];
    }

    pub fn _get_tension(&self, i: usize) -> Vec2 {
        let active = &self.body[i];

        let left = &self.body[((i + self.body.len() - 1) % self.body.len()) as usize];
        let right = &self.body[((i + 1) % self.body.len()) as usize];

        -((active.position - left.position) + (active.position - right.position))
            * self.slime_props.cohesion
            / 2.0
    }

    pub fn _get_pressure(&self, i: usize) -> Vec2 {
        (self.body[i].position - self.centroid()).normalize() * self.volume() / self.perimeter()
    }

    pub fn _get_target_force(&self, i: usize, target: Vec2) -> Vec2 {
        if let Some(active) = self.body.get(i) {
            let target_dist = (active.position - target).length();

            (target - active.position).normalize() / target_dist.sqrt() * 100.0
        } else {
            vec2(0.0, 0.0)
        }
    }

    pub fn update(&mut self, dt: f32, slimes: &Vec<Slime>, mouse_pos: Vec2) {
        let centroid = self.centroid();
        let core_radius = self.core_radius();
        // Update positions based on old velocities
        for point in self.body.iter_mut() {
            point.position += dt * point.velocity;
        }

        for point in self.body.iter_mut() {
            if (point.position - centroid).length() < core_radius {
                point.position = (point.position - centroid).normalize() * (core_radius * 1.15);
            }
        }

        let mut new_points = vec![];
        // Update velocities based on cohesion and adhesion
        for (i, point) in self.body.iter().enumerate() {
            let mut active = point.clone();
            active.velocity *= 1.0 - self.slime_props.velocity_decay;

            active.velocity += self._get_tension(i) * self.slime_props.cohesion;

            active.velocity += self._get_pressure(i) * self.slime_props.pressure;

            for target in self.get_targets(slimes, mouse_pos) {
                if self.contains(target) {
                    continue;
                }
                active.velocity += self._get_target_force(i, target);
            }

            active.velocity = active.velocity * dt + point.velocity * (1.0 - dt);
            new_points.push(active);
        }
        self.body = new_points;
    }

    pub fn draw(&self, draw: &Draw) {
        draw.polygon()
            .stroke(self.slime_props.color)
            .stroke_weight(1.0)
            .color(self.slime_props.color)
            .points(self.body.iter().cloned());
    }
}
