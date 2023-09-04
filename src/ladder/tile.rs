use bevy::prelude::*;

use num_derive::FromPrimitive;

#[derive(Clone, Copy, Default, Debug)]
#[derive(FromPrimitive)]
pub enum Wire {
    #[default]
    Horz,
    Vert,
    LeftDown,
    LeftUp,
    RightDown,
    RightUp,
    T000,
    T090,
    T180,
    T270,
    Cross,
    _Length,
}

impl Wire {
    fn path_string(&self) -> String {
        match self {
            Self::Horz => "M 0,0.5 H 1.0",
            Self::Vert => "M 0.5,0 V 1.0",
            Self::LeftDown => "M 0,0.5 H 0.5 V 1.0",
            Self::LeftUp => "M 0,0.5 H 0.5 V 0",
            Self::RightDown => "M 1.0,0.5 H 0.5 V 1.0",
            Self::RightUp => "M 1.0,0.5 H 0.5 V 0",
            Self::T000 => "M 0,0.5 H 1.0 M 0.5,0.5 V 1.0",
            Self::T090 => "M 0.5,0 V 1.0 M 0.5,0.5 H 1.0",
            Self::T180 => "M 0,0.5 H 1.0 M 0.5,0.5 V 0",
            Self::T270 => "M 0.5,0 V 1.0 M 0.5,0.5 H 0",
            Self::Cross => "M 0,0.5 H 1.0 M 0.5,0 V 1.0",
            Self::_Length => unreachable!(),
        }.into()
    }
    //TODO TEMP
    fn scroll(&mut self, x: f32) {
        let len = Self::_Length as i32;
        let change = x.round() as i32;
        let delta_index = *self as i32 + change;
        let index = (delta_index + len) % len;
        *self = num::FromPrimitive::from_i32(index).unwrap()
    }
}

#[derive(Clone, Copy, Default, Debug)]
enum Polarity {
    #[default]
    NO,
    NC,
}

impl Polarity {
    fn invert(&mut self) {
        *self = match *self {
            Polarity::NO => Polarity::NC,
            Polarity::NC => Polarity::NO,
        };
    }
}

#[derive(Clone, Copy, Default, Debug)]
enum ContactOrCoil {
    #[default]
    Contact,
    Coil,
}
#[derive(Clone,  Debug)]
pub struct BoolElement {
    contact_or_coil: ContactOrCoil,
    address: String,
    polarity: Polarity,
}

impl BoolElement {
    //TODO Trait?
    fn path_string(&self) -> String {
        //TODO OPT? Memo: Contact/Coil and the Nc line are separate same path sections, and could
        //be extracted, and combined here instead of the match arms
        match (self.contact_or_coil, self.polarity) {
            (ContactOrCoil::Contact, Polarity::NO) => "M 0.625,0.5 H 1.0 M 0.625,0.1875 V 0.8125 M 0.375,0.1875 V 0.8125 M 0,0.5 H 0.375",
            (ContactOrCoil::Contact, Polarity::NC) => "M 0.6875,0.25L 0.3125,0.75 M 0.375,0.5 H 0 M 0.625,0.5 H 1.0 M 0.625,0.1875 V 0.8125 M 0.375,0.1875 V 0.8125",
            (ContactOrCoil::Coil, Polarity::NO) => "M 0.75,0.5 H 1.0 M 0.25,0.5 H 0 M 0.5625,0.75 A 0.26046875,0.26046875 0 0 1 0.5625,0.25M 0.4375,0.25A 0.26046875,0.26046875 0 0 1 0.4375,0.75",
            (ContactOrCoil::Coil, Polarity::NC) => "M 0.6875,0.25L 0.3125,0.75 M 0.5625,0.75 A 0.26046875,0.26046875 0 0 1 0.5625,0.25M 0.4375,0.25A 0.26046875,0.26046875 0 0 1 0.4375,0.75 M 1.0,0.5 H 0.75 M 0,0.5 H 0.25",
        }.into()
    }
}

#[derive(Component)]
pub struct TileLabelRef(pub Entity);

#[derive(Component)]
pub struct TileLabel;

#[derive(Clone, Default, Debug)]
#[derive(Component)]
pub enum Tile {
    #[default]
    None,
    BoolElement(BoolElement),
    Wire(Wire),
}

impl Tile {
    //TODO trait?
    pub fn path_string(&self) -> String {
        match self {
            Self::None => "".into(), //TODO Return Option<String>?
            Self::BoolElement(bool_element) => bool_element.path_string(),
            Self::Wire(wire) => wire.path_string(),
        }
    }

    pub fn label_string(&self) -> String {
        match self {
            Self::None |
            Self::Wire(_) => "".to_string(),
            Self::BoolElement(bool_element) => bool_element.address.clone(),
        }
    }
}

