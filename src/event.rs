// event.rs

use alloy_primitives::U256;

/// Estructura para representar un evento con `id` y `starttime`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: u8,
    pub starttime: U256,
}

impl Event {
    /// Crea un nuevo evento con `id` y `starttime`
    pub fn new(id: u8, starttime: U256) -> Self {
        Event { id, starttime }
    }
}
