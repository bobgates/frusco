extern crate three;

use std::env;

fn main() {
    
    let mut args = env::args();
    let path = args.nth(1).unwrap_or("test_data/car.obj".to_string());

    println!("{}", path);

    let mut win = three::Window::new("Three-rs obj loading example", "data/shaders");
    let cam = win.factory.perspective_camera(60.0, 1.0, 10.0);
    let mut controls = three::OrbitControls::new(&cam, [0.0, 2.0, -5.0], [0.0, 0.0, 0.0]);

    let mut dir_light = win.factory.directional_light(0xffffff, 0.9);
    dir_light.look_at([15.0, 35.0, 35.0], [0.0, 0.0, 2.0], None);
    win.scene.add(&dir_light);

    let mut root = win.factory.group();
    win.scene.add(&root);
   let (group_map, _meshes) = win.factory.load_obj(&path);
    for g in group_map.values() {
        root.add(g);
    }


    let mut win2 = three::Window::new("Window 2", "data/shaders");
    let cam2 = win2.factory.perspective_camera(60.0, 1.0, 10.0);
    let mut controls2 = three::OrbitControls::new(&cam2, [0.0, 2.0, -5.0], [0.0, 0.0, 0.0]);

    let mut dir_light2 = win2.factory.directional_light(0xffffff, 0.9);
    dir_light2.look_at([15.0, 35.0, 35.0], [0.0, 0.0, 2.0], None);
    win2.scene.add(&dir_light2);

    let mut root2 = win2.factory.group();
    win2.scene.add(&root2);
    let (group_map2, _meshes2) = win2.factory.load_obj(&path);
    for g in group_map2.values() {
        root2.add(g);
    }




    while win2.update() && !three::KEY_ESCAPE.is_hit(&win2.input) && win.update() && !three::KEY_ESCAPE.is_hit(&win.input){
        controls.update(&win2.input);
        win.render(&cam);
        controls2.update(&win2.input);
        win2.render(&cam2);
    }
}
