use bevy::{prelude::*, window::PresentMode};

mod mouse;//must include this in main, along with mod.rs, to find mod in crate
use mouse::mouse_events;

mod geometry;
use geometry::my_plane;

mod camera;
use camera::pan_orbit;

// mod resources;
// use resources::plugin;
//test git
fn main() {
    println!("Hello, world!");
    let mut app = bevy::app::App::new(); //new vs empty //bevy::App has more trait implementations than bevy_app
    app.add_event::<mouse_events::MyCursorMoved>() //never used
    //Events

    //Resources
        //Window: event loops, changing contexts
        .insert_resource(Msaa { samples: 4 }) //remove jaggedness
        .insert_resource(WindowDescriptor { //must come before DefaultPlugins
            title: "I am a window!".to_string(),
            width: 1000.0,
            height: 800.0,
            present_mode: PresentMode::Fifo,
            ..default()
        })

    //Plugins
        .add_plugins(DefaultPlugins) //disable log and winit plugin when put into subapp 
        .add_plugins(bevy_mod_picking::DefaultPickingPlugins)
        .add_plugin(bevy_transform_gizmo::TransformGizmoPlugin::default()) // Use TransformGizmoPlugin::default() to align to the scene's coordinate system.

    //Startup Systems
       // .add_system(mouse_events::print_mouse_events_system)
        .add_startup_system(my_plane::setup_plane)
        .add_startup_system(pan_orbit::spawn_camera)

    //Systems
        .add_system(pan_orbit::pan_orbit_camera)
        .add_system(my_plane::add_block)
        .run();
}
