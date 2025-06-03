use nannou::{prelude::{
    vec2, Vec2
}, rand::rand};

use crate::map::{Map, Resource};

#[derive(Debug, Clone)]
enum Intention {
    Move(Vec2),
    Deposit,
    Harvest,
}

#[derive(Debug, Clone)]
pub struct Ant {
    position: Vec2,
    carrying: Option<Resource>,
    intention: Option<Intention>,
}

impl Ant {
    pub fn new(position: Vec2) -> Self {
        Ant {
            position,
            carrying: None,
            intention: None,
        }
    }

    /// Calculate the intention of the ant
    pub fn update(&mut self, map: &Map) {
        let tile = map.get_tile(self.position);
        match tile.peek() {
            Some(Resource::Food) => {
                self.intention = Some(Intention::Harvest);
            }
            None => {
                let neighbours = map.get_neighbourhood(self.position, 3);
                let mut max = 0;
                let mut max_pos = vec2(0.0, 0.0);
                for (pos, tile) in neighbours.iter() {
                    if tile.peek() == Some(Resource::Food) && tile.get_remaining() > max {
                        max = tile.get_remaining();
                        max_pos = *pos;
                    }
                }
                if max > 0 {
                    self.intention = Some(Intention::Move(max_pos));
                }else {
                    self.intention = Some(Intention::Move(neighbours[rand::random::<usize>() % neighbours.len()].0));
                }

            }
        }
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }
}