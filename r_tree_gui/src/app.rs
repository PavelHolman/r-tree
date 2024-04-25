use std::{collections::HashMap, iter::repeat};

use egui::{Color32, ComboBox, DragValue, Slider, Vec2};
use r_tree::{BoundingBox, RTree, SplitPolicy};

use crate::{draw_tree, BoundedPainter, Bounds, Highlight};

struct TreeParams {
    dimension: u8,
    capacity: usize,
    split_policy: SplitPolicy,
}

impl TreeParams {
    fn new(dimension: u8, capacity: usize, split_policy: SplitPolicy) -> TreeParams {
        Self {
            dimension,
            capacity,
            split_policy,
        }
    }
}

impl Default for TreeParams {
    fn default() -> Self {
        Self::new(2, 4, SplitPolicy::Half)
    }
}

enum TreeDimension {
    Two(RTree<i32, 2>),
    Three,
    Four,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    insertion_bounds: Bounds,

    new_value: i32,

    search_bounds: Bounds,

    #[serde(skip)]
    highlighted: HashMap<BoundingBox<2>, Highlight>,

    #[serde(skip)]
    search_box: Option<BoundingBox<2>>,

    #[serde(skip)]
    tree_dimension: TreeDimension,

    #[serde(skip)]
    db_3d: RTree<i32, 3>,
    #[serde(skip)]
    db_4d: RTree<i32, 4>,

    #[serde(skip)]
    new_tree: TreeParams,

    #[serde(skip)]
    error_message: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            insertion_bounds: Bounds {
                min: [0, 0],
                max: [1200, 1200],
            },
            search_bounds: Bounds {
                min: [0, 0],
                max: [1200, 1200],
            },
            new_value: 420,
            highlighted: HashMap::new(),
            search_box: None,
            tree_dimension: TreeDimension::Two(RTree::new()),
            db_3d: RTree::new(),
            db_4d: RTree::new(),
            error_message: "".into(),
            new_tree: Default::default(),
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
        self.error_message = err.into();
    }

    fn hide_error(&mut self) {
        self.error_message.clear();
    }

    fn reset_info(&mut self) {
        self.hide_error();
        self.highlighted.clear();
        self.search_box = None;
    }

    fn tree_builder(&mut self, ui: &mut egui::Ui) {
        ui.label("New tree");

        ComboBox::from_label("node dimension")
            .selected_text(self.new_tree.dimension.to_string())
            .show_ui(ui, |ui| {
                for dim in 2..=4 {
                    ui.selectable_value(&mut self.new_tree.dimension, dim, dim.to_string());
                }
            });

        ui.add(Slider::new(&mut self.new_tree.capacity, 2..=16).text("node capacity"));

        ComboBox::from_label("split policy")
            .selected_text(format!("{:?}", self.new_tree.split_policy))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.new_tree.split_policy, SplitPolicy::Half, "Half");
                ui.selectable_value(
                    &mut self.new_tree.split_policy,
                    SplitPolicy::Random,
                    "Random",
                );
                ui.selectable_value(
                    &mut self.new_tree.split_policy,
                    SplitPolicy::Greedy,
                    "Greedy",
                );
                ui.selectable_value(
                    &mut self.new_tree.split_policy,
                    SplitPolicy::BruteForce,
                    "BruteForce",
                );
            });

        if ui.button("Create tree").clicked() {
            self.reset_info();

            match &self.tree_dimension {
                TreeDimension::Two(_) => {
                    self.tree_dimension =
                        TreeDimension::Two(RTree::with_capacity_and_split_policy(
                            self.new_tree.capacity,
                            self.new_tree.split_policy.clone(),
                        ));
                }
                TreeDimension::Three => {
                    self.tree_dimension = TreeDimension::Three;
                    self.db_3d = RTree::with_capacity_and_split_policy(
                        self.new_tree.capacity,
                        self.new_tree.split_policy.clone(),
                    );
                }
                TreeDimension::Four => {
                    self.tree_dimension = TreeDimension::Four;
                    self.db_4d = RTree::with_capacity_and_split_policy(
                        self.new_tree.capacity,
                        self.new_tree.split_policy.clone(),
                    );
                }
            }
        }
    }

    fn insert_controls(&mut self, ui: &mut egui::Ui) {
        ui.label("Insert");
        ui.add(Slider::new(&mut self.insertion_bounds.min[0], 0..=1200).text("min x"));
        ui.add(Slider::new(&mut self.insertion_bounds.min[1], 0..=700).text("min y"));
        ui.separator();
        ui.add(Slider::new(&mut self.insertion_bounds.max[0], 0..=1200).text("max x"));
        ui.add(Slider::new(&mut self.insertion_bounds.max[1], 0..=700).text("max y"));
        ui.add(DragValue::new(&mut self.new_value).speed(10));

        if ui.button("Insert").clicked() {
            self.reset_info();

            match BoundingBox::from_points(self.insertion_bounds.min, self.insertion_bounds.max) {
                Ok(bbox) => match &mut self.tree_dimension {
                    TreeDimension::Two(db) => {
                        self.highlighted.insert(bbox, Highlight::Leaf);
                        db.insert(bbox, self.new_value);
                    }
                    _ => todo!("Insertion into higher dimension rtrees"),
                },
                Err(err) => {
                    self.show_error(err);
                }
            }
        }
    }

    fn search_controls(&mut self, ui: &mut egui::Ui) {
        ui.label("Search");
        ui.add(Slider::new(&mut self.search_bounds.min[0], 0..=1200).text("min x"));
        ui.add(Slider::new(&mut self.search_bounds.min[1], 0..=700).text("min y"));
        ui.separator();
        ui.add(Slider::new(&mut self.search_bounds.max[0], 0..=1200).text("max x"));
        ui.add(Slider::new(&mut self.search_bounds.max[1], 0..=700).text("max y"));

        if ui.button("Search inside").clicked() {
            self.reset_info();

            match BoundingBox::from_points(self.search_bounds.min, self.search_bounds.max) {
                Ok(bbox) => match &mut self.tree_dimension {
                    TreeDimension::Two(db) => {
                        self.highlighted = db
                            .search(&bbox)
                            .map(|(bbox, _val)| *bbox)
                            .zip(repeat(Highlight::Leaf))
                            .collect();
                        self.search_box = Some(bbox);
                    }
                    _ => todo!("Searching in higher dimension rtrees"),
                },
                Err(err) => {
                    self.show_error(err);
                }
            }
        }
    }
}

const N_COLUMNS: usize = 8;

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

            ui.columns(N_COLUMNS, |columns| {
                columns[0].vertical(|ui| self.tree_builder(ui));

                columns[1].vertical(|ui| self.insert_controls(ui));

                columns[2].vertical(|ui| self.search_controls(ui));

                if columns[3].button("Reset highlight").clicked() {
                    self.reset_info();
                }

                columns[N_COLUMNS - 1].vertical(|ui| {
                    ui.colored_label(Color32::RED, &self.error_message);
                });
            });

            ui.separator();

            if let TreeDimension::Two(db) = &self.tree_dimension {
                let (response, painter) =
                    ui.allocate_painter(Vec2::new(1280.0, 720.0), egui::Sense::click());
                let painter = BoundedPainter::new(painter, response.rect);
                draw_tree(painter, &db, &self.highlighted, self.search_box);

                ui.separator();
            }
        });
    }
}
