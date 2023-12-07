use bevy_ecs::event::Event;


#[derive(Event)]
pub enum AudioEvent {
    Destroyed,
    Bounce,
}
