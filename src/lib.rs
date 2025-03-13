mod actr;
mod component;
mod di;
mod ecs;

use actr::{
    _actr_log_length, _actr_three_init, actr_performance, actr_prng, actr_three_camera_perspective, actr_three_geometry_box, actr_three_material_standard, actr_three_mesh, actr_three_object_add, actr_three_object_remove, actr_three_render, actr_three_scene, actr_three_transform_buffer, actr_three_transform_read
};
use component::{
    gravity::Gravity, identity::Identity, rigid_body::RigidBody, transform::Transform,
};
use di::container::Container;
use ecs::{
    Entity, MAX_ENTITIES, component_manager::ComponentManager, coordinator::Coordinator,
    entity_manager::EntityManager, signature::Signature, system::System,
    system_manager::SystemManager,
};
use std::{
    alloc::{self, Layout, alloc},
    collections::HashSet,
};

fn update_sample_system(entities: &HashSet<Entity>, delta: f64) {
    let container = Container::new();
    let component_manager = container.get_service::<ComponentManager>();
    let mut remove = HashSet::new();

    let mut entity_max: usize = 0;
    let mut entity_min: usize = 9999;
    let mut count = 0;

    let mut hash = HashSet::new();

    for entity in entities {
        hash.insert(*entity);
        entity_max = usize::max(*entity, entity_max);
        entity_min = usize::min(*entity, entity_min);

        let transform = component_manager.get_component::<Transform>(*entity);
        let gravity = component_manager.get_component::<Gravity>(*entity);
        let rigid_body = component_manager.get_component::<RigidBody>(*entity);
        

        if transform.position.y < 0.0 {
            //log(format!("entity should have been dropped"));
            //panic!("entity should have been dropped");
        }
        rigid_body.velocity += gravity.direction * delta;
        transform.position += rigid_body.velocity * delta;

        if transform.position.y < -100.0 {
            

            remove.insert(entity);
        }
    }

    let count = hash.len();
    if entity_max < 4095 {
        //log(format!("entity max {entity_max}"));
        //panic!("entity max {entity_max}");
    }

    if entity_min > 0 {
        log(format!("entity min {entity_max}"));
        panic!("entity min {entity_max}");
    }

    if count < 4095 {
        //log(format!("entity count {count}"));
        //panic!("entity count {count}");
    }
    //log(format!("entity count {count}"));
    if remove.len() > 0 {
        let mut bi = container.get_service::<BufferInfo>();
        let coordinator = container.get_service::<Coordinator>();
        for entity in remove {
            let identity = component_manager.get_component::<Identity>(*entity);
            unsafe {actr_three_object_remove(bi.scene, identity.mesh);}
            coordinator.destroy_entity(*entity);
            add_entity(&mut bi);
        }
    }
}

fn register_sample_system() {
    let container = Container::new();
    let component_manager = container.get_service::<ComponentManager>();
    let coordinator = container.get_service::<Coordinator>();

    let mut signature = Signature::zero();
    signature.set(component_manager.get_component_type::<Transform>());
    signature.set(component_manager.get_component_type::<Gravity>());
    signature.set(component_manager.get_component_type::<RigidBody>());
    signature.set(component_manager.get_component_type::<Identity>());
    let sample_system = System::new(signature, update_sample_system);
    coordinator.register_system(sample_system);
}

fn add_entity(bi: &mut BufferInfo) {
    let container = Container::new();
    let coordinator = container.get_service::<Coordinator>();

    let entity = coordinator.create_entity();
    let x = unsafe { actr_prng() * 1000.0 - 500.0 };
    let z = unsafe { actr_prng() * 1000.0 - 500.0 };
    coordinator.add_component(entity, Transform::new(x, 100.0, z));
    coordinator.add_component(entity, Gravity::new());
    coordinator.add_component(entity, RigidBody::new());

    let mesh = unsafe { actr_three_mesh(bi.geometry, bi.material) };
    coordinator.add_component(entity, Identity { mesh, entity });

    unsafe {
        let ptr = bi.index_address.add(entity * 4) as *mut i32;
        *ptr = mesh;
    }

    unsafe {
        actr_three_object_add(bi.scene, mesh);
    }
}
struct BufferInfo {
    pub buffer_address: usize,
    pub index_address: *mut u8,
    pub material: i32,
    pub geometry: i32,
    pub scene: i32,
}
#[unsafe(no_mangle)]
pub extern "C" fn actr_init(_w: f32, _h: f32) {
    unsafe {
        _actr_three_init(0, 0);
        actr_three_camera_perspective(90.0, 0.1, 10000.1);
    }
    Container::initialize();
    let mut container = Container::new();
    container.register_service(EntityManager::new());
    container.register_service(ComponentManager::new());
    container.register_service(SystemManager::new());
    container.register_service(Coordinator::new());

    let coordinator = container.get_service::<Coordinator>();
    let buffer_address = coordinator.register_component::<Transform>();

    let layout = Layout::array::<i32>(MAX_ENTITIES).unwrap();
    let index_address = unsafe { alloc::alloc_zeroed(layout) };

    let geometry = unsafe { actr_three_geometry_box(1.0, 1.0, 1.0) };
    let material =
        unsafe { actr_three_material_standard(0xffffff, 0xffffff, false, 1.0, false, true) };

        let scene = unsafe { actr_three_scene()};
    let mut bi = BufferInfo {
        buffer_address,
        index_address,
        material,
        geometry,
        scene,
    };

    for i in 0..MAX_ENTITIES {
        unsafe {
            let ptr = bi.index_address.add(i * 4) as *mut i32;
            *ptr = 0;
        }
    }

    coordinator.register_component::<Gravity>();
    coordinator.register_component::<RigidBody>();
    coordinator.register_component::<Identity>();

    register_sample_system();

    for _n in 0..MAX_ENTITIES {
        add_entity(&mut bi);
    }
    unsafe {
        actr_three_transform_buffer(bi.index_address as usize, bi.buffer_address, 48, MAX_ENTITIES);
    }
    container.register_service(bi);
}

#[unsafe(no_mangle)]
pub extern "C" fn actr_step(delta: f64) {
    let container = Container::new();
    let coordinator = container.get_service::<Coordinator>();

    let start = unsafe { actr_performance() };

    coordinator.update(delta);


    let end = unsafe { actr_performance() };

    unsafe {
        actr_three_transform_read();
        actr_three_render();
    }

    let time = end - start;

    log(format!("time {time}"));
}

pub fn log(message: String) {
    unsafe {
        _actr_log_length(message.as_ptr(), message.len());
    }
}
