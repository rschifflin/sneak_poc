use ECS;
use pubsub::{Pubsub, Event};
use events::{EventChannel, EventPayload};

pub type PubsubECS<'a> = Pubsub<'a, ECS, EventChannel, EventPayload>;
pub type EventVec = Vec<Event<EventChannel, EventPayload>>;
