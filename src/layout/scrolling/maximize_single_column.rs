//! maximize-single-column: when a workspace has a single column, size it as full-width (like
//! maximize-column) without touching its stored width, so that it returns to that width as soon as
//! another column appears.
//!
//! A manual width change while the column is alone (set-column-width, switch-preset-column-width,
//! maximize-column, interactive resize) opts the column out until it stops being alone.

use std::iter::zip;

use niri_config::PresetSize;

use super::{Column, ScrollingSpace};
use crate::layout::LayoutElement;

impl<W: LayoutElement> Column<W> {
    /// Returns whether this column is full-width due to maximize-single-column.
    pub(super) fn is_single_column_maximized(&self) -> bool {
        self.options.layout.maximize_single_column
            && self.is_sole_column
            && !self.is_single_column_maximize_disabled
    }

    /// Returns whether this column should be sized as full-width.
    pub(super) fn is_effectively_full_width(&self) -> bool {
        self.is_full_width || self.is_single_column_maximized()
    }

    pub(super) fn set_is_sole_column(&mut self, is_sole_column: bool) {
        if self.is_sole_column == is_sole_column {
            return;
        }

        let was_full_width = self.is_effectively_full_width();

        self.is_sole_column = is_sole_column;
        // A manual resize only opts out for the current stretch of being the sole column.
        self.is_single_column_maximize_disabled = false;

        if was_full_width != self.is_effectively_full_width() {
            self.update_tile_sizes(true);
        }
    }
}

impl<W: LayoutElement> ScrollingSpace<W> {
    /// Updates whether columns are the sole column on this workspace.
    pub(super) fn update_sole_column(&mut self) {
        let is_sole = self.columns.len() == 1;
        for (col, data) in zip(&mut self.columns, &mut self.data) {
            col.set_is_sole_column(is_sole);
            data.update(col);
        }
    }

    /// Width to request from a new window.
    ///
    /// The first window on an empty workspace will become the sole column, so request full width
    /// right away to avoid an extra resize after it maps. When width is None, the window picks its
    /// own width, which then becomes the stored column width, so we must not override it.
    pub(super) fn maximize_single_column_new_window_width(
        &self,
        width: Option<PresetSize>,
    ) -> Option<PresetSize> {
        if width.is_some() && self.options.layout.maximize_single_column && self.columns.is_empty()
        {
            Some(PresetSize::Proportion(1.))
        } else {
            width
        }
    }
}
