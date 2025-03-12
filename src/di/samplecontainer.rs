use parking_lot::{MappedMutexGuard, Mutex, MutexGuard};
use std::any::Any;
use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::LazyLock;
use std::{any::TypeId, collections::HashMap};

pub struct Container;

pub struct Holder<T>(Arc<Mutex<Box<dyn Any + Send>>>, PhantomData<T>);

impl<T: 'static> Holder<T> {
    fn lock(&self) -> MappedMutexGuard<'_, T> {
        MutexGuard::map(self.0.lock(), |x| x.downcast_mut().unwrap())
    }
}

static MAP: LazyLock<Mutex<HashMap<TypeId, Arc<Mutex<Box<dyn Any + Send>>>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

impl Container {
    pub fn new() -> Container {
        Container
    }

    pub fn register_service<T>(&mut self, service: T)
    where
        T: Send + 'static,
    {
        let id = TypeId::of::<T>();

        let mut map = MAP.lock();

        if map.contains_key(&id) {
            return;
        }

        map.insert(id, Arc::new(Mutex::new(Box::new(service))));
    }

    pub fn get_service<'a, T>(&self) -> Holder<T>
    where
        T: Send + 'static,
    {
        let id = TypeId::of::<T>();

        let map = MAP.lock();

        let entry = map.get(&id).unwrap();

        Holder(entry.clone(), PhantomData)
    }
}

fn main() {
    let value = vec![1, 2, 3];

    let mut container = Container::new();
    container.register_service(value);

    let x = container.get_service::<Vec<i32>>();
    let mut y = x.lock();

    println!("{y:?}");
    y.extend([4, 5, 6, 7, 8, 9, 10]);
    println!("{y:?}");
}