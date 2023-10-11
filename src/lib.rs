#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    unused_results,
    rustdoc::broken_intra_doc_links
)]

use iced_widget::{renderer, style};

mod grid;
mod row;

pub use grid::Grid;
pub use grid::Strategy;
pub use row::GridRow;

type Renderer<Theme = style::Theme> = renderer::Renderer<Theme>;

/// Creates a [`Grid`] with the given [`GridRow`]s.
#[macro_export]
macro_rules! grid {
    () => (
        $crate::Grid::new()
    );
    ($($x:expr),+ $(,)?) => (
        $crate::Grid::with_rows(vec![$($x),+])
    );
}

/// Creates a [`GridRow`] with the given widgets.
#[macro_export]
macro_rules! grid_row {
    () => (
        $crate::GridRow::new()
    );
    ($($x:expr),+ $(,)?) => (
        $crate::GridRow::with_elements(vec![$(iced::Element::from($x)),+])
    );
}
