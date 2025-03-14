use std::collections::BTreeMap;

#[derive(Ord, Eq, PartialOrd, PartialEq)]
pub enum Component {
    Physics,
    Shape,
    Render,
}

#[derive(PartialEq)]
pub enum State {
    Idle,
    Walking,
    Jumping,
}

pub struct Entity {
    component_idx: BTreeMap<Component, usize>,
    current_state: State,
}

#[derive(Default)]
pub struct EntityBuilder {
    component_idx: BTreeMap<Component, usize>,
}

impl Entity {
    pub fn builder() -> EntityBuilder {
        EntityBuilder::default()
    }

    pub fn get_index_for(&self, comp: Component) -> Option<usize> {
        self.component_idx.get(&comp).copied()
    }

    pub fn update_state(&mut self, state: State) -> bool {
        // TODO:
        if self.current_state == State::Jumping && state == State::Jumping {
            return false;
        }

        self.current_state = state;
        true
    }
}

impl EntityBuilder {
    pub fn new() -> Self {
        Self {
            component_idx: BTreeMap::<Component, usize>::default(),
        }
    }

    pub fn with_physics_component(mut self, idx: usize) -> Self {
        self.component_idx.insert(Component::Physics, idx);
        self
    }

    pub fn with_render_component(mut self, idx: usize) -> Self {
        self.component_idx.insert(Component::Render, idx);
        self
    }

    pub fn with_shape_component(mut self, idx: usize) -> Self {
        self.component_idx.insert(Component::Shape, idx);
        self
    }

    pub fn build(self) -> Entity {
        Entity {
            component_idx: self.component_idx,
            // A new entity is always idle.
            current_state: State::Idle,
        }
    }
}

#[test]
fn entity_builder_test() {
    let entity = EntityBuilder::new()
        .with_physics_component(123)
        .with_render_component(456)
        .build();
    assert_eq!(123, entity.get_index_for(Component::Physics).unwrap());
    assert_eq!(456, entity.get_index_for(Component::Render).unwrap());
}
