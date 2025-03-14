use std::ffi::{c_char, c_void};
pub enum StringEncoding {
	ASCII, UTF8, UTF16
}
// when passing string to the platform you typically pass the pointer and the string length
// for example
// returns 1 if current user is authenticated, meaning your program can persist data and use other required authentication features
unsafe extern {
	// actr_performance
	// expose browsers performance.now()

	pub unsafe fn actr_performance() -> f32;

	// abort
	// this is specific to assembly script
	// @param message
	// @param fileName
	// @param line
	// @param column

	pub unsafe fn abort(message: *const u8, fileName: *const u8, line: i32, column: i32);

	// actr_authenticated

	pub unsafe fn actr_authenticated() -> bool;

	// actr_2d_init
	// canvas methods should match https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D
	// canvas2d methods should be avoided if using three.js support
	// init canvas context 2d must be called before other canvas methods

	pub unsafe fn actr_2d_init() -> bool;

	// actr_canvas2d_fill_style
	// set fill style rgb are 0 to 255 a is 0 to 100
	// @param r
	// @param g
	// @param b
	// @param a

	pub unsafe fn actr_canvas2d_fill_style(r: u8, g: u8, b: u8, a: u8);

	// actr_canvas2d_stroke_style
	// @param r
	// @param g
	// @param b
	// @param a

	pub unsafe fn actr_canvas2d_stroke_style(r: u8, g: u8, b: u8, a: u8);

	// actr_canvas2d_begin_path

	pub unsafe fn actr_canvas2d_begin_path();

	// actr_canvas2d_arc
	// @param x
	// @param y
	// @param radius
	// @param startAngle
	// @param endAngle
	// @param counterclockwise

	pub unsafe fn actr_canvas2d_arc(x: f32, y: f32, radius: f32, startAngle: f32, endAngle: f32, counterclockwise: bool);

	// actr_canvas2d_arcTo
	// @param x1
	// @param y1
	// @param x2
	// @param y2
	// @param radius

	pub unsafe fn actr_canvas2d_arcTo(x1: f32, y1: f32, x2: f32, y2: f32, radius: f32);

	// actr_canvas2d_close_path

	pub unsafe fn actr_canvas2d_close_path();

	// actr_canvas2d_ellipse
	// @param x
	// @param y
	// @param radiusX
	// @param radiusY
	// @param rotation
	// @param startAngle
	// @param endAngle
	// @param counterclockwise

	pub unsafe fn actr_canvas2d_ellipse(x: f32, y: f32, radiusX: f32, radiusY: f32, rotation: f32, startAngle: f32, endAngle: f32, counterclockwise: bool);

	// actr_canvas2d_fill

	pub unsafe fn actr_canvas2d_fill();

	// actr_canvas2d_fill_rect
	// @param x
	// @param y
	// @param w
	// @param h

	pub fn actr_canvas2d_fill_rect(x: f32, y: f32, w: f32, h: f32);

	// _actr_canvas2d_fill_text_length
	// @param x
	// @param y
	// @param ptr
	// @param length

	pub fn _actr_canvas2d_fill_text_length(x: f32, y: f32, ptr: *const u8, length: i32);

	// actr_canvas2d_fill_gradient_all
	// fill a gradient of many colors useful for color picker
	// @param x
	// @param y
	// @param w
	// @param h

	pub fn actr_canvas2d_fill_gradient_all(x: f32, y: f32, w: f32, h: f32);

	// actr_canvas2d_fill_gradient_pick
	// fill gradient of specific color useful for color picker
	// @param x
	// @param y
	// @param w
	// @param h
	// @param r
	// @param g
	// @param b

	pub fn actr_canvas2d_fill_gradient_pick(x: f32, y: f32, w: f32, h: f32, r: u8, g: u8, b: u8);

	// actr_canvas2d_pick
	// returns the color at the specified position
	// @param x
	// @param y

	pub fn actr_canvas2d_pick(x: f32, y: f32) -> u32;

	// actr_canvas2d_moveto
	// @param x
	// @param y

	pub fn actr_canvas2d_moveto(x: f32, y: f32);

	// actr_canvas2d_lineto
	// @param x
	// @param y

	pub fn actr_canvas2d_lineto(x: f32, y: f32);

	// actr_canvas2d_stroke

	pub fn actr_canvas2d_stroke();

	// actr_canvas2d_stroke_rect
	// @param x
	// @param y
	// @param w
	// @param h

	pub fn actr_canvas2d_stroke_rect(x: f32, y: f32, w: f32, h: f32);

	// actr_debugger
	// pauses execution in the browser when in debug mode
	// @param value

	pub fn actr_debugger(value: usize);

	// actr_canvas2d_draw_image
	// @param image
	// @param sx
	// @param sy
	// @param sWidth
	// @param sHeight
	// @param dx
	// @param dy
	// @param dWidth
	// @param dHeight

	pub fn actr_canvas2d_draw_image(image: f32, sx: f32, sy: f32, sWidth: f32, sHeight: f32, dx: f32, dy: f32, dWidth: f32, dHeight: f32);

	// _actr_fetch_json_length
	// fetch json from the provided url
	// @param url
	// @param urlLength
	// @param jsonId

	pub fn _actr_fetch_json_length(url: *const u8, urlLength: u32, jsonId: i32) -> i32;

	// _actr_fetch_image_length
	// fetch the image from the provided url
	// @param url
	// @param urlLength
	// @param jsonId

	pub fn _actr_fetch_image_length(url: *const u8, urlLength: u32, jsonId: i32) -> i32;

	// _actr_fetch_text_length
	// fetch text from the provided url
	// @param url
	// @param urlLength
	// @param jsonId
	// @param path
	// @param pathLength

	pub fn _actr_fetch_text_length(url: *const u8, urlLength: u32, jsonId: i32, path: *const u8, pathLength: u32) -> i32;

	// actr_free
	// free allocated memory
	// @param ptr

	pub fn actr_free(ptr: usize);

	// actr_cos
	// @param value

	pub fn actr_cos(value: f64) -> f64;

	// actr_sin
	// @param value

	pub fn actr_sin(value: f64) -> f64;

	// actr_sqrt
	// @param value

	pub fn actr_sqrt(value: f64) -> f64;

	// actr_atan2
	// @param y
	// @param x

	pub fn actr_atan2(y: f64, x: f64) -> f64;

	// _actr_json_get_int_length
	// @param jsonId
	// @param path
	// @param pathLength

	pub fn _actr_json_get_int_length(jsonId: i32, path: *const u8, pathLength: u32) -> i64;

	// _actr_json_get_float_length
	// @param jsonId
	// @param path
	// @param pathLength

	pub fn _actr_json_get_float_length(jsonId: i32, path: *const u8, pathLength: u32) -> f64;

	// _actr_json_get_string_length
	// @param jsonId
	// @param path
	// @param pathLength

	pub fn _actr_json_get_string_length(jsonId: i32, path: *const u8, pathLength: u32) -> i32;

	// _actr_json_set_int_length
	// @param jsonId
	// @param path
	// @param pathLength
	// @param value

	pub fn _actr_json_set_int_length(jsonId: i32, path: *const u8, pathLength: u32, value: i64);

	// _actr_json_set_float_length
	// @param jsonId
	// @param path
	// @param pathLength
	// @param value

	pub fn _actr_json_set_float_length(jsonId: i32, path: *const u8, pathLength: u32, value: f64);

	// _actr_json_set_string_length
	// @param jsonId
	// @param path
	// @param pathLength
	// @param value
	// @param valueLength

	pub fn _actr_json_set_string_length(jsonId: i32, path: *const u8, pathLength: u32, value: *const u8, valueLength: u32);

	// actr_json_store
	// persist the json object to server storage
	// @param jsonId

	pub fn actr_json_store(jsonId: i32) -> i32;

	// actr_json_load
	// load the json object from server storage
	// @param jsonId

	pub fn actr_json_load(jsonId: i32) -> i32;

	// actr_json_delete
	// delete the json object from server storage
	// @param jsonId

	pub fn actr_json_delete(jsonId: i32) -> i32;

	// _actr_log_length
	// log a message to the console when in debug mode
	// @param message
	// @param length

	pub fn _actr_log_length(message: *const u8, length: usize);

	// actr_malloc
	// allocate memory, do not use if your assembly has it's own runtime for memory
	// @param size

	pub fn actr_malloc(size: u32) -> *const c_void;

	// actr_memory_report
	// return string pointer to a report about memory usage do not use if your assembly has it's own runtime for memory
	// allocates, must be freed manually

	pub fn actr_memory_report() -> *const c_char;

	// actr_memory_usage
	// report memory usage in bytes do not use if your assembly has it's own runtime for memory

	pub fn actr_memory_usage() -> u32;

	// actr_prng
	// random number generator, this is cryptographically secure

	pub fn actr_prng() -> f64;

	// _actr_sanity
	// set the string encoding used by your assembly
	// @param stringEncoding

	pub fn _actr_sanity(stringEncoding: StringEncoding);

	// actr_time
	// current unix time

	pub fn actr_time() -> u32;

	// actr_time_string
	// current javascript time string, allocates must be freed manually, do not use if your runtime has memory management

	pub fn actr_time_string() -> *const c_char;

	// _actr_three_init
	// three js support is complicated, I'll try to make it possible for simpleparts of three and expand over time
	// initialize three.js
	// identity returned from three methods links to the javascript object instance
	// @param buffer
	// memory buffer for storing float vectors
	// @param length

	pub fn _actr_three_init(buffer: usize, length: u32) -> bool;

	// actr_three_camera_perspective
	// initialize three perspective camera and return the identity
	// @param fov
	// @param near
	// @param far

	pub fn actr_three_camera_perspective(fov: f32, near: f32, far: f32) -> i32;

	// actr_three_geometry_dispose
	// @param identity

	pub fn actr_three_geometry_dispose(identity: i32);

	// actr_three_geometry_box
	// initialize box geometry
	// @param width
	// @param height
	// @param depth

	pub fn actr_three_geometry_box(width: f32, height: f32, depth: f32) -> i32;

	// actr_three_geometry_buffer
	// initialize buffer geometry with provided index/vertex list
	// @param indexCount
	// @param indices
	// @param vertexCount
	// @param vertices

	pub fn actr_three_geometry_buffer(indexCount: i32, indices: usize, vertexCount: i32, vertices: usize) -> i32;

	// actr_three_geometry_sphere
	// initialize sphere geometry
	// @param radius
	// @param width
	// @param height

	pub fn actr_three_geometry_sphere(radius: f32, width: i32, height: i32) -> i32;

	// actr_three_light_dispose
	// @param identity

	pub fn actr_three_light_dispose(identity: i32);

	// actr_three_light_ambient
	// @param color
	// @param intensity

	pub fn actr_three_light_ambient(color: u32, intensity: f32) -> i32;

	// actr_three_light_directional
	// @param color
	// @param intensity

	pub fn actr_three_light_directional(color: u32, intensity: f32) -> i32;

	// actr_three_material_dispose
	// @param object

	pub fn actr_three_material_dispose(object: i32);

	// actr_three_material_standard
	// @param color
	// @param emissive
	// @param transparent
	// @param opacity
	// @param wireframe
	// @param flatShading

	pub fn actr_three_material_standard(color: u32, emissive: u32, transparent: bool, opacity: f32, wireframe: bool, flatShading: bool) -> i32;

	// actr_three_mesh
	// @param geometry
	// @param material

	pub fn actr_three_mesh(geometry: i32, material: i32) -> i32;

	// actr_three_object_add
	// add object to other object
	// @param containerIdentity
	// @param objectIdentity

	pub fn actr_three_object_add(containerIdentity: i32, objectIdentity: i32);

	// actr_three_object_lookat
	// @param identity
	// @param x
	// @param y
	// @param z

	pub fn actr_three_object_lookat(identity: i32, x: f32, y: f32, z: f32);

	// actr_three_object_position
	// set object position
	// @param identity
	// @param x
	// @param y
	// @param z

	pub fn actr_three_object_position(identity: i32, x: f64, y: f64, z: f64);

	// actr_three_object_remove
	// remove object from object
	// @param containerIdentity
	// @param objectIdentity

	pub fn actr_three_object_remove(containerIdentity: i32, objectIdentity: i32);

	// actr_three_object_move_local
	// translate the object in local space
	// @param identity
	// @param x
	// @param y
	// @param z

	pub fn actr_three_object_move_local(identity: i32, x: f32, y: f32, z: f32);

	// actr_three_object_move_world
	// translate the objecy in world space
	// @param identity
	// @param x
	// @param y
	// @param z

	pub fn actr_three_object_move_world(identity: i32, x: f32, y: f32, z: f32);

	// actr_three_object_rotate
	// rotate the object
	// @param identity
	// @param x
	// @param y
	// @param z

	pub fn actr_three_object_rotate(identity: i32, x: f32, y: f32, z: f32);

	// actr_three_object_rotation
	// set object rotation
	// @param identity
	// @param x
	// @param y
	// @param z

	pub fn actr_three_object_rotation(identity: i32, x: f32, y: f32, z: f32);

	// actr_three_transform_read

	pub fn actr_three_transform_read();

	// actr_three_transform_buffer
	// @param index
	// @param buffer
	// @param offset
	// @param count

	pub fn actr_three_transform_buffer(index: usize, buffer: usize, offset: usize, count: usize);

	// actr_three_render

	pub fn actr_three_render();

	// actr_three_object_to_local
	// transform the provided world position to object local position
	// @param identity
	// @param x
	// @param y
	// @param z

	pub fn actr_three_object_to_local(identity: i32, x: f32, y: f32, z: f32);

	// actr_three_object_to_world
	// transform the provided object local position to world position
	// @param identity
	// @param x
	// @param y
	// @param z

	pub fn actr_three_object_to_world(identity: i32, x: f32, y: f32, z: f32);

	// actr_three_scene
	// initialize a scene, can only be done once

	pub fn actr_three_scene() -> i32;

	// seed
	// this is specific to Assembly Script

	pub fn seed() -> f64;

}
