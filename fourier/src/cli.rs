use std::str::FromStr;

use clap::{builder::OsStr, Parser};

#[derive(Parser, Debug)]
pub struct Cli {
    /// The path to the input file containing points
    #[clap(short, long)]
    file: Option<String>,

    /// The example mode to run
    #[clap(short, long, default_value = "default")]
    example: Option<String>,

    /// Run in interactive mode
    #[clap(short, long)]
    interactive: bool,

    /// The elements to draw
    #[clap(short, long, default_value = "all", action = clap::ArgAction::Append, default_values_t = vec![
        DrawElement::Nodes,
        DrawElement::Arms,
        DrawElement::Path,
    ])]
    draw: Vec<DrawElement>,
}

impl Cli {
    pub fn new() -> Self {
        Cli::parse()
    }

    pub fn file(&self) -> Option<&String> {
        self.file.as_ref()
    }

    pub fn example(&self) -> Option<&String> {
        self.example.as_ref()
    }

    pub fn interactive(&self) -> bool {
        self.interactive
    }

    pub fn draw_elements(&self) -> &Vec<DrawElement> {
        &self.draw
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawElement {
    Nodes,
    Shape,
    Oscillators,
    Arms,
    Tip,
    Path,
}

impl FromStr for DrawElement {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "nodes" => Ok(DrawElement::Nodes),
            "shape" => Ok(DrawElement::Shape),
            "oscillators" => Ok(DrawElement::Oscillators),
            "arms" => Ok(DrawElement::Arms),
            "tip" => Ok(DrawElement::Tip),
            "path" => Ok(DrawElement::Path),
            _ => Err(format!("Unknown draw element: {}", s)),
        }
    }
}

impl std::fmt::Display for DrawElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DrawElement::Nodes => write!(f, "nodes"),
            DrawElement::Shape => write!(f, "shape"),
            DrawElement::Oscillators => write!(f, "oscillators"),
            DrawElement::Arms => write!(f, "arms"),
            DrawElement::Tip => write!(f, "tip"),
            DrawElement::Path => write!(f, "path"),
        }
    }
}