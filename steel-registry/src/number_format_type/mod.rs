use rustc_hash::FxHashMap;
use serde_json::Value;
use steel_utils::{DowncastType, Identifier};

pub mod vanilla_number_format_types;

#[derive(Debug)]
pub struct NumberFormatType {
    pub key: Identifier,
}

struct NumberFormatCodec<T: NumberFormatType> {
    serialize: fn(&T) -> Value,
    deserialize: fn(&Value) -> Option<T>,
}

impl NumberFormatType {
    /// Creates a new number format type.
    pub const fn new(key: Identifier) -> Self {
        Self { key }
    }
}

pub trait ErasedNumberFormatType {

}

pub type ErasedNumberFormatTypeRef = &'static dyn ErasedNumberFormatType;

pub struct NumberFormatTypeRegistry {
    number_format_types_by_id: Vec<ErasedNumberFormatTypeRef>,
    number_format_types_by_key: FxHashMap<Identifier, usize>,
    allows_registering: bool,
}

impl NumberFormatTypeRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self {
            number_format_types_by_id: Vec::new(),
            number_format_types_by_key: FxHashMap::default(),
            allows_registering: true,
        }
    }
}

crate::impl_standard_methods!(
    NumberFormatTypeRegistry,
    ErasedNumberFormatTypeRef,
    number_format_types_by_id,
    number_format_types_by_key,
    allows_registering
);

crate::impl_registry!(
    NumberFormatTypeRegistry,
    dyn ErasedNumberFormatType,
    number_format_types_by_id,
    number_format_types_by_key,
    number_format_types
);