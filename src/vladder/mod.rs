use bevy::prelude::*;

#[derive(Component, Reflect)]
pub struct InputModule {
    digital: Vec<bool>,
}

#[derive(Component, Reflect)]
pub struct OutputModule {
    digital: Vec<bool>,
}

#[derive(Component)]
pub struct DebugCpuModule;

const DIGITAL_OFF: bool = false;
const DIGITAL_ON: bool = !DIGITAL_OFF;

impl DebugCpuModule {
    pub fn spawn_new(
        commands: &mut Commands,
        digital_input_num: usize,
        digital_output_num: usize,
    ) -> Entity {
        commands.spawn((
            DebugCpuModule,
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

pub fn init_debug_input_system(
//    mut commands: Commands,
    mut input_module_query: Query<&mut InputModule, Added<InputModule>>,
) {
    for mut input_module in input_module_query.iter_mut() {
        for (i, input) in input_module.digital.iter_mut().enumerate() {
            *input = i % 2 == 1;
        }
    }
}
