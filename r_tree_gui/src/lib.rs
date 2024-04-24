#![warn(clippy::all, rust_2018_idioms)]

pub use app::TemplateApp;
use egui::{layers::ShapeIdx, Color32, Painter, Pos2, Rect, Rounding, Stroke};
use r_tree::{BoundingBox, RTree};
use std::{collections::HashSet, fmt::Display};

mod app;

struct BoundedPainter {
    painter: Painter,
    bounding_box: Rect,
}

const STROKE_WIDTH: f32 = 2.;
const HIGHLIGHTED_STROKE_WIDTH: f32 = 2. * STROKE_WIDTH;
const ROUNDING: f32 = 0.;

impl BoundedPainter {
    fn new(painter: Painter, bounding_box: Rect) -> Self {
        BoundedPainter {
            painter,
            bounding_box,
        }
    }

    fn normalize(rect: Rect, bounding_box: &Rect) -> Rect {
        let (min, _max) = (bounding_box.min.to_vec2(), bounding_box.max.to_vec2());
        let normalized = Rect::from_min_max(rect.min + min, rect.max + min);

        if !bounding_box.contains_rect(normalized) {
            unimplemented!("BoundingBox too large to visualize");
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

    fn draw_leaf<T>(&self, rect: Rect, data: T, highlighted: bool)
    where
        T: Display,
    {
        if !highlighted {
            self.rect(rect, ROUNDING, Stroke::new(STROKE_WIDTH, Color32::WHITE))
        } else {
            self.rect(
                rect,
                ROUNDING,
                Stroke::new(HIGHLIGHTED_STROKE_WIDTH, Color32::GREEN),
            )
        };
    }
}

fn to_rect(bbox: &BoundingBox<2>) -> Rect {
    let min = Pos2::new(bbox.bounds[0].min() as f32, bbox.bounds[1].min() as f32);
    let max = Pos2::new(bbox.bounds[0].max() as f32, bbox.bounds[1].max() as f32);
    Rect::from_min_max(min, max)
}

pub(crate) fn draw_tree<T>(
    painter: BoundedPainter,
    tree: &RTree<T, 2>,
    highlighted: &HashSet<BoundingBox<2>>,
) where
    T: Display,
{
    // TODO: visualize data in each bbox
    // TODO: visualize not only leaves, but all nodes
    // TODO: GUI interface for extended insertion, split policy choice etc.
    for (bbox, data) in tree.iter() {
        let rect = to_rect(bbox);
        painter.draw_leaf(rect, data, highlighted.contains(bbox));
    }
}
