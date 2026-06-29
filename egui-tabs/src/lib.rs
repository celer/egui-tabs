use egui::{Color32, CornerRadius, CursorIcon, Layout, Rect, Sense, Stroke, StrokeKind};

pub struct Tabs {
    cols: i32,
    height: f32,
    sense: Sense,
    layout: Layout,
    clip: bool,
    selected_bg: TabColor,
    selected_fg: TabColor,
    hover_bg: TabColor,
    hover_fg: TabColor,
    selected: Option<i32>,
    /// Spacing between tabs.
    spacing: f32,
    /// Corner radius for tabs.
    corner_radius: CornerRadius,
}

pub enum VisualsVariant {
    HoverBackground,
    HoverForeground,
    SelectedBackground,
    SelectedForeground,
}

pub enum TabColor {
    Nothing,
    VisualsDefault(VisualsVariant),
    Custom(Color32),
}

impl TabColor {
    pub fn custom(color: Color32) -> Self {
        TabColor::Custom(color)
    }

    pub fn visuals(variant: VisualsVariant) -> Self {
        TabColor::VisualsDefault(variant)
    }

    pub fn none() -> Self {
        TabColor::Nothing
    }

    pub fn color(&self, visuals: &egui::Visuals) -> Option<Color32> {
        match self {
            TabColor::Nothing => None,
            TabColor::VisualsDefault(VisualsVariant::HoverBackground) => {
                Some(visuals.widgets.hovered.bg_fill)
            }
            TabColor::VisualsDefault(VisualsVariant::HoverForeground) => {
                Some(visuals.widgets.hovered.fg_stroke.color)
            }
            TabColor::VisualsDefault(VisualsVariant::SelectedBackground) => {
                Some(visuals.selection.bg_fill)
            }
            TabColor::VisualsDefault(VisualsVariant::SelectedForeground) => {
                Some(visuals.selection.stroke.color)
            }
            TabColor::Custom(c) => Some(*c),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TabState {
    ind: i32,
    hovered_tab: i32,
    selected_tab: i32,
}

impl TabState {
    pub fn is_hovered(&self) -> bool {
        self.hovered_tab == self.ind
    }

    pub fn is_selected(&self) -> bool {
        self.selected_tab == self.ind
    }

    pub fn hovered_tab(&self) -> Option<i32> {
        if self.hovered_tab < 0 {
            None
        } else {
            Some(self.hovered_tab)
        }
    }

    pub fn selected_tab(&self) -> Option<i32> {
        if self.selected_tab < 0 {
            None
        } else {
            Some(self.selected_tab)
        }
    }

    pub fn index(&self) -> i32 {
        self.ind
    }
}

#[derive(Default, Debug)]
pub struct TabResponse<T> {
    inner: Vec<egui::InnerResponse<T>>,
    hovered: Option<i32>,
    selected: Option<i32>,
}

impl<T> TabResponse<T> {
    pub fn hovered(&self) -> Option<i32> {
        self.hovered
    }

    pub fn selected(&self) -> Option<i32> {
        self.selected
    }

    pub fn inner(self) -> Vec<egui::InnerResponse<T>> {
        self.inner
    }
}

impl Tabs {
    pub fn new(cols: i32) -> Self {
        let height = 24.0;
        let sense = Sense::click();
        let layout = Layout::default();
        let clip = false;
        let hover_bg = TabColor::visuals(VisualsVariant::HoverBackground);
        let hover_fg = TabColor::visuals(VisualsVariant::HoverForeground);
        let selected_bg = TabColor::visuals(VisualsVariant::SelectedBackground);
        let selected_fg = TabColor::visuals(VisualsVariant::SelectedForeground);
        let selected: Option<i32> = None;
        let spacing = 2.0;
        let corner_radius: CornerRadius = CornerRadius {
            nw: 4,
            ne: 4,
            sw: 0,
            se: 0,
        };

        Tabs {
            cols,
            height,
            sense,
            layout,
            clip,
            selected_bg,
            selected_fg,
            hover_bg,
            hover_fg,
            selected,
            spacing,
            corner_radius,
        }
    }

    pub fn hover_bg(mut self, bg_fill: TabColor) -> Self {
        self.hover_bg = bg_fill;
        self
    }

    pub fn hover_fg(mut self, hover_fg: TabColor) -> Self {
        self.hover_fg = hover_fg;
        self
    }

    pub fn selected_fg(mut self, selected_fg: TabColor) -> Self {
        self.selected_fg = selected_fg;
        self
    }

    pub fn selected_bg(mut self, bg_fill: TabColor) -> Self {
        self.selected_bg = bg_fill;
        self
    }

    /// The initial selection value
    pub fn selected(mut self, selected: i32) -> Self {
        self.selected = Some(selected);
        self
    }

    pub fn sense(mut self, sense: Sense) -> Self {
        self.sense = sense;
        self
    }

    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    /// The layout of the content in the cells
    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Spacing between tabs. Default is 2.0.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Corner radius for tabs. Default is rounded top corners only.
    pub fn corner_radius(mut self, corner_radius: CornerRadius) -> Self {
        self.corner_radius = corner_radius;
        self
    }

    pub fn show<F, R>(&mut self, ui: &mut egui::Ui, add_tab: F) -> TabResponse<R>
    where
        F: Fn(&mut egui::Ui, TabState) -> R,
    {
        let mut inner = Vec::with_capacity(self.cols as usize);

        if self.cols == 0 {
            return TabResponse {
                selected: None,
                hovered: None,
                inner,
            };
        }

        // Capture visuals colors upfront to avoid borrow conflicts
        let colors = {
            let visuals = ui.visuals();
            (
                visuals.extreme_bg_color,
                visuals.window_fill(),
                visuals.widgets.noninteractive.bg_stroke.color,
                visuals.widgets.hovered.bg_stroke.color,
                visuals.text_color(),
                visuals.strong_text_color(),
            )
        };
        let (
            extreme_bg,
            window_fill,
            noninteractive_stroke,
            hovered_stroke,
            text_color_default,
            strong_text_color,
        ) = colors;

        // Allocate tab bar area
        let (_, tabbar_rect) =
            ui.allocate_space(egui::vec2(ui.available_width(), self.height));

        // Draw tab bar background
        ui.painter()
            .rect_filled(tabbar_rect, CornerRadius::ZERO, extreme_bg);

        // Calculate tab dimensions
        let total_spacing = self.spacing * (self.cols - 1) as f32;
        let cell_width = (tabbar_rect.width() - total_spacing) / self.cols as f32;

        let tabs_id = ui.id().with("tabs");
        let hover_id = tabs_id.with("hover");
        let mut any_hover = false;
        let mut hovered: Option<i32> = None;
        // Restore selection from previous frame, or use initial value
        let mut selected: Option<i32> = ui.ctx()
            .data(|d| d.get_temp::<i32>(tabs_id))
            .or(self.selected);

        // Pre-compute colors for inactive/hovered tabs
        let inactive_bg =
            egui::ecolor::tint_color_towards(window_fill, extreme_bg);
        let inactive_outline =
            egui::ecolor::tint_color_towards(noninteractive_stroke, extreme_bg);

        // Combined sense for hover + click
        let combined_sense = Sense::click() | Sense::hover();

        let mut x_offset = 0.0;
        for ind in 0..self.cols {
            let mut tab_rect = tabbar_rect;
            tab_rect.set_left(tabbar_rect.left() + x_offset);
            tab_rect.set_width(cell_width);

            let resp = ui.allocate_rect(tab_rect, combined_sense);

            if resp.clicked() {
                selected = Some(ind);
                ui.ctx()
                    .data_mut(|d| d.insert_temp(tabs_id, ind));
            }

            if resp.hovered() {
                any_hover = true;
                hovered = Some(ind);
                ui.ctx()
                    .data_mut(|d| d.insert_temp(hover_id, ind));
            }

            let is_selected = selected == Some(ind);
            let is_hovered = hovered == Some(ind);

            let tab_state = TabState {
                ind,
                selected_tab: selected.unwrap_or(-1),
                hovered_tab: hovered.unwrap_or(-1),
            };

            // Determine tab colors based on state
            let (bg_fill, outline_color, text_color) = if is_selected {
                (window_fill, noninteractive_stroke, text_color_default)
            } else if is_hovered {
                (inactive_bg, hovered_stroke, strong_text_color)
            } else {
                (inactive_bg, inactive_outline, text_color_default)
            };

            // Draw tab background
            ui.painter()
                .rect_filled(tab_rect, self.corner_radius, bg_fill);

            // Draw outline around tab
            let stroke_rect = rect_stroke_box(tab_rect, 1.0);
            ui.painter().rect_stroke(
                stroke_rect,
                self.corner_radius,
                Stroke::new(1.0, outline_color),
                StrokeKind::Inside,
            );

            // For selected tab, erase bottom border to merge with content area
            if is_selected {
                let bottom_start =
                    tab_rect.min.x + (self.corner_radius.sw as f32).max(1.5);
                let bottom_end =
                    tab_rect.max.x - (self.corner_radius.se as f32).max(1.5);
                ui.painter().hline(
                    bottom_start..=bottom_end,
                    tab_rect.bottom(),
                    Stroke::new(2.0, bg_fill),
                );
            }

            // Draw horizontal line below inactive tabs
            if !is_selected {
                let px = 1.0 / ui.ctx().pixels_per_point();
                ui.painter().hline(
                    tab_rect.x_range(),
                    tab_rect.bottom() - px,
                    (px, noninteractive_stroke),
                );
            }

            // Set cursor icon on hover
            if is_hovered {
                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
            }

            // Create child UI for tab content
            let mut child_ui = ui.new_child(
                egui::UiBuilder::new()
                    .layout(self.layout)
                    .max_rect(tab_rect),
            );
            // Left padding
            child_ui.allocate_space(egui::vec2(8.0, 0.0));
            if self.clip {
                let margin =
                    egui::Vec2::splat(ui.visuals().clip_rect_margin);
                let margin =
                    margin.min(0.5 * ui.spacing().item_spacing);
                let clip_rect = tab_rect.expand2(margin);
                child_ui.set_clip_rect(clip_rect.intersect(child_ui.clip_rect()));
            }

            // Override text color based on state
            child_ui.style_mut().visuals.override_text_color =
                Some(text_color);

            let user_value = add_tab(&mut child_ui, tab_state);
            inner.push(egui::InnerResponse::new(user_value, resp));

            x_offset += cell_width + self.spacing;
        }

        // Draw bottom separator line
        let px = 1.0 / ui.ctx().pixels_per_point();
        ui.painter().hline(
            tabbar_rect.left()..=tabbar_rect.right(),
            tabbar_rect.bottom() - px,
            (px, noninteractive_stroke),
        );

        if !any_hover {
            ui.data_mut(|data| data.remove::<i32>(hover_id));
            hovered = None;
        }

        TabResponse {
            selected,
            hovered,
            inner,
        }
    }
}

/// Shrink a rectangle uniformly from all sides to make room for a stroke.
fn rect_stroke_box(rect: Rect, stroke_width: f32) -> Rect {
    rect.expand(-f32::ceil(stroke_width / 2.0))
}
