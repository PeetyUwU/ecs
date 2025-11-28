use std::any::{Any, TypeId};
use std::collections::HashMap;

#[derive(Default)]
pub struct EventBus {
    events: HashMap<TypeId, Vec<Box<dyn Any>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push<E: Any>(&mut self, event: E) {
        self.events
            .entry(TypeId::of::<E>())
            .or_insert_with(Vec::new)
            .push(Box::new(event));
    }

    pub fn iter<E: Any>(&self) -> impl Iterator<Item = &E> {
        self.events
            .get(&TypeId::of::<E>())
            .into_iter()
            .flat_map(|vec| vec.iter())
            .filter_map(|b| b.downcast_ref::<E>())
    }

    pub fn clear(&mut self) {
        for events in self.events.values_mut() {
            events.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct DamageEvent {
        entity_id: u32,
        amount: i32,
    }

    #[test]
    fn test_event_bus() {
        let mut bus = EventBus::new();
        bus.push(DamageEvent {
            entity_id: 1,
            amount: 10,
        });
        bus.push(DamageEvent {
            entity_id: 2,
            amount: 20,
        });

        let events: Vec<&DamageEvent> = bus.iter::<DamageEvent>().collect();
        assert_eq!(events.len(), 2);
        assert_eq!(
            events[0],
            &DamageEvent {
                entity_id: 1,
                amount: 10
            }
        );
        assert_eq!(
            events[1],
            &DamageEvent {
                entity_id: 2,
                amount: 20
            }
        );

        bus.clear();
        let events_after_clear: Vec<&DamageEvent> = bus.iter::<DamageEvent>().collect();
        assert!(events_after_clear.is_empty());
    }
}
