pub mod actr;
pub mod component;
pub mod di;
pub mod ecs;

use actr::{
    _actr_log_length, _actr_three_init, actr_performance, actr_prng, actr_three_camera_perspective, actr_three_geometry_box, actr_three_material_standard, actr_three_mesh, actr_three_object_add, actr_three_object_lookat, actr_three_object_position, actr_three_object_remove, actr_three_render, actr_three_scene, actr_three_transform_buffer, actr_three_transform_read
};
use component::{
    gravity::Gravity, rigid_body::RigidBody, transform::Transform, vector3::Vector3,
};
use di::container::Container;
use ecs::{
    Entity, MAX_ENTITIES, component_manager::ComponentManager, coordinator::Coordinator,
    entity_manager::EntityManager, signature::Signature, system::System,
    system_manager::SystemManager,
};
use std::{
    alloc::{self, alloc, Layout},
    collections::HashSet, panic::{self, PanicHookInfo},
};

fn init_systems() {
    register_sample_system();
    register_mesh_transformer_system();
}
fn update_sample_system(entities: &HashSet<Entity>, delta: f64) {
    let container = Container::new();
    let component_manager = container.get_service::<ComponentManager>();
    let mut remove = HashSet::new();

    for entity in entities {

        let transform = component_manager.get_component::<Transform>(*entity);
        let gravity = component_manager.get_component::<Gravity>(*entity);
        let rigid_body = component_manager.get_component::<RigidBody>(*entity);

        rigid_body.velocity += gravity.direction * delta;
        
        transform.position += rigid_body.velocity * delta;
        transform.rotation += rigid_body.angularVelocity * delta;

        if transform.position.y < -200.0 {
            remove.insert(entity);
        }
    }

    if remove.len() > 0 {
        let three_info = container.get_service::<ThreeInfo>();
        let coordinator = container.get_service::<Coordinator>();
        for entity in remove {
            let three_transform_buffer_index = component_manager.get_component::<ThreeTransformBufferIndex>(*entity);
            unsafe {
                actr_three_object_remove(three_info.scene, three_transform_buffer_index.mesh);
            }
            coordinator.destroy_entity(*entity);
            add_entity(&three_info);
        }
        update_transform_buffer();
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
    signature.set(component_manager.get_component_type::<ThreeTransformBufferIndex>());
    let sample_system = System::new(signature, update_sample_system);
    coordinator.register_system(sample_system);
}

fn update_mesh_transformer_system(entities: &HashSet<Entity>, delta: f64) {}
fn register_mesh_transformer_system() {
    let container = Container::new();
    let cm = container.get_service::<ComponentManager>();
    let coor = container.get_service::<Coordinator>();

    let mut sig = Signature::zero();
    sig.set(cm.get_component_type::<Transform>());
    let sample_system = System::new(sig, update_mesh_transformer_system);
    coor.register_system(sample_system);
}

fn update_transform_buffer() {
    let container = Container::new();
    let cm = container.get_service::<ComponentManager>();
    let three_info = container.get_service::<ThreeInfo>();
    
    unsafe {
        actr_three_transform_buffer(
            three_info.transform_index_address,
            three_info.transform_buffer_address,
            48,
            cm.compoennt_count::<Transform>(),
        );
    }
}

fn add_entity(bi: &ThreeInfo) {
    let container = Container::new();
    let coor = container.get_service::<Coordinator>();
    let cm = container.get_service::<ComponentManager>();
    let three_info = container.get_service::<ThreeInfo>();
    
    let mut material = bi.material_red;
    let mat = unsafe { actr_prng() * 3.0 };

    if mat < 1.0 {
        material = bi.material_green;
    } else if mat < 2.0 {
        material = bi.material_blue;
    }
    
    let mesh = unsafe { actr_three_mesh(bi.geometry, material) };

    let entity = coor.create_entity();
    let x = unsafe { actr_prng() * 500.0 - 250.0 };
    let z = unsafe { actr_prng() * 500.0 - 250.0 };
    let position = Vector3::new(x, 500.0, z);
    let x = unsafe { actr_prng() - 0.5 };
    let y = unsafe { actr_prng() - 0.5 };
    let z = unsafe { actr_prng() - 0.5 };
    
    let rotation = Vector3::new(x, y, z);
    coor.add_component(entity, Transform::new(position, Vector3::zero()));
    coor.add_component(entity, Gravity::new());
    coor.add_component(entity, RigidBody::new(Vector3::zero(), Vector3::new(x, y, z)));
    coor.add_component(entity, ThreeTransformBufferIndex { mesh });

    unsafe {
        actr_three_object_add(bi.scene, mesh);
    }

    
}


fn add_floor(bi: &ThreeInfo) {
    let container = Container::new();
    let coordinator = container.get_service::<Coordinator>();
    
    let material = unsafe {
        actr_three_material_standard(0xffffff, 0xffffff, false, 1.0, false, true)
    };
    
    let geometry = unsafe {actr_three_geometry_box(1000.0, 0.1, 1000.0)};

    let mesh = unsafe { actr_three_mesh(geometry, material) };

    let entity = coordinator.create_entity();
    log(format!("floor entity {entity} mesh {mesh}"));
    coordinator.add_component(entity, Transform::new(Vector3::new(0.0, -200.0, 0.0), Vector3::zero()));
    coordinator.add_component(entity, ThreeTransformBufferIndex { mesh });

    unsafe {
        actr_three_object_add(bi.scene, mesh);
    }
}

struct ThreeTransformBufferIndex {
    mesh: i32,
}

struct ThreeInfo {
    pub transform_buffer_address: usize,
    pub transform_index_address: usize,
    pub material_red: i32,
    pub material_green: i32,
    pub material_blue: i32,
    pub geometry: i32,
    pub scene: i32,
    pub camera: i32,
    pub camera_state: f64,
}

fn init_services() {
    let container = Container::new();
    container.register_service(EntityManager::new());
    container.register_service(ComponentManager::new());
    container.register_service(SystemManager::new());
    container.register_service(Coordinator::new());
}

fn init_components() -> (usize, usize) {
    let container = Container::new();
    let coor = container.get_service::<Coordinator>();
    coor.register_component::<Gravity>();
    coor.register_component::<RigidBody>();
    let transform_buffer_address = coor.register_component::<Transform>();
    let transform_index_address  = coor.register_component::<ThreeTransformBufferIndex>();
    (transform_buffer_address, transform_index_address)
}

fn init_materials() -> (i32, i32, i32) {
    unsafe {
        let red = actr_three_material_standard(0xff0000, 0xff0000, false, 1.0, false, true);
        let green = actr_three_material_standard(0x00ff00, 0x00ff00, false, 1.0, false, true);
        let blue = actr_three_material_standard(0x0000ff, 0x0000ff, false, 1.0, false, true);

        (red, green, blue)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn actr_init(_w: i32, _h: i32) {
    
    panic::set_hook(Box::new(|info| {
        log(info.to_string());    
    }));
    let container = Container::initialize();
    
    init_services();
    let (transform_buffer_address, transform_index_address) = init_components();

    let mut camera = 0;
    unsafe {
        _actr_three_init(0, 0);
        camera = actr_three_camera_perspective(90.0, 0.1, 10000.1);
    }

    let geometry = unsafe { actr_three_geometry_box(10.0, 10.0, 10.0) };

    let (material_red, material_green, material_blue) = init_materials();


    let scene = unsafe { actr_three_scene() };

    container.register_service(ThreeInfo {
        material_red,
        material_green,
        material_blue,
        transform_buffer_address,
        transform_index_address,
        geometry,
        scene,
        camera,
        camera_state: 0.0
    });
    let three_info = container.get_service::<ThreeInfo>();
    init_systems();

    add_floor(&three_info);
    for _n in 0..1024 {
        add_entity(&three_info);
    }
    

    unsafe {
        actr_three_object_position(three_info.camera, 0.0, 0.0, 400.0);
        actr_three_object_lookat(three_info.camera, 0.0, 0.0, 0.0);
    }
    
    
    update_transform_buffer();

}

#[unsafe(no_mangle)]
pub extern "C" fn actr_step(delta: f64) {
    log(format!("stepping {delta}"));
    let container = Container::new();
    let coor = container.get_service::<Coordinator>();
    let three_info = container.get_service::<ThreeInfo>();


    let dist = 500.0;
    three_info.camera_state += delta * 0.1;
    unsafe {
        actr_three_object_position(three_info.camera, three_info.camera_state.cos() * dist, 0.0, three_info.camera_state.sin() * dist);
        actr_three_object_lookat(three_info.camera, 0.0, 0.0, 0.0);
    }
    
    //let start = unsafe { actr_performance() };

    coor.update(delta);

    //let end = unsafe { actr_performance() };

    unsafe {
        actr_three_transform_read();
        actr_three_render();
    }

    //let time = end - start;

    //log(format!("time {time}"));
}

pub fn log(message: String) {
    unsafe {
        _actr_log_length(message.as_ptr(), message.len());
    }
}
