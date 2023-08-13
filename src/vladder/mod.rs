#![allow(dead_code)]
use std::collections::HashMap;

use bevy::prelude::*;

//TODO Combine with Output and have permissions?
#[derive(Component, Reflect)]
pub struct InputModule {
    digital: Vec<bool>,
}
#[derive(Component, Reflect)]
pub struct OutputModule {
    digital: Vec<bool>,
}

#[derive(Default, Component)]
pub struct DebugCpuModule {
    mappings: HashMap<String, Entity>,
}

impl DebugCpuModule {
    fn create_mapping(&mut self, address: String, io_module: Entity) {
        //TODO try_insert when stable?
        //TODO Handle collisions
        //TODO Verify existence of module? Doesn't ensure prolonged existence
        //TODO Verify that io_module is child of self?
        self.mappings.insert(address, io_module);
    }
}

const DIGITAL_OFF: bool = false;
const DIGITAL_ON: bool = !DIGITAL_OFF;

impl DebugCpuModule {
    pub fn spawn_new(
        commands: &mut Commands,
        digital_input_num: usize,
        digital_output_num: usize,
    ) -> Entity {
        commands.spawn((
            DebugCpuModule::default(),
            InputModule{ digital: vec![DIGITAL_OFF; digital_input_num] },
            OutputModule{ digital: vec![DIGITAL_OFF; digital_output_num] },
        )).id()
    }
}

pub fn debug_cpu_system(
//    mut commands: Commands,
    mut debug_module_query: Query<(&InputModule, &mut OutputModule), With<DebugCpuModule>>,
) {
    for (inputs, mut outputs) in debug_module_query.iter_mut() {
        //TODO Assert size diff?

        for (i, output) in outputs.digital.iter_mut().enumerate() {
            *output = inputs.digital[i];
        }
    }
}

