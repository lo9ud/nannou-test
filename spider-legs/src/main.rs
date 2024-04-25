use nannou::prelude::*;
fn build_mst(nodes: &Vec<Node>) -> Vec<Edge> {
    // let mut edges = Vec::new();
    // for node1 in 0..nodes.len() {
    //     for node2 in 0..nodes.len() {
    //         if node1 == node2 {
    //             continue;
    //         }
    //         edges.push((node1, node2));
    //     }
    // }
    // edges.sort_by(|a, b| {
    //     let node1 = nodes[a.0];
    //     let node2 = nodes[a.1];
    //     let node3 = nodes[b.0];
    //     let node4 = nodes[b.1];
    //     let dist1 = node1.pos.distance(node2.pos);
    //     let dist2 = node3.pos.distance(node4.pos);
    //     dist1.partial_cmp(&dist2).unwrap()
    // });
    // let mut mst: Vec<Edge> = vec![];
    // let mut visited = vec![false; nodes.len()];
    // let start = random_range(0, nodes.len());
    // visited[start] = true;
    // while mst.len() < nodes.len() - 1 {
    //     let new_edge = edges
    //         .iter()
    //         .skip_while(|edge| !visited[edge.0] || visited[edge.1])
    //         .next()
    //         .unwrap();
    //     visited[new_edge.1] = true;
    //     mst.push(Edge {
    //         a: new_edge.0 as u16,
    //         b: new_edge.1 as u16,
    //     });
    // }
    // mst
    let mut edges = Vec::new();
    for node in nodes {
        match nodes
            .iter()
            .filter(|node2| node2.ring == node.ring + 1)
            .min_by_key(|a| ((a.pos.angle() - node.pos.angle()).abs() * 100.0) as u16)
        {
            Some(node) => {
                edges.push(Edge { a: *node, b: *node });
            }
            None => {}
        }
    }
    edges
}

fn main() {
    nannou::app(model)
        .update(update)
        .event(event)
        .simple_window(view)
        .run();
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Edge {
    a: Node,
    b: Node,
}
#[derive(Debug, Copy, Clone, PartialEq)]
struct Node {
    pos: Point2,
    direction: f32,
    ring: u16,
    sector: u16,
}

struct Model {
    nodes: Vec<Node>,
}

fn model(_app: &App) -> Model {
    let rings = 5;
    let sectors = 10;
    let mut nodes = Vec::new();
    for i in 0..rings {
        for k in 1..sectors+1 {
            nodes.push(Node {
                pos: pt2(40.0 + 100.0 * i as f32, 40.0 + 100.0 * i as f32)
                    .rotate(2.0 * PI / rings as f32 * (k + i) as f32),
                direction: (i % 2) as f32 * 2.0 - 1.0,
                ring: i,
                sector: k,
            });
        }
    }
    nodes.push(Node {
        pos: pt2(0.0, 0.0),
        direction: 0.0,
        ring: 0,
        sector: 0,
    });
    Model { nodes }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for node in model.nodes.iter_mut() {
        node.pos = node.pos.rotate(0.01 * node.direction);
    }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

// fn draw_arc(draw: &Draw, r: f32, theta0: f32, theta1: f32) {
//     let n = 100;
//     let dt = (theta1 - theta0) / n as f32;
//     let mut points = Vec::new();
//     for i in 0..n {
//         let theta = theta0 + i as f32 * dt;
//         points.push(r * pt2(theta.cos(), theta.sin()));
//     }
//     draw.polyline()
//         .stroke_weight(1.0)
//         .points(points)
//         .color(WHITE);
// }

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let mst = build_mst(&model.nodes);
    let edge = mst[0];
    println!("{:?} -> {:?}", edge.a.pos, edge.b.pos);
    let a = edge.a.pos.clone();
    let b = edge.b.pos.clone();
    draw.line()
        .start(a)
        .end(b)
        .stroke_weight(10.0)
        .color(rgb(1.0, 0.0, 1.0));
    // for node in model.nodes.iter() {
    //     draw.ellipse().xy(node.pos).radius(5.0).color(WHITE);
    // }
    // for edge in mst.iter() {
    //     let node1 = edge.a;
    //     let node2 = edge.b;
    //     let (inner_node, outer_node) = if node1.pos.length() < node2.pos.length() {
    //         (node1, node2)
    //     } else {
    //         (node2, node1)
    //     };
    //     let r = inner_node.pos.length()
    //         + (outer_node.pos.length() - inner_node.pos.length())
    //             * inner_node.sector.abs_diff(outer_node.sector) as f32
    //             / (inner_node.sector + outer_node.sector) as f32;
    //     draw.line()
    //         .start(inner_node.pos)
    //         .end(inner_node.pos.normalize() * r)
    //         .stroke_weight(1.0)
    //         .color(WHITE);
    //     let theta0 = inner_node.pos.angle();
    //     let theta1 = outer_node.pos.angle();
    //     draw_arc(&draw, r, theta0.min(theta1), theta1.max(theta0));
    //     draw.line()
    //         .start(
    //             inner_node
    //                 .pos
    //                 .rotate(outer_node.pos.angle() - inner_node.pos.angle())
    //                 .normalize()
    //                 * r,
    //         )
    //         .end(outer_node.pos)
    //         .stroke_weight(1.0)
    //         .color(WHITE);
    // }
    draw.to_frame(app, &frame).unwrap();
}
