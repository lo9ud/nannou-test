use nannou::prelude::{Vec2, vec2};
use bracket_noise::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Resource {
    Food
}

pub struct Tile {
    resource: Option<Resource>,
    resource_amount: usize,
}

impl Tile {
    pub fn new() -> Self {
        Tile { resource: None, resource_amount: 0 }
    }

    pub fn peek(&self) -> Option<Resource> {
        self.resource
    }

    pub fn deplete(&mut self) -> Option<Resource> {
        let inner = self.resource;
        self.resource_amount -= 1;
        if self.resource_amount == 0 {
            self.resource = None;
        }
        inner
    }

    pub fn deposit(&mut self, resource: Resource) {
        match self.resource {
            Some(r) => {
                if r == resource {
                    self.resource_amount += 1;
                } else {
                    unimplemented!("Cannot deposit different resources in the same tile");
                }
            }
            None => {
                self.resource = Some(resource);
                self.resource_amount += 1;
            }
        }
        self.resource = Some(resource);
        self.resource_amount += 1;
    }

    pub fn get_remaining(&self) -> usize {
        self.resource_amount
    }

    pub fn is_empty(&self) -> bool {
        self.resource_amount == 0
    }
}

pub struct Map {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let mut tiles = Vec::with_capacity(width * height);
        let mut noise = FastNoise::seeded(0);
        noise.set_noise_type(NoiseType::SimplexFractal);
        noise.set_fractal_octaves(4);
        noise.set_fractal_gain(0.5);
        for i in 0..width {
            for j in 0..height {
                let mut tile = Tile::new();
                let value = noise.get_noise(i as f32, j as f32);
                if value > 0.5 {
                    tile.deposit(Resource::Food);
                }
                tiles.push(tile);
            }
        }
        Map { tiles, width, height }
    }

    pub fn get_tile(&self, xy: Vec2) -> &Tile {
        let x = xy.x as usize;
        let y = xy.y as usize;
        &self.tiles[y * self.width + x]
    }

    pub fn get_tile_mut(&mut self, xy: Vec2) -> &mut Tile {
        let x = xy.x as usize;
        let y = xy.y as usize;
        &mut self.tiles[y * self.width + x]
    }

    pub fn get_neighbourhood(&self, xy: Vec2, r:usize) -> Vec<(Vec2, &Tile)> {
        let mut neighbourhood = Vec::new();
        let x = xy.x as usize;
        let y = xy.y as usize;
        for i in 0..2*r {
            for j in 0..2*r {
                let i = i as i32 - r as i32;
                let j = j as i32 - r as i32;
                if i.pow(2) + j.pow(2) <= r.pow(2) as i32 {
                    let x = x as i32 + i;
                    let y = y as i32 + j;
                    if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                        let p = vec2(x as f32, y as f32);
                        neighbourhood.push((p, self.get_tile(p)));
                    }
                }
            }
        }
        neighbourhood
    }

    pub fn tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }
}