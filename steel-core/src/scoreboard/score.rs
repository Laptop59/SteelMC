use text_components::TextComponent;
use crate::scoreboard::number_format::NumberFormat;

pub struct Score {
    value: i32,
    locked: bool,
    display: Option<Box<TextComponent>>,
    number_format: Option<Box<dyn NumberFormat>>
}