use std::collections::HashMap;

use macroquad::prelude::*;
use macroquad::ui::{
    hash, root_ui,
    widgets::{self},
    Skin,
};

use crate::{game::keymapping::KEY_MAPPINGS, Context};

use super::world::world_to_map;

/// set the default style here
pub async fn default_skin() -> Skin {
    // To load font
    // let font = load_ttf_font("resources/fonts/Roboto/Roboto-Medium.ttf").await.unwrap();

    let label_style = root_ui()
        .style_builder()
        // .with_font(&font).unwrap()
        .text_color(BLACK)
        .font_size(16)
        .build();

    let checkbox_style = root_ui()
        .style_builder()
        .color_hovered(GRAY)
        .color_selected(GRAY)
        .build();

    let button_style = root_ui()
        .style_builder()
        .background_margin(RectOffset::new(2., 2., 2., 2.))
        .font_size(16)
        .text_color(LIGHTGRAY)
        .color(DARKGRAY)
        .build();

    Skin {
        label_style,
        checkbox_style,
        button_style,
        ..root_ui().default_skin()
    }
}

pub struct Settings {
    pub display_settings: bool,
    pub display_keymapping: bool,
    pub dark_theme: bool,
    pub debug: bool,
    pub skin: HashMap<String, Skin>,
    pub position: Vec2,
    pub window_size: Vec2,
}

impl Settings {
    pub fn builder() -> SettingsBuilder {
        SettingsBuilder {
            display_settings: None,
            display_keymapping: None,
            dark_theme: None,
            debug: None,
            skin: None,
            position: None,
            window_size: None,
        }
    }

    pub fn refresh_position(&mut self) {
        self.position = vec2(screen_width(), screen_height());
        self.position = self.position / 2. - self.window_size / 2.;
    }

    pub fn toggle_display_settings(&mut self) {
        self.display_settings = !self.display_settings;
    }

    pub fn toggle_display_keymapping(&mut self) {
        self.display_keymapping = !self.display_keymapping;
    }

    pub fn switch_theme(&mut self) {
        self.dark_theme = !self.dark_theme;
    }

    pub fn toggle_debug(&mut self) {
        self.debug = !self.debug;
    }
}

pub struct SettingsBuilder {
    display_settings: Option<bool>,
    display_keymapping: Option<bool>,
    dark_theme: Option<bool>,
    debug: Option<bool>,
    skin: Option<HashMap<String, Skin>>,
    position: Option<Vec2>,
    window_size: Option<Vec2>,
}

#[allow(dead_code)]
impl SettingsBuilder {
    pub fn display_settings(mut self, display: bool) -> Self {
        self.display_settings = Some(display);
        self
    }

    pub fn display_keymapping(mut self, display: bool) -> Self {
        self.display_keymapping = Some(display);
        self
    }

    pub fn dark_theme(mut self, dark_theme: bool) -> Self {
        self.dark_theme = Some(dark_theme);
        self
    }

    pub fn debug(mut self, dark_theme: bool) -> Self {
        self.dark_theme = Some(dark_theme);
        self
    }

    pub fn skin(mut self, skin: HashMap<String, Skin>) -> Self {
        self.skin = Some(skin);
        self
    }

    pub fn position(mut self, position: Vec2) -> Self {
        self.position = Some(position);
        self
    }

    pub fn window_size(mut self, window_size: Vec2) -> Self {
        self.window_size = Some(window_size);
        self
    }

    pub async fn build(self) -> Settings {
        let window_size = self.window_size.unwrap_or(vec2(320., 400.));
        let position = self.position.unwrap_or(vec2(
            screen_width() / 2. - window_size.x,
            screen_height() / 2. - window_size.y,
        ));

        Settings {
            display_settings: self.display_settings.unwrap_or(false),
            display_keymapping: self.display_keymapping.unwrap_or(false),
            dark_theme: self.dark_theme.unwrap_or(false),
            debug: self.debug.unwrap_or(false),
            skin: self.skin.unwrap_or(HashMap::from([(
                "Default".to_string(),
                default_skin().await,
            )])),
            position,
            window_size,
        }
    }
}

pub fn show_settings(settings: &mut Settings) {
    let (_, skin) = settings.skin.get_key_value(&"Default".to_string()).unwrap();
    root_ui().push_skin(skin);
    settings.refresh_position();

    widgets::Window::new(hash!(), settings.position, settings.window_size)
        .label("Settings")
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            ui.checkbox(hash!(), "Dark theme", &mut settings.dark_theme);
            ui.checkbox(hash!(), "Debug mode", &mut settings.debug);

            // Exit button
            if ui.button(
                vec2(settings.window_size.x - 60., settings.window_size.y - 60.),
                "Close",
            ) {
                settings.toggle_display_settings();
                ui.close_current_window();
            }
        });
}

/// Shows debuging info such as camera position, current screen size, cursor position, etc.
pub fn show_debug_info(ctx: &Context, _settings: &Settings, text_color: Color) {
    // Current screen size
    draw_text(
        &format!(
            "screen_width: {}, screen_height: {}",
            screen_width(),
            screen_height()
        ),
        10.0,
        40.0,
        20.0,
        text_color,
    );

    // Camera position
    draw_text(
        &format!(
            "camera position (x: {}, y: {})",
            ctx.camera.target.y, ctx.camera.target.y
        ),
        10.0,
        70.0,
        20.0,
        text_color,
    );

    // Cursor position global
    let mouse_in_world = ctx.camera.screen_to_world(mouse_position().into());
    draw_text(
        &format!("Cursor at position: {}", mouse_in_world),
        10.0,
        100.0,
        20.0,
        text_color,
    );

    // Cursor position in isometric context
    let mouse_in_world = ctx.camera.screen_to_world(mouse_position().into());
    draw_text(
        &format!(
            "Cursor at position in isometric context: {}",
            world_to_map(mouse_in_world)
        ),
        10.0,
        130.0,
        20.0,
        text_color,
    );

    // Hovered card index
    draw_text(
        &format!("hovered_card: {}", ctx.hand.hovered_card),
        10.0,
        160.0,
        20.0,
        text_color,
    );

    // Selected card index
    draw_text(
        &format!("selected_card: {}", ctx.hand.selected_card),
        10.0,
        190.0,
        20.0,
        text_color,
    );
}

pub async fn keymappings_skin() -> Skin {
    // let font = load_ttf_font("resources/fonts/Roboto/Roboto-Regular.ttf").await.unwrap();

    let label_style = root_ui()
        .style_builder()
        // .with_font(&font).unwrap()
        .text_color(BLACK)
        .font_size(16)
        .text_color_hovered(RED)
        .color_hovered(RED)
        .build();

    let checkbox_style = root_ui()
        .style_builder()
        .color_hovered(GRAY)
        .color_selected(GRAY)
        .build();

    let group_style = root_ui()
        .style_builder()
        .color_hovered(RED)
        .text_color_hovered(RED)
        .color_hovered(RED)
        .build();

    Skin {
        label_style,
        checkbox_style,
        group_style,
        ..root_ui().default_skin()
    }
}

pub fn show_keymapping(settings: &mut Settings) {
    settings.refresh_position();
    let (_, skin) = settings
        .skin
        .get_key_value(&"Keymapping".to_string())
        .unwrap();

    let mut close_clicked = false;

    widgets::Window::new(hash!(), settings.position, settings.window_size)
        .label("Keymappings")
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            ui.push_skin(skin);
            for (key, description) in KEY_MAPPINGS {
                ui.separator();
                ui.group(hash!(key, 1), vec2(280., 60.), |inner_ui| {
                    inner_ui.label(None, key);
                    inner_ui.separator();
                    inner_ui.same_line(20.);
                    inner_ui.label(None, description);
                });
                ui.separator();
            }

            // Exit button
            if ui.button(None, "Close") {
                close_clicked = true;
                ui.close_current_window();
            }

            ui.separator();
            ui.pop_skin();
        });

    if close_clicked {
        settings.toggle_display_keymapping();
    }
}
