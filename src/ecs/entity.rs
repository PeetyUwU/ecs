use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Entity {
    pub id: u32,
    pub generation: u32,
}

pub struct EntityManager {
    next_id: u32,
    generations: Vec<u32>,
    free_ids: VecDeque<u32>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            generations: Vec::new(),
            free_ids: VecDeque::new(),
        }
    }

    pub fn create(&mut self) -> Entity {
        let id = if let Some(free_id) = self.free_ids.pop_front() {
            free_id
        } else {
            let id = self.next_id;
            self.next_id += 1;
            self.generations.push(0);
            id
        };

        Entity {
            id,
            generation: self.generations[id as usize],
        }
    }

    pub fn destroy(&mut self, entity: Entity) {
        if !self.is_alive(&entity) {
            return;
        }
        self.generations[entity.id as usize] += 1;
        self.free_ids.push_back(entity.id);
    }

    pub fn is_alive(&self, entity: &Entity) -> bool {
        let idx = entity.id as usize;
        self.generations.len() > idx && self.generations[idx] == entity.generation
    }
}
