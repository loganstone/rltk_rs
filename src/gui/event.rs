pub struct Event {
    pub widget : String,
    pub event_type : EventType
}

impl Event {
    pub fn new(id: &str, t : EventType) -> Event {
        Event {
            widget : id.to_string(),
            event_type : t
        }
    }
}

pub enum EventType {
    MouseOver, Clicked
}