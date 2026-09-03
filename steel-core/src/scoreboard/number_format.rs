use std::io::{Cursor, Write};
use steel_registry::number_format_type::{vanilla_number_format_types, NumberFormatTypeRef};
use steel_utils::serial::{ReadFrom, WriteTo};

/// A trait that number formats implement.
pub trait NumberFormat: WriteTo + ReadFrom {
    fn format_type(&self) -> NumberFormatTypeRef;
}

/// This format uses a blank instead of actually displaying the number.
pub struct BlankFormat;

impl WriteTo for BlankFormat {
    fn write(&self, writer: &mut impl Write) -> std::io::Result<()> {
        Ok(())
    }
}

impl ReadFrom for BlankFormat {
    fn read(data: &mut Cursor<&[u8]>) -> std::io::Result<Self> {
        Ok(BlankFormat)
    }
}

impl NumberFormat for BlankFormat {
    fn format_type(&self) -> NumberFormatTypeRef {
        &vanilla_number_format_types::BLANK
    }
}