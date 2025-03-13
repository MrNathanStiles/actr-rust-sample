use std::{any::TypeId, collections::HashMap};

pub struct Container {}

static mut SERVICE_MAP: usize = 0; //Container::to_generic_pointer(HashMap::<TypeId, *mut u8>::new());

impl Container {
    pub fn initialize() {
        unsafe {
            if SERVICE_MAP == 0 {
                let hash_map: HashMap<TypeId, usize> = HashMap::new();
                SERVICE_MAP = Container::to_generic_pointer(hash_map);
            }
        }
    }
    pub fn new() -> Container {
        Container {}
    }

    pub fn to_generic_pointer<T>(thing: T) -> usize
    where
        T: 'static,
    {
        Box::into_raw(Box::new(thing)) as usize
    }

    pub fn from_generic_pointer<T>(generic_pointer: usize) -> &'static mut T
    where
        T: 'static,
    {
        let pointer = unsafe { *(generic_pointer as *mut u8) as *mut T };
        unsafe { &mut *pointer }
    }

    fn get_services(&self) -> &'static mut HashMap<TypeId, usize> {
        unsafe { Container::from_generic_pointer::<HashMap<TypeId, usize>>(SERVICE_MAP) }
    }

    pub fn register_service<T>(&mut self, service: T)
    where
        T: 'static,
    {
        let id = TypeId::of::<T>();
        let services = self.get_services();

        if services.contains_key(&id) {
            return;
        }
        
        services.insert(id, Container::to_generic_pointer(service));
    }

    pub fn get_service<T>(&self) -> &mut T
    where
        T: 'static,
    {
        let id = TypeId::of::<T>();
        let services = self.get_services();
        let ptr = services.get(&id).unwrap();
        let raw = *ptr as *mut T;
        unsafe { &mut *raw }
    }
}
