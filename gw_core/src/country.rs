use serde::Serialize;

#[derive(Serialize, Clone, Copy, PartialEq, Debug, Default)]
pub enum Country {
    #[default]
    GB,
    US,
}
