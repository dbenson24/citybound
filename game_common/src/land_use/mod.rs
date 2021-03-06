use kay::{ActorSystem};

pub mod buildings;
pub mod vacant_lots;
pub mod construction;
pub mod zone_planning;

pub fn setup(system: &mut ActorSystem) {
    buildings::setup(system);
    vacant_lots::setup(system);
}
