use clap::Parser;
use cli::Cli;
use complex::Complex;
use nannou::{prelude::*, rand::seq::SliceRandom};

mod cli;
mod complex;
mod oscillator;

use oscillator::Oscillator;

const MAX_UPDATE_DISTANCE: f32 = 0.01;
const UPDATE_RATE: f32 = 0.05;

fn main() {
    nannou::app(model)
        .update(update)
        .event(event)
        .simple_window(view)
        .run();
}

struct Model {
    draw_elements: Vec<cli::DrawElement>,
    nodes: NodeManager,
    oscillators: Vec<Oscillator>,
    draw_path: Vec<Vec2>,
}

#[derive(Debug, Clone)]
pub enum NodeManager {
    Interactive(Vec<Complex>),
    Static(Vec<Complex>),
}

impl NodeManager {
    fn handle_click(&mut self, pos: Vec2) {
        match self {
            NodeManager::Interactive(nodes) => {
                nodes.push(pos.into());
            }
            NodeManager::Static(_) => {}
        }
    }
    fn pop(&mut self) {
        match self {
            NodeManager::Interactive(nodes) => {
                nodes.pop();
            }
            NodeManager::Static(_) => {}
        }
    }
}

impl IntoIterator for NodeManager {
    type IntoIter = <Vec<Complex> as IntoIterator>::IntoIter;
    type Item = Complex;

    fn into_iter(self) -> std::vec::IntoIter<Self::Item> {
        match self {
            NodeManager::Interactive(nodes) => nodes.into_iter(),
            NodeManager::Static(nodes) => nodes.into_iter(),
        }
    }
}

impl NodeManager {
    fn new_interactive(nodes: Vec<Complex>) -> Self {
        NodeManager::Interactive(nodes)
    }

    fn new_static(nodes: Vec<Complex>) -> Self {
        NodeManager::Static(nodes)
    }
    fn len(&self) -> usize {
        match self {
            NodeManager::Interactive(nodes) => nodes.len(),
            NodeManager::Static(nodes) => nodes.len(),
        }
    }
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn vec(&self) -> &Vec<Complex> {
        match self {
            NodeManager::Interactive(nodes) => nodes,
            NodeManager::Static(nodes) => nodes,
        }
    }
}



impl Model {
    fn recalculate(&mut self) {
        println!("Recalculating oscillators from {} nodes", self.nodes.len());
        // let mean = self.nodes.iter().fold(Complex::zero(), |acc, &x| acc + x)/ self.nodes.len() as f32;
        // self.nodes.iter_mut().for_each(|node| *node = *node - mean);
        self.oscillators = oscillator::oscillators_from_complex(self.nodes.vec());
        self.draw_path.clear();
        println!("Oscillators: {}\n", self.oscillators.len());
    }
}

fn model(_app: &App) -> Model {
    let c = Cli::parse();
    let draw_elements = c.draw_elements().clone();
    let points = match c.file() {
        Some(file) => {
            println!("Loading nodes from file: {}", file);
            complex::load_from_file(file).expect("Failed to load nodes from file")
        }
        None => {
            println!("Starting with no nodes, click to add");
            vec![]
        }
    };
    let nodes = if c.interactive() {
        NodeManager::new_interactive(points)
    } else {
        NodeManager::new_static(points)
    };
    let mut model = Model {
        draw_elements,
        nodes,
        oscillators: vec![],
        draw_path: vec![],
    };
    model.recalculate();
    println!("Model initialized with {} nodes", model.nodes.len());
    model
}

fn osc_list_to_final_point(oscillators: &Vec<Oscillator>) -> Vec2 {
    let mut root = Vec2::new(0.0, 0.0);
    for osc in oscillators {
        root += osc.as_vector();
    }
    root
}

fn path_to_length(path: &Vec<Vec2>) -> f32 {
    let mut length = 0.0;
    for i in 1..path.len() {
        length += path[i - 1].distance(path[i]);
    }
    length
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for _ in 0..(UPDATE_RATE / MAX_UPDATE_DISTANCE) as i32 {
        for oscillator in &mut model.oscillators {
            oscillator.update(MAX_UPDATE_DISTANCE);
        }

        for oscillator in &mut model.oscillators {
            oscillator.update(0.01);
        }
        model
            .draw_path
            .push(osc_list_to_final_point(&model.oscillators));
        if path_to_length(&model.draw_path) > 600.0 {
            model.draw_path.remove(0);
        }
    }
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        // Handle mouse release events
        Event::WindowEvent {
            id: _,
            simple: Some(WindowEvent::MouseReleased(MouseButton::Left)),
        } => {
            println!("Adding new node: {:?}", (app.mouse.x, app.mouse.y));
            model.nodes.handle_click(Vec2::new(app.mouse.x, app.mouse.y).into());
            model.recalculate();
            println!("Current nodes: {:?}", model.nodes);
        }
        Event::WindowEvent {
            id: _,
            simple: Some(WindowEvent::MouseReleased(MouseButton::Right)),
        } => {
            println!("Removing last node");
            model.nodes.pop();
            model.recalculate();
            println!("Current nodes: {:?}", model.nodes);
        }
        _ => (),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(rgb(0.1, 0.1, 0.1));

    if model.draw_elements.contains(&cli::DrawElement::Nodes) {
        for node in model.nodes.clone() {
            draw.ellipse()
                .x_y(node.re(), node.im())
                .radius(5.0)
                .stroke_weight(1.0)
                .no_fill()
                .color(BLACK);
        }
    }
    if model.nodes.len() < 2 {
        draw.text("Click to add nodes (right click to remove last node)")
            .color(WHITE)
            .font_size(20)
            .x_y(0.0, 200.0);
        draw.to_frame(app, &frame).unwrap();
        return;
    } else {
        let mut root = model.oscillators[0].as_vector();
        for osc in &model.oscillators[1..] {
            if model.draw_elements.contains(&cli::DrawElement::Oscillators) {
                draw.ellipse()
                    .xy(root)
                    .radius(osc.amplitude())
                    .no_fill()
                    .stroke_weight(1.0)
                    .color(BLUE);
            }
            let next = root + osc.as_vector();
            if model.draw_elements.contains(&cli::DrawElement::Arms) {
                draw.line().start(root).end(next).color(GRAY).weight(1.0);
            }
            root = next;
        }
        if model.draw_elements.contains(&cli::DrawElement::Tip) {
            draw.ellipse().xy(root).radius(2.0).color(RED);
        }

        if model.draw_elements.contains(&cli::DrawElement::Shape) {
            for (i, node) in model.nodes.vec().iter().enumerate() {
                let next_node = model.nodes.vec().get((i + 1) % model.nodes.len());
                if let Some(next) = next_node {
                    draw.line()
                        .start(node.into())
                        .end(next.into())
                        .color(GRAY)
                        .weight(1.0);
                }
            }
        }

        if model.draw_elements.contains(&cli::DrawElement::Path) {
            draw.polyline()
                .weight(2.0)
                .points_colored(model.draw_path.iter().enumerate().map(|(i, p)| {
                    let fade = 1.0 - (i as f32 / model.draw_path.len() as f32);
                    (*p, rgba(1.0, 0.0, 0.0, fade))
                }));
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
