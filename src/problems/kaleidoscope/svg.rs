use crate::solver::ExactCover;

use super::pieces::{KaleidoscopeBoard, KaleidoscopeProblem, Square, SquareColour};

use svg::Document;
use svg::node::element::Rectangle;
use svg::node::element::path::Data;

const SQUARE_WIDTH: f64 = 1.0;
const SOLUTION_BORDER_STROKE_WIDTH: f64 = 8.0/1.5;

impl KaleidoscopeBoard {
    
}

enum LineDirection { Horizontal, Vertical }

impl KaleidoscopeProblem {
    fn to_svg(&self, cover: ExactCover) -> Document {
        Document::new()
            .set("viewBox", (0, 0, (SQUARE_WIDTH*8.0), (SQUARE_WIDTH*8.0)))
            .add()
    }

    fn svg_square(&self, sq: &Square, color: SquareColour, faint: bool) -> Rectangle {
        Rectangle::new()
            .set("x", SQUARE_WIDTH*sq.x() as f64)
            .set("y", SQUARE_WIDTH*sq.y() as f64)
            .set("width", SQUARE_WIDTH)
            .set("height", SQUARE_WIDTH)
            .set("fill", Self::svg_square_color(color, faint))
    }

    fn svg_square_color(color: SquareColour, faint: bool) -> String {
        let mut s = match color {
            SquareColour::Black => "#111111",
            SquareColour::Red => "#EF1E02",
            SquareColour::Blue => "#3366FF",
            SquareColour::Yellow => "#EFD300",
        }.to_string();
        if faint {
            s.push_str("E6");
        }
        s
    }

    fn svg_line_between(sq: &Square, dir: LineDirection) {
        
    }
}