#![allow(dead_code)]
use std::{collections::HashMap, error::{Error, self}, fmt};

use regex::Regex;

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

#[derive(Debug)]
struct AddressError;
impl error::Error for AddressError {}
impl fmt::Display for AddressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(TODO: Better) invalid address")
    }
}

#[derive(Bundle)]
pub struct DebugCpuBundle {
    debug_cpu: DebugCpuModule,
    input_module: InputModule,
    output_module: OutputModule,
}

impl DebugCpuModule {
    pub fn new(
        digital_input_num: usize,
        digital_output_num: usize,
    ) -> DebugCpuBundle {
        DebugCpuBundle {
            debug_cpu: DebugCpuModule::default(),
            input_module: InputModule{ digital: vec![DIGITAL_OFF; digital_input_num] },
            output_module: OutputModule{ digital: vec![DIGITAL_OFF; digital_output_num] },
        }
    }

    pub fn digital(&self, address: String) -> Result<bool, Box<dyn Error>> {
        let re = Regex::new(r"([[:alpha:]]+)(\d+)").unwrap();
        let captures = re.captures(&address).ok_or(AddressError)?;
        let _word = captures.get(1).ok_or(AddressError)?;
        let _number = captures.get(2).ok_or(AddressError)?;

        //TODO Finish
        //dbg!(word, number);

        Ok(true)
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
