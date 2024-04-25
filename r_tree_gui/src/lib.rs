#![warn(clippy::all, rust_2018_idioms)]

pub use app::TemplateApp;
use egui::{
    layers::ShapeIdx, Align2, Color32, FontFamily, FontId, Painter, Pos2, Rect, Rounding, Stroke,
};
use r_tree::{BoundingBox, RTree};
use std::{collections::HashMap, fmt::Display};

mod app;

pub(crate) struct Bounds {
    min: [u64; 2],
    max: [u64; 2],
}

#[derive(Copy, Clone)]
pub(crate) enum Highlight {
    None,
    Search,
    Leaf,
}

struct BoundedPainter {
    painter: Painter,
    bounding_box: Rect,
}

const STROKE_WIDTH: f32 = 2.;
const HIGHLIGHTED_STROKE_WIDTH: f32 = 2. * STROKE_WIDTH;
const ROUNDING: f32 = 0.;
const FONT_SIZE: f32 = 14.;

#[derive(Debug)]
struct NormalizedRect {
    inner_rect: Rect,
}

impl NormalizedRect {
    fn new(rect: Rect, bounding_box: Rect) -> NormalizedRect {
        let (min, _max) = (bounding_box.min.to_vec2(), bounding_box.max.to_vec2());
        let normalized = Rect::from_min_max(rect.min + min, rect.max + min);

        if !bounding_box.contains_rect(normalized) {
            unimplemented!("BoundingBox too large to visualize");
        }

        Self {
            inner_rect: normalized,
        }
    }

    fn center(&self) -> Pos2 {
        self.inner_rect.center()
    }
}

impl Into<Rect> for NormalizedRect {
    fn into(self) -> Rect {
        self.inner_rect
    }
}

impl BoundedPainter {
    fn new(painter: Painter, bounding_box: Rect) -> Self {
        BoundedPainter {
            painter,
            bounding_box,
        }
    }

    fn rect(
        &self,
        rect: NormalizedRect,
        rounding: impl Into<Rounding>,
        stroke: impl Into<Stroke>,
    ) -> ShapeIdx {
        self.painter.rect_stroke(rect.into(), rounding, stroke)
    }

    fn apply_highlight(highlight: Highlight) -> (f32, Color32) {
        match highlight {
            Highlight::Leaf => (HIGHLIGHTED_STROKE_WIDTH, Color32::GREEN),
            Highlight::Search => (HIGHLIGHTED_STROKE_WIDTH, Color32::YELLOW),
            Highlight::None => (STROKE_WIDTH, Color32::WHITE),
        }
    }

    fn draw_rect(&self, rect: Rect, highlight: Highlight) {
        let rect = NormalizedRect::new(rect, self.bounding_box);
        let (stroke_width, stroke_color) = BoundedPainter::apply_highlight(highlight);

        self.rect(rect, ROUNDING, Stroke::new(stroke_width, stroke_color));
    }

    fn draw_leaf<T>(&self, rect: Rect, data: &T, highlight: Highlight)
    where
        T: ToString + Display,
    {
        let rect = NormalizedRect::new(rect, self.bounding_box);
        let (stroke_width, stroke_color) = BoundedPainter::apply_highlight(highlight);

        let rect_center = rect.center();
        let font_id = FontId::new(FONT_SIZE, FontFamily::default());

        self.painter.text(
            rect_center,
            Align2::CENTER_CENTER,
            data,
            font_id,
            stroke_color,
        );

        self.rect(rect, ROUNDING, Stroke::new(stroke_width, stroke_color));
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
    highlighted: &HashMap<BoundingBox<2>, Highlight>,
    search_box: Option<BoundingBox<2>>,
) where
    T: ToString + Display,
{
    // TODO: visualize data in each bbox
    // TODO: visualize not only leaves, but all nodes
    // TODO: GUI interface for extended insertion, split policy choice etc.

    // draw leaves
    for (bbox, data) in tree.iter() {
        let rect = to_rect(bbox);
        let highlight = highlighted.get(bbox).unwrap_or(&Highlight::None);
        painter.draw_leaf(rect, &data, *highlight);
    }

    // draw search box
    if let Some(search_box) = search_box {
        let rect = to_rect(&search_box);
        painter.draw_rect(rect, Highlight::Search);
    }
}
