//! This module defines all the number formats from Vanilla.

use crate::number_format_type::{NumberFormatType, NumberFormatTypeRef, NumberFormatTypeRegistry};
use steel_utils::Identifier;
use steel_utils::serial::{ReadFrom, WriteTo};

pub static BLANK: NumberFormatType = NumberFormatType::new(Identifier::vanilla_static("blank"));
pub static STYLED: NumberFormatType = NumberFormatType::new(Identifier::vanilla_static("styled"));
pub static FIXED: NumberFormatType = NumberFormatType::new(Identifier::vanilla_static("fixed"));

/// Registers Vanilla number formats to the registry.
pub fn register_number_format_types(registry: &mut NumberFormatTypeRegistry) {
    registry.register(&BLANK);
    registry.register(&STYLED);
    registry.register(&FIXED);
}

// TODO: Add tests here
