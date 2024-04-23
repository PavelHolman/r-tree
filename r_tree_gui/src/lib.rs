#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TemplateApp;

use log::debug;
use r_tree::RTree;

pub(crate) fn draw_tree<T, const N: usize>(tree: &RTree<T, N>) {
    for (bbox, data) in tree.iter() {
        println!("{:?}", bbox);
    }
}
