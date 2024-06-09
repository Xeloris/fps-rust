use bevy::prelude::*;

use crate::structs::{
    GunController,
    PlayerController,
    CameraController
};

pub fn update(
    player_query: Query<(Entity, &Children), With<PlayerController>>,
    camera_query: Query<(&GlobalTransform, &Children), (With<CameraController>, Without<PlayerController>)>,
    mut gun_query: Query<&mut GunController>,
) {
    if let Ok((_player_entity, player_children)) = player_query.get_single() {
        for child in player_children.iter() {
            if let Ok((_camera_transform, camera_children)) = camera_query.get(*child) {
                for child in camera_children.iter() {
                    if let Ok(mut gun_controller) = gun_query.get_mut(*child) {
                        let shooting = gun_controller.shooting;
                        let just_pressed = gun_controller.just_pressed;
                        if let Some(bullet_delay) = &mut gun_controller.bullet_delay {
                            if shooting && (just_pressed || bullet_delay.finished()) {
                                bullet_delay.reset();
                                gun_controller.just_pressed = false;
                            }
                        };
                    }
                }
            }
        }
    }
}