use bevy::{
    prelude::*, render::view::screenshot::ScreenshotManager, window::PrimaryWindow,
};

pub fn screenshot_on_spacebar(
    input: Res<Input<KeyCode>>,
    main_window: Query<Entity, With<PrimaryWindow>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    mut counter: Local<u32>,
) {
    if !input.just_pressed(KeyCode::P) { return; }
    let path = format!("./screenshot-{}.png", *counter);
    *counter += 1;
    screenshot_manager.save_screenshot_to_disk(main_window.single(), path).unwrap();
}

