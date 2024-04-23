#![warn(clippy::all, rust_2018_idioms)]

use egui::{Color32, layers::ShapeIdx, Painter, Pos2, Rect, Rounding, Stroke};

pub use app::TemplateApp;
use r_tree::{BoundingBox, RTree};

mod app;

struct BoundedPainter {
    painter: Painter,
    bounding_box: Rect,
}

impl BoundedPainter {
    fn new(painter: Painter, bounding_box: Rect) -> Self {
        BoundedPainter {
            painter,
            bounding_box,
        }
    }

    fn normalize(rect: Rect, bounding_box: &Rect) -> Rect {
        let (min, max) = (bounding_box.min.to_vec2(), bounding_box.max.to_vec2());
        let mut normalized = Rect::from_min_max(rect.min + min, rect.max + min);

        println!("Drawing {:?}", normalized);

        if !bounding_box.contains_rect(normalized) {
            todo!("BoundingBox too large to visualize");
        }

        normalized
    }

    fn rect(
        &self,
        rect: Rect,
        rounding: impl Into<Rounding>,
        stroke: impl Into<Stroke>,
    ) -> ShapeIdx {
        let rect = Self::normalize(rect, &self.bounding_box);
        self.painter.rect_stroke(rect, rounding, stroke)
    }
}

fn to_rect(bbox: &BoundingBox<2>) -> Rect {
    let min = Pos2::new(bbox.bounds[0].min() as f32, bbox.bounds[1].min() as f32);
    let max = Pos2::new(bbox.bounds[0].max() as f32, bbox.bounds[1].max() as f32);
    Rect::from_min_max(min, max)
}

// TODO: accept a BoundedPainter in parameters and paint using it
pub(crate) fn draw_tree<T>(
    painter: Painter,
    viz_bounding_box: Rect,
    tree: &RTree<T, 2>,
    highlighted_idx: Option<&BoundingBox<2>>,
) {
    let painter = BoundedPainter::new(painter, viz_bounding_box);

    for (bbox, _data) in tree.iter() {
        let rect = to_rect(bbox);
        match highlighted_idx {
            Some(hbox) if hbox == bbox => {
                painter.rect(rect, Rounding::ZERO, Stroke::new(1.0, Color32::GREEN))
            }
            _ => painter.rect(rect, Rounding::ZERO, Stroke::new(1.0, Color32::WHITE)),
        };
    }
}
