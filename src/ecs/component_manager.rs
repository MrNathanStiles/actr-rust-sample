use super::{component_array::{ComponentArray, ThingTrait}, Entity};
use std::{any::{Any, TypeId}, collections::HashMap, ops::DerefMut};

pub type ComponentType = u64;
// pub const MAX_COMPONENTS: ComponentType = 64;

pub struct ComponentManager {
    component_array: HashMap<TypeId, Box<dyn ThingTrait>>,
    component_types: HashMap<TypeId, ComponentType>,
    next_component_type: ComponentType,
}

impl ComponentManager {
    pub fn new() -> ComponentManager {
        ComponentManager {
            component_array: HashMap::new(),
            component_types: HashMap::new(),
            next_component_type: 1,
        }
    }

    pub fn register_component<T>(&mut self)
    where
        T: 'static + Send + Clone,
    {
        let id = TypeId::of::<T>();
        let mut ca = ComponentArray::<T>::new();
        self.component_array.insert(id, Box::new(ca));

        self.component_types.insert(id, self.next_component_type);
        self.next_component_type += 1;

    }

    pub fn get_component_type<T>(&self) -> ComponentType
    where
        T: 'static,
    {
        let id = &TypeId::of::<T>();
        *self.component_types.get(id).unwrap()
    }

    pub fn add_component<T>(&mut self, entity: Entity, component: T)
    where
        T: 'static + Send,
    {
        let array = self.get_array_mut::<T>();
        array.insert_data(entity, component);
    }

    fn get_array_mut<T>(&mut self) -> &mut ComponentArray<T>
    where 
        T: 'static
    {
        let id = &TypeId::of::<T>();
        let boxed = self.component_array.get_mut(id).unwrap();
        let result = boxed.as_any_mut();
        result.downcast_mut::<ComponentArray<T>>().unwrap()
    }

    fn get_array<T>(&self) -> &ComponentArray<T>
    where 
        T: 'static
    {
        let id = &TypeId::of::<T>();
        let boxed = self.component_array.get(id).unwrap();
        let result = boxed.as_any();
        result.downcast_ref::<ComponentArray<T>>().unwrap()
    }

    fn get_thing<T>(&mut self) -> &mut Box<dyn ThingTrait>
    where 
        T: 'static
    {
        let id = &TypeId::of::<T>();
        self.component_array.get_mut(id).unwrap()
    }

    
    pub fn remove_component<T>(&mut self, entity: Entity)
    where
        T: 'static + Send + Clone,
    {
        let array = self.get_array_mut::<T>();
        array.remove_data(entity);
    }

    pub fn get_component<T>(&mut self, entity: Entity) -> &mut T
    where
        T: 'static + Send,
    {
        let array = self.get_array_mut::<T>();
        
        array.get_component(entity)
    }

    pub fn entity_destroyed(&mut self, entity: Entity) {
        for (_id, component) in self.component_array.iter_mut() {
            component.entity_destroyed(entity);
        }
    }
}
