use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
}
