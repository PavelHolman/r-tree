use std::collections::HashSet;

use egui::{Color32, Slider, Vec2};
use r_tree::{BoundingBox, RTree};

use crate::{draw_tree, BoundedPainter};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    min: [u64; 2],
    max: [u64; 2],

    #[serde(skip)]
    highlighted: HashSet<BoundingBox<2>>,

    #[serde(skip)]
    db: RTree<i32, 2>,

    #[serde(skip)]
    error_message: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            min: [0, 0],
            max: [100, 100],
            highlighted: HashSet::new(),
            db: example_db(),
            error_message: "".into(),
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

impl TemplateApp {
    fn show_error(&mut self, err: &'static str) {
        println!("show_error: '{}'", err);
        self.error_message = err.into();
    }

    fn hide_error(&mut self) {
        println!("hide error");
        self.error_message.clear();
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

            ui.columns(8, |columns| {
                columns[0].vertical(|ui| {
                    ui.add(Slider::new(&mut self.min[0], 0..=800).text("min x"));
                    ui.add(Slider::new(&mut self.min[1], 0..=600).text("min y"));
                    ui.separator();
                    ui.add(Slider::new(&mut self.max[0], 100..=1200).text("max x"));
                    ui.add(Slider::new(&mut self.max[1], 100..=700).text("max y"));
                });

                columns[1].vertical(|ui| {
                    ui.colored_label(Color32::RED, &self.error_message);
                });
            });

            if ui.button("Insert").clicked() {
                self.highlighted.clear();
                match BoundingBox::from_points(self.min, self.max) {
                    Ok(bbox) => {
                        self.hide_error();
                        self.highlighted.insert(bbox);
                        self.db.insert(bbox, 420);
                    }
                    Err(err) => {
                        self.show_error(err);
                    }
                }
            }

            if ui.button("Search inside").clicked() {
                self.highlighted.clear();
                match BoundingBox::from_points(self.min, self.max) {
                    Ok(bbox) => {
                        self.hide_error();
                        self.highlighted =
                            self.db.search(&bbox).map(|(bbox, _val)| *bbox).collect();
                    }
                    Err(err) => {
                        self.show_error(err);
                    }
                }
            }

            ui.separator();

            let (response, painter) =
                ui.allocate_painter(Vec2::new(1280.0, 720.0), egui::Sense::click());
            let painter = BoundedPainter::new(painter, response.rect);
            draw_tree(painter, &self.db, &self.highlighted);

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

    db.insert(p0.unwrap(), m0);
    db.insert(p1.unwrap(), m1);
    db.insert(p2.unwrap(), m2);
    db.insert(p3.unwrap(), m3);

    let (p4, m4) = (BBox::from_points([0, 0], [40, 40]), 4);
    let (p6, m6) = (BBox::from_points([0, 0], [60, 60]), 6);
    let (p8, m8) = (BBox::from_points([0, 0], [80, 80]), 8);
    let (p9, m9) = (BBox::from_points([20, 30], [100, 90]), 8);

    db.insert(p4.unwrap(), m4);
    db.insert(p6.unwrap(), m6);
    db.insert(p8.unwrap(), m8);
    db.insert(p9.unwrap(), m9);

    for (bbox, data) in db.iter() {
        println!("bbox: {:?}, data: {}", bbox, data);
    }

    db
}
