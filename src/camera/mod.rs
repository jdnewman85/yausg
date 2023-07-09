use bevy::{
    input::mouse::MouseMotion,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy::prelude::*;
use std::f32::consts::PI;

#[derive(Component)]
pub struct OrbitCamera {
    pub distance: f32,
    pub y_angle: f32,
}

#[derive(Component)]
pub struct GodModeCamera;

#[derive(Component)]
pub struct OrbitalTarget;

#[allow(dead_code)]
pub fn orbital_camera_system(
    keys: Res<Input<KeyCode>>,
    orbital_target_query: Query<&Transform, (With<OrbitalTarget>, Without<OrbitCamera>)>,
    mut camera_query: Query<(&mut Transform, &mut OrbitCamera)>,
) {
    let Ok(orbital_target) = orbital_target_query.get_single() else { return };
    let Ok((mut transform, mut camera)) = camera_query.get_single_mut() else { return };

    if keys.pressed(KeyCode::S) {
        camera.y_angle -= 0.05;
    }
    if keys.pressed(KeyCode::F) {
        camera.y_angle += 0.05;
    }
    if keys.pressed(KeyCode::E) {
        camera.distance -= 0.5;
    }
    if keys.pressed(KeyCode::D) {
        camera.distance += 0.5;
    }

    let target_position = orbital_target.translation;
    let start_position = target_position + Vec3::new(camera.distance, 0.0, 0.0);
    let mut camera_transform = Transform::from_translation(start_position);
    camera_transform.translate_around(
        target_position,
        Quat::from_euler(EulerRot::YXZ, camera.y_angle, 0.0, PI / 4.0),
    );
    *transform = camera_transform.looking_at(orbital_target.translation, Vec3::Y);
}

#[allow(dead_code)]
pub fn god_mode_camera_system(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut camera_query: Query<(&GodModeCamera, &mut Transform)>,
    mut ev_motion: EventReader<MouseMotion>,
    mouse_buttons: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
) {
    let Ok(mut window) = window_query.get_single_mut() else { return };
    let Ok((_camera, mut camera_transform)) = camera_query.get_single_mut() else { return };

    if mouse_buttons.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = CursorGrabMode::Confined;
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    }

    if keys.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }

    let cursor_locked = window.cursor.grab_mode == CursorGrabMode::Confined
        || window.cursor.grab_mode == CursorGrabMode::Locked;
    if cursor_locked {
        let mut mouse_move = Vec2::ZERO;
        for motion_event in ev_motion.into_iter() {
            mouse_move += motion_event.delta;
        }

        if mouse_move.length_squared() > 0.0 {
            let delta_x = {
                let delta = mouse_move.x / window.resolution.width() * std::f32::consts::PI * 2.0;
                //if pan_orbit.upside_down { -delta } else { delta }
                delta
            };
            let delta_y = mouse_move.y / window.resolution.height() * std::f32::consts::PI;

            let sensitivity = 0.1;
            let delta_x = delta_x * sensitivity;
            let delta_y = delta_y * sensitivity;

            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            camera_transform.rotation = yaw * camera_transform.rotation; // rotate around global y axis
            camera_transform.rotation = camera_transform.rotation * pitch; // rotate around local x axis
        }

        //let move_speed = 0.05;
        let move_speed = 0.2;
        let mut move_direction = Vec3::ZERO;
        if keys.pressed(KeyCode::S) {
            move_direction -= camera_transform.local_x();
        }
        if keys.pressed(KeyCode::F) {
            move_direction += camera_transform.local_x();
        }
        if keys.pressed(KeyCode::D) {
            move_direction += camera_transform.local_z();
        }
        if keys.pressed(KeyCode::E) {
            move_direction -= camera_transform.local_z();
        }
        if keys.pressed(KeyCode::Space) {
            move_direction += camera_transform.local_y();
        }
        if keys.pressed(KeyCode::Z) {
            move_direction -= camera_transform.local_y();
        }

        camera_transform.translation += move_direction.normalize_or_zero() * move_speed;
    }

    ev_motion.clear();
}
