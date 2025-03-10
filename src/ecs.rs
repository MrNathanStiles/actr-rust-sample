pub mod coordinator;
pub mod component_manager;
pub mod component_array;
pub mod entity_manager;
pub mod signature;
pub mod system;
pub mod system_manager;

pub type Entity = usize;

pub const MAX_ENTITIES: Entity = 4096;
