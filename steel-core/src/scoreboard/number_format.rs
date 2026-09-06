use serde::{Deserialize, Serialize};
use std::io::{Cursor, Write};
use steel_registry::number_format_type::{NumberFormatTypeRef, vanilla_number_format_types};
use steel_utils::serial::{ReadFrom, WriteTo};
use text_components::TextComponent;
use text_components::format::Format;

/// A trait that number formats implement.
pub trait NumberFormat<'de>: WriteTo + ReadFrom + Serialize + Deserialize<'de> {
    fn format_type(&self) -> NumberFormatTypeRef;
}

/// Does not display anything, leaving it as a blank.
#[derive(Serialize, Deserialize)]
pub struct BlankFormat;

/// The number is displayed with a given style.
#[derive(Serialize, Deserialize)]
pub struct StyledFormat(pub Format);

/// Always displays a fixed text component instead of the number provided.
#[derive(Serialize, Deserialize)]
pub struct FixedFormat(pub TextComponent);

impl WriteTo for BlankFormat {
    fn write(&self, _writer: &mut impl Write) -> std::io::Result<()> {
        Ok(())
    }
}

impl ReadFrom for BlankFormat {
    fn read(_data: &mut Cursor<&[u8]>) -> std::io::Result<Self> {
        Ok(BlankFormat)
    }
}

impl NumberFormat<'_> for BlankFormat {
    fn format_type(&self) -> NumberFormatTypeRef {
        &vanilla_number_format_types::BLANK
    }
}

impl WriteTo for StyledFormat {
    fn write(&self, writer: &mut impl Write) -> std::io::Result<()> {
        self.0.write(writer)
    }
}

impl ReadFrom for StyledFormat {
    fn read(data: &mut Cursor<&[u8]>) -> std::io::Result<Self> {
        Format::read(data).map(StyledFormat)
    }
}

impl NumberFormat<'_> for StyledFormat {
    fn format_type(&self) -> NumberFormatTypeRef {
        &vanilla_number_format_types::STYLED
    }
}

impl WriteTo for FixedFormat {
    fn write(&self, writer: &mut impl Write) -> std::io::Result<()> {
        self.0.write(writer)
    }
}

impl ReadFrom for FixedFormat {
    fn read(data: &mut Cursor<&[u8]>) -> std::io::Result<Self> {
        TextComponent::read(data).map(Self)
    }
}

impl NumberFormat<'_> for FixedFormat {
    fn format_type(&self) -> NumberFormatTypeRef {
        &vanilla_number_format_types::FIXED
    }
}
