use egui::{Slider, Vec2};
use r_tree::{BoundingBox, RTree};

use crate::draw_tree;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    min: [u64; 2],
    max: [u64; 2],

    #[serde(skip)]
    db: RTree<i32, 2>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            min: [0, 0],
            max: [100, 100],
            db: example_db(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("R-Tree");

            ui.add(Slider::new(&mut self.min[0], 0..=200).text("min x"));
            ui.add(Slider::new(&mut self.min[1], 0..=200).text("min y"));
            ui.add(Slider::new(&mut self.max[0], 100..=300).text("max x"));
            ui.add(Slider::new(&mut self.max[1], 100..=300).text("max y"));

            if ui.button("Insert").clicked() {
                let bbox = BoundingBox::from_points(self.min, self.max);
                self.db.insert(bbox, 420);
            }

            ui.separator();

            let (response, painter) =
                ui.allocate_painter(Vec2::new(1000.0, 700.0), egui::Sense::click());
            let viz_bounding_box = response.rect;
            draw_tree(
                painter,
                viz_bounding_box,
                &self.db,
                Some(self.db.iter().last().unwrap().0),
            );

            ui.separator();
        });
    }
}

fn example_db() -> RTree<i32, 2> {
    let mut db = RTree::with_capacity(4);

    type BBox = BoundingBox<2>;

    let (p0, m0) = (BBox::from_points([0, 0], [0, 0]), 0);
    let (p1, m1) = (BBox::from_points([0, 0], [10, 10]), 1);
    let (p2, m2) = (BBox::from_points([0, 0], [20, 20]), 2);
    let (p3, m3) = (BBox::from_points([0, 0], [30, 30]), 3);

    db.insert(p0, m0);
    db.insert(p1, m1);
    db.insert(p2, m2);
    db.insert(p3, m3);

    let (p4, m4) = (BBox::from_points([0, 0], [40, 40]), 4);
    let (p6, m6) = (BBox::from_points([0, 0], [60, 60]), 6);
    let (p8, m8) = (BBox::from_points([0, 0], [80, 80]), 8);
    let (p9, m9) = (BBox::from_points([20, 30], [100, 90]), 8);

    db.insert(p4, m4);
    db.insert(p6, m6);
    db.insert(p8, m8);
    db.insert(p9, m9);

    for (bbox, data) in db.iter() {
        println!("bbox: {:?}, data: {}", bbox, data);
    }

    db
}
