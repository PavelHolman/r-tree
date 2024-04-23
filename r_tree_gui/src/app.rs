use eframe::egui_glow::painter;
use egui::{LayerId, Painter, Vec2};
use r_tree::{BoundingBox, RTree};

use crate::draw_tree;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    label: String,

    #[serde(skip)]
    db: RTree<i32, 2>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
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

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("R-Tree");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            // ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            // if ui.button("Increment").clicked() {
            //     self.value += 1.0;
            // }

            ui.separator();

            let (response, painter) =
                ui.allocate_painter(Vec2::new(100.0, 100.0), egui::Sense::click());
            let viz_bounding_box = response.rect;
            draw_tree(
                painter,
                viz_bounding_box,
                &self.db,
                Some(self.db.iter().last().unwrap().0),
            );

            ui.separator();

            // ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            //     powered_by_egui_and_eframe(ui);
            //     egui::warn_if_debug_build(ui);
            // });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

fn example_db() -> RTree<i32, 2> {
    let mut db = RTree::with_capacity(4);

    type BBox = BoundingBox<2>;

    let (p0, m0) = (BBox::new([(0, 0), (0, 0)]), 0);
    let (p1, m1) = (BBox::new([(0, 0), (10, 10)]), 1);
    let (p2, m2) = (BBox::new([(0, 0), (20, 20)]), 2);
    let (p3, m3) = (BBox::new([(0, 0), (30, 30)]), 3);

    db.insert(p0, m0);
    db.insert(p1, m1);
    db.insert(p2, m2);
    db.insert(p3, m3);

    let (p4, m4) = (BBox::new([(0, 0), (40, 40)]), 4);
    let (p6, m6) = (BBox::new([(0, 0), (60, 60)]), 6);
    let (p8, m8) = (BBox::new([(0, 0), (80, 80)]), 8);

    db.insert(p4, m4);
    db.insert(p6, m6);
    db.insert(p8, m8);

    for (bbox, data) in db.iter() {
        println!("bbox: {:?}, data: {}", bbox, data);
    }

    db
}
