use macroquad::prelude::*;

use crate::{game::ui::Settings, Context, CAM_SPEED};

pub const KEY_MAPPINGS: [(&str, &str); 10] = [
    ("[arrows][W/A/S/D]", "Control the camera"),
    ("[LeftClick + mouse mouvement]", "Control the camera"),
    ("[R]", "Reset Camera"),
    ("[Escape]", "Open/Close settings"),
    ("[Q]", "Quit the application"),
    ("[K]", "Open the keymapping"),
    ("[B]", "Toggle the debug output"),
    ("[T]", "Switch theme"),
    ("[Mousewheel UP]", "Zoom"),
    ("[Mousewheel Down]", "Unzoom"),
];

/// Apply the input given by the user.
///
pub async fn apply_input(ctx: &mut Context, settings: &mut Settings) {
    #[cfg_attr(any(), rustfmt::skip)]
    { // Camera related //
    // Camera mouvements with keyboard
    if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up)    { ctx.camera.target.y -= CAM_SPEED; }
    if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left)  { ctx.camera.target.x -= CAM_SPEED; }
    if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down)  { ctx.camera.target.y += CAM_SPEED; }
    if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) { ctx.camera.target.x += CAM_SPEED; }

    // Mouse mouvements control
    let delta = get_frame_time();
    let mouse_position: Vec2 = mouse_position().into();
    let mouse_delta = mouse_position - ctx.last_mouse_position;
    ctx.last_mouse_position = mouse_position;

    // NOTE possibly increase the CAM_SPEED for this is currently a bit awkward
    if is_mouse_button_down(MouseButton::Left) && !ctx.hand.card_is_hovered() { 
        ctx.camera.target.x -= mouse_delta.x * delta * CAM_SPEED;
        ctx.camera.target.y -= mouse_delta.y * delta * CAM_SPEED; 
    }

    // mouse_wheel zoom
    let (_, scroll_y) = mouse_wheel();
    ctx.camera.zoom *= 1.1_f32.powf(scroll_y);

    // reset camera
    if is_key_pressed(KeyCode::R) {
        ctx.camera.target.x = 0.;
        ctx.camera.target.y = 0.;
    }
    }

    #[cfg_attr(any(), rustfmt::skip)]
    { // Settings related //
    if is_key_pressed(KeyCode::Escape) { settings.toggle_display_settings(); }
    if is_key_pressed(KeyCode::K)      { settings.toggle_display_keymapping(); }
    if is_key_pressed(KeyCode::B)      { settings.toggle_debug(); }
    if is_key_pressed(KeyCode::T)      { settings.switch_theme(); }
    }

    #[cfg_attr(any(), rustfmt::skip)]
    { // Game related //
    // Card selection
    if is_mouse_button_pressed(MouseButton::Left) && ctx.hand.card_is_hovered() { 
        ctx.hand.select_hovered_card();
    }

    // Playing card
    if ctx.hand.card_is_selected() &&
       is_mouse_button_pressed(MouseButton::Left) &&
       ctx.world.cursor_within_map(&ctx.camera)
    {
        ctx.hand.play_card(&mut ctx.creatures, ctx.world.cursor_position(&ctx.camera)).await;
    }
    }
}
