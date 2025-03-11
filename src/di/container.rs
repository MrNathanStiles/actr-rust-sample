use std::{any::{Any, TypeId}, collections::HashMap, marker::PhantomData, sync::{Arc, LazyLock, Mutex}};


static SERVICES: LazyLock<Mutex<HashMap<TypeId, Arc<Mutex<Box<dyn Any + Send>>>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
pub struct Container;

pub struct Holder<T> {
    reference: Box<T>,
    phantom: PhantomData<T>
}


impl Container {
    pub fn new() -> Container {
        Container
    }
    pub fn register_service<T>(&mut self, service: T)
    where
        T: Send + 'static,
    {
        let id = TypeId::of::<T>();
        let mut services = SERVICES.lock().unwrap();
        if services.contains_key(&id) {
            return;
        }
        services.insert(id, Arc::new(Mutex::new(Box::new(service))));
    }
    pub fn get_service<T>(&self) -> Arc<Mutex<Box<dyn Any + Send>>>
    where
        T: 'static,
    {
        let id = TypeId::of::<T>();
        let services = SERVICES.lock().unwrap();
        
        let value = services.get(&id).unwrap().clone();
        value
        
    }
}
