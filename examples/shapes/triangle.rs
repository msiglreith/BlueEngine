/*
 * Blue Engine by Elham Aryanpur
 *
 * Triangle example using pre-defined shapes
 *
 * The license is same as the one on the root.
*/

use blue_engine::{
    header::{Engine, ObjectSettings, WindowDescriptor},
    primitive_shapes::triangle,
};

use log::info;

#[cfg_attr(
    target_os = "android",
    ndk_glue::main(backtrace = "on", logger(level = "debug", tag = "wgpu"))
)]
fn main() {
    info!("hello world");

    // let mut engine = Engine::new(WindowDescriptor::default()).expect("win");

    // let _ = triangle(ObjectSettings::default(), &mut engine).unwrap();

    // engine
    //     .update_loop(move |_, _, _, _, _| {})
    //     .expect("Error during update loop");
}
