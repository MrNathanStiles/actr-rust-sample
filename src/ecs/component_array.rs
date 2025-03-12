use std::{any::Any, collections::HashMap};

use as_any::AsAny;

use super::{Entity, MAX_ENTITIES};

pub struct ComponentArray<T: 'static>
{
    components: Vec<T>,
    entity_to_index: HashMap<Entity, usize>,
    index_to_entity: HashMap<usize, Entity>,
    component_count: usize,
}

pub trait ThingTrait: Any + AsAny + Send
{
    fn entity_destroyed(&mut self, entity: Entity);
    fn remove_data(&mut self, entity: Entity);
}

impl<T: 'static> ThingTrait for ComponentArray<T>
where 
    T: 'static + Send + Clone
{
    fn entity_destroyed(&mut self, entity: Entity)
    where 
        T: 'static + Clone
    {
        if self.entity_to_index.contains_key(&entity) {
            self.remove_data(entity);
        }
    }

    fn remove_data(&mut self, entity: Entity)
    where 
        T: 'static + Clone
    {

        let index_removed = *self.entity_to_index.get(&entity).unwrap();

        self.component_count -= 1;
        let index_last = self.component_count;

        if index_removed == index_last {
            return;
        }

        let entity_last = *self.index_to_entity.get(&index_last).unwrap();

        self.entity_to_index.insert(entity_last, index_removed);
        self.index_to_entity.insert(index_removed, entity_last);

        

        self.components[index_removed] = self.components[index_last].clone();
    }

}

impl<T> ComponentArray<T>
{
    pub fn new() -> ComponentArray<T>
    where
        T: Send + 'static,
    {
        ComponentArray {
            components: Vec::<T>::with_capacity(MAX_ENTITIES),
            entity_to_index: HashMap::new(),
            index_to_entity: HashMap::new(),
            component_count: 0,
        }
    }

    pub fn insert_data(&mut self, entity: Entity, cmp: T)
    {
        let index = self.component_count;
        self.entity_to_index.insert(entity, index);
        self.index_to_entity.insert(index, entity);


        self.components[index] = cmp;

        self.component_count += 1;
    }

    pub fn get_component(&mut self, entity: Entity) -> &mut T
    where
        T: 'static,
    {
        let index = self.entity_to_index.get(&entity).unwrap();
        let pentity = self.index_to_entity.get(index).unwrap();
        if *pentity != entity {
            panic!("entity {entity} index {index} pentity {pentity}")
        }
        self.components.get_mut(*index).unwrap()
        
    }




}
