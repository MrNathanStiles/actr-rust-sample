use std::{any::TypeId, collections::HashMap};

pub struct Container {}

static mut GENERIC_POINTER: usize = 0; //Container::to_generic_pointer(HashMap::<TypeId, *mut u8>::new());

impl Container {
    pub fn initialize() {
        unsafe {
            if GENERIC_POINTER == 0 {
                let hash_map: HashMap<TypeId, *mut u8> = HashMap::<TypeId, *mut u8>::new();
                GENERIC_POINTER = Container::to_generic_pointer(hash_map) as usize;
            }
        }
    }
    pub fn new() -> Container {
        Container {}
    }

    fn to_generic_pointer<T>(thing: T) -> *mut u8
    where
        T: 'static,
    {
        let bx = Box::new(thing);
        Box::into_raw(bx) as *mut u8
    }

    fn from_generic_pointer<T>(generic_pointer: usize) -> &'static mut T
    where
        T: 'static,
    {
        let pointer = unsafe { *(generic_pointer as *mut u8) as *mut T };
        unsafe { &mut *pointer }
    }

    fn get_services(&self) -> &'static mut HashMap<TypeId, *mut u8> {
        unsafe { Container::from_generic_pointer::<HashMap<TypeId, *mut u8>>(GENERIC_POINTER) }
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
        let bx = Box::new(service);
        let raw = Box::into_raw(bx) as *mut u8;
        services.insert(id, raw);
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
