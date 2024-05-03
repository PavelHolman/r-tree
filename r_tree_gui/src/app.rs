use std::{collections::HashMap, iter::repeat, usize};

use egui::{Color32, ComboBox, DragValue, FontFamily, Slider, Vec2};
use r_tree::{BoundingBox, Interval, RTree, SplitPolicy};

use crate::{draw_tree, BoundedPainter, Highlight};

pub type TreeValueType = u64;

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
        Self::new(2, 2, SplitPolicy::Half)
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
enum TreeDimension {
    Two(RTree<TreeValueType, 2>),
    Three(RTree<TreeValueType, 3>),
}
struct Bounds<const N: usize> {
    min: [u64; N],
    max: [u64; N],
}

struct BoundGroup2d {
    insertion_bounds: Bounds<2>,
    search_bounds: Bounds<2>,
    nearest_bounds: Bounds<2>,
}

impl Default for BoundGroup2d {
    fn default() -> Self {
        Self {
            insertion_bounds: Bounds {
                min: [0, 0],
                max: [1200, 700],
            },
            search_bounds: Bounds {
                min: [0, 0],
                max: [1200, 700],
            },
            nearest_bounds: Bounds {
                min: [0, 0],
                max: [1200, 700],
            },
        }
    }
}

struct BoundGroup<const N: usize> {
    insertion_bounds: Bounds<N>,
    search_bounds: Bounds<N>,
    nearest_bounds: Bounds<N>,
}

impl<const N: usize> Default for BoundGroup<N> {
    fn default() -> Self {
        Self {
            insertion_bounds: Bounds {
                min: [0; N],
                max: [1200; N],
            },
            search_bounds: Bounds {
                min: [0; N],
                max: [1200; N],
            },
            nearest_bounds: Bounds {
                min: [0; N],
                max: [1200; N],
            },
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct GUIApp {
    new_value: TreeValueType,

    k_nearest: u8,

    #[serde(skip)]
    two_d: BoundGroup2d,

    #[serde(skip)]
    three_d: BoundGroup<3>,

    #[serde(skip)]
    highlighted: HashMap<BoundingBox<2>, Highlight>,

    #[serde(skip)]
    search_box: Option<BoundingBox<2>>,

    tree_dimension: TreeDimension,

    #[serde(skip)]
    new_tree: TreeParams,

    #[serde(skip)]
    error_message: String,
}

impl Default for GUIApp {
    fn default() -> Self {
        Self {
            two_d: Default::default(),
            three_d: Default::default(),
            new_value: 420,
            k_nearest: 2,
            highlighted: HashMap::new(),
            search_box: None,
            tree_dimension: TreeDimension::Two(RTree::new()),
            error_message: "".into(),
            new_tree: Default::default(),
        }
    }
}

impl GUIApp {
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

impl GUIApp {
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

    fn format_interval(interval: Interval) -> String {
        format!("{}..{}", interval.min(), interval.max())
    }

    fn format_bbox_3d(bbox: BoundingBox<3>) -> String {
        format!(
            "{: >10} {: >10} {: >10}",
            Self::format_interval(bbox.bounds[0]),
            Self::format_interval(bbox.bounds[1]),
            Self::format_interval(bbox.bounds[2]),
        )
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

            match self.new_tree.dimension {
                2 => {
                    self.tree_dimension = TreeDimension::Two(RTree::with_capacity_and_split_policy(
                        self.new_tree.capacity,
                        self.new_tree.split_policy.clone(),
                    ))
                }
                3 => {
                    self.tree_dimension =
                        TreeDimension::Three(RTree::with_capacity_and_split_policy(
                            self.new_tree.capacity,
                            self.new_tree.split_policy.clone(),
                        ))
                }
                n => unimplemented!(
                    "For purposes of the GUI, R-Trees must be of dimension 2 or 3, got {n}"
                ),
            };
        }
    }

    fn insert_controls(&mut self, ui: &mut egui::Ui) {
        match self.tree_dimension {
            TreeDimension::Two(_) => self.insert_controls_2d(ui),
            TreeDimension::Three(_) => self.insert_controls_3d(ui),
        }
    }

    fn insert_controls_2d(&mut self, ui: &mut egui::Ui) {
        ui.label("Insert");
        ui.add(Slider::new(&mut self.two_d.insertion_bounds.min[0], 0..=1200).text("min x"));
        ui.add(Slider::new(&mut self.two_d.insertion_bounds.min[1], 0..=700).text("min y"));
        ui.separator();
        ui.add(Slider::new(&mut self.two_d.insertion_bounds.max[0], 0..=1200).text("max x"));
        ui.add(Slider::new(&mut self.two_d.insertion_bounds.max[1], 0..=700).text("max y"));
        ui.add(DragValue::new(&mut self.new_value).speed(10));

        if ui.button("Insert").clicked() {
            self.reset_info();

            match BoundingBox::from_points(
                self.two_d.insertion_bounds.min,
                self.two_d.insertion_bounds.max,
            ) {
                Ok(bbox) => {
                    self.highlighted.insert(bbox, Highlight::Leaf);
                    if let TreeDimension::Two(ref mut db) = self.tree_dimension {
                        db.insert(bbox, self.new_value);
                    }
                }
                Err(err) => {
                    self.show_error(err);
                }
            }
        }
    }

    fn insert_controls_3d(&mut self, ui: &mut egui::Ui) {
        ui.label("Insert");
        ui.add(Slider::new(&mut self.three_d.insertion_bounds.min[0], 0..=1000).text("min x"));
        ui.add(Slider::new(&mut self.three_d.insertion_bounds.min[1], 0..=1000).text("min y"));
        ui.add(Slider::new(&mut self.three_d.insertion_bounds.min[2], 0..=1000).text("min z"));
        ui.separator();
        ui.add(Slider::new(&mut self.three_d.insertion_bounds.max[0], 0..=1000).text("max x"));
        ui.add(Slider::new(&mut self.three_d.insertion_bounds.max[1], 0..=1000).text("max y"));
        ui.add(Slider::new(&mut self.three_d.insertion_bounds.max[2], 0..=1000).text("max z"));
        ui.add(DragValue::new(&mut self.new_value).speed(10));

        if ui.button("Insert").clicked() {
            self.reset_info();

            match BoundingBox::from_points(
                self.three_d.insertion_bounds.min,
                self.three_d.insertion_bounds.max,
            ) {
                Ok(bbox) => {
                    if let TreeDimension::Three(ref mut db) = self.tree_dimension {
                        let old = db.insert(bbox, self.new_value);
                        println!(
                            "Inserted {: >6} at {}",
                            self.new_value,
                            Self::format_bbox_3d(bbox)
                        );

                        if let Some(old) = old {
                            println!("Replaced value {}", old);
                        }
                    }
                }
                Err(err) => {
                    self.show_error(err);
                }
            }
        }
    }

    fn search_controls(&mut self, ui: &mut egui::Ui) {
        match self.tree_dimension {
            TreeDimension::Two(_) => self.search_controls_2d(ui),
            TreeDimension::Three(_) => self.search_controls_3d(ui),
        }
    }

    fn search_controls_2d(&mut self, ui: &mut egui::Ui) {
        ui.label("Search");
        ui.add(Slider::new(&mut self.two_d.search_bounds.min[0], 0..=1200).text("min x"));
        ui.add(Slider::new(&mut self.two_d.search_bounds.min[1], 0..=700).text("min y"));
        ui.separator();
        ui.add(Slider::new(&mut self.two_d.search_bounds.max[0], 0..=1200).text("max x"));
        ui.add(Slider::new(&mut self.two_d.search_bounds.max[1], 0..=700).text("max y"));

        if ui.button("Search inside").clicked() {
            self.reset_info();

            match BoundingBox::from_points(
                self.two_d.search_bounds.min,
                self.two_d.search_bounds.max,
            ) {
                Ok(bbox) => {
                    if let TreeDimension::Two(db) = &self.tree_dimension {
                        self.highlighted = db
                            .search(&bbox)
                            .map(|(bbox, _val)| *bbox)
                            .zip(repeat(Highlight::Leaf))
                            .collect();
                        self.search_box = Some(bbox);
                    }
                }
                Err(err) => {
                    self.show_error(err);
                }
            }
        }
    }

    fn search_controls_3d(&mut self, ui: &mut egui::Ui) {
        ui.label("Search");
        ui.add(Slider::new(&mut self.three_d.search_bounds.min[0], 0..=1000).text("min x"));
        ui.add(Slider::new(&mut self.three_d.search_bounds.min[1], 0..=1000).text("min y"));
        ui.add(Slider::new(&mut self.three_d.search_bounds.min[2], 0..=1000).text("min z"));
        ui.separator();
        ui.add(Slider::new(&mut self.three_d.search_bounds.max[0], 0..=1000).text("max x"));
        ui.add(Slider::new(&mut self.three_d.search_bounds.max[1], 0..=1000).text("max y"));
        ui.add(Slider::new(&mut self.three_d.search_bounds.max[2], 0..=1000).text("max z"));

        if ui.button("Search inside").clicked() {
            self.reset_info();

            match BoundingBox::from_points(
                self.three_d.search_bounds.min,
                self.three_d.search_bounds.max,
            ) {
                Ok(bbox) => {
                    if let TreeDimension::Three(db) = &self.tree_dimension {
                        println!("Searching @ {}", Self::format_bbox_3d(bbox));
                        let found: Vec<_> = db.search(&bbox).collect();
                        for (bbox, data) in found {
                            println!("{: >8} at {}", data, Self::format_bbox_3d(*bbox));
                        }
                    }
                }
                Err(err) => {
                    self.show_error(err);
                }
            }
        }
    }

    fn nearest_search_controls(&mut self, ui: &mut egui::Ui) {
        match self.tree_dimension {
            TreeDimension::Two(_) => self.nearest_search_controls_2d(ui),
            TreeDimension::Three(_) => self.nearest_search_controls_3d(ui),
        }
    }

    fn nearest_search_controls_2d(&mut self, ui: &mut egui::Ui) {
        ui.label("Nearest Neighbors");
        ui.add(Slider::new(&mut self.two_d.nearest_bounds.min[0], 0..=1200).text("min x"));
        ui.add(Slider::new(&mut self.two_d.nearest_bounds.min[1], 0..=700).text("min y"));
        ui.separator();
        ui.add(Slider::new(&mut self.two_d.nearest_bounds.max[0], 0..=1200).text("max x"));
        ui.add(Slider::new(&mut self.two_d.nearest_bounds.max[1], 0..=700).text("max y"));
        ui.add(DragValue::new(&mut self.k_nearest).speed(0.1))
            .on_hover_text("k");

        if ui.button("Find nearest neighbors").clicked() {
            self.reset_info();

            match BoundingBox::from_points(
                self.two_d.nearest_bounds.min,
                self.two_d.nearest_bounds.max,
            ) {
                Ok(bbox) => {
                    if let TreeDimension::Two(db) = &self.tree_dimension {
                        self.highlighted = db
                            .nearest(&bbox)
                            .take(self.k_nearest.into())
                            .map(|(bbox, _val)| *bbox)
                            .zip(repeat(Highlight::Leaf))
                            .collect();
                        self.search_box = Some(bbox);
                    }
                }
                Err(err) => {
                    self.show_error(err);
                }
            }
        }
    }

    fn nearest_search_controls_3d(&mut self, ui: &mut egui::Ui) {
        ui.label("Nearest Neighbors");
        ui.add(Slider::new(&mut self.three_d.nearest_bounds.min[0], 0..=1000).text("min x"));
        ui.add(Slider::new(&mut self.three_d.nearest_bounds.min[1], 0..=1000).text("min y"));
        ui.add(Slider::new(&mut self.three_d.nearest_bounds.min[2], 0..=1000).text("min z"));
        ui.separator();
        ui.add(Slider::new(&mut self.three_d.nearest_bounds.max[0], 0..=1000).text("max x"));
        ui.add(Slider::new(&mut self.three_d.nearest_bounds.max[1], 0..=1000).text("max y"));
        ui.add(Slider::new(&mut self.three_d.nearest_bounds.max[2], 0..=1000).text("max z"));
        ui.add(DragValue::new(&mut self.k_nearest).speed(0.1))
            .on_hover_text("k");

        if ui.button("Find nearest neighbors").clicked() {
            self.reset_info();

            match BoundingBox::from_points(
                self.three_d.nearest_bounds.min,
                self.three_d.nearest_bounds.max,
            ) {
                Ok(bbox) => {
                    if let TreeDimension::Three(db) = &self.tree_dimension {
                        println!(
                            "Searching {} nearest neighbors of {}",
                            self.k_nearest,
                            Self::format_bbox_3d(bbox)
                        );
                        let found: Vec<_> = db.nearest(&bbox).take(self.k_nearest.into()).collect();
                        println!("Found");
                        for (bbox, data) in found {
                            println!("{: >8} at {}", data, Self::format_bbox_3d(*bbox));
                        }
                    }
                }
                Err(err) => {
                    self.show_error(err);
                }
            }
        }
    }
}

const N_COLUMNS: usize = 8;

impl eframe::App for GUIApp {
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

                columns[3].vertical(|ui| self.nearest_search_controls(ui));

                if columns[4].button("Reset highlight").clicked() {
                    self.reset_info();
                }

                columns[N_COLUMNS - 1].vertical(|ui| {
                    ui.colored_label(Color32::RED, &self.error_message);
                });
            });

            ui.separator();

            let (response, painter) =
                ui.allocate_painter(Vec2::new(1280., 720.), egui::Sense::click());

            if let TreeDimension::Two(db) = &self.tree_dimension {
                let painter = BoundedPainter::new(painter, response.rect);
                draw_tree(painter, &db, &self.highlighted, self.search_box);
            } else {
                painter.text(
                    response.rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "Visualization is only available for trees of dimension 2\nRefer to the standard output for output",
                    egui::FontId::new(18.0, FontFamily::default()),
                    Color32::WHITE,
                );
            }

            ui.separator();
        });
    }
}
