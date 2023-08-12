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
pub struct DebugCpuModule {
    digital_inputs: Entity,
    digital_outputs: Entity,
}

const DIGITAL_OFF: bool = false;
const DIGITAL_ON: bool = !DIGITAL_OFF;

impl DebugCpuModule {
    pub fn spawn_new(
        commands: &mut Commands,
        digital_input_num: usize,
        digital_output_num: usize,
    ) -> Entity {
        let input_module = commands.spawn(
            InputModule{ digital: vec![DIGITAL_OFF; digital_input_num] }
        ).id();
        let output_module = commands.spawn(
            OutputModule{ digital: vec![DIGITAL_OFF; digital_output_num] }
        ).id();

        let mut cpu_module = commands.spawn(
            DebugCpuModule {
                digital_inputs: input_module,
                digital_outputs: output_module,
            }
        );
        cpu_module.push_children(&vec![input_module, output_module]);

        cpu_module.id()
    }
}

pub fn debug_cpu_system(
//    mut commands: Commands,
    input_module_query: Query<&InputModule, With<Parent>>,
    mut output_module_query: Query<&mut OutputModule, With<Parent>>,
    debug_module_query: Query<&DebugCpuModule>,
) {
    for cpu in debug_module_query.iter() {
        let input_module = input_module_query.get(cpu.digital_inputs).unwrap();
        let mut output_module = output_module_query.get_mut(cpu.digital_outputs).unwrap();
        //TODO Assert size diff?

        for (i, output) in output_module.digital.iter_mut().enumerate() {
            *output = input_module.digital[i];
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
