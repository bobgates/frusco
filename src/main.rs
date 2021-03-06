//extern crate three;

//use std::env;
//use std::vec::Vec;

#[macro_use] 
extern crate conrod;
extern crate find_folder;
//extern crate rand;
extern crate cgmath;
//use cgmath::Point3;


mod gui;
mod theme;
mod layout;
mod eventloop;

use conrod::backend::glium::glium::{self, Surface};

pub fn main() {

    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;


    // Build the window.
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Conrod with glium!")
        .with_dimensions(WIDTH, HEIGHT);
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    // let display = glium::Display::new(window, context, &events_loop).unwrap();

    // // construct our `Ui`.
    // let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).theme(theme::theme()).build();;

    // // Instantiate the generated list of widget identifiers.
    // let ids = &mut gui::Ids::new(ui.widget_id_generator());

    let mut window1 = layout::Window::new(window, WIDTH, HEIGHT, context, &events_loop);


    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    window1.ui.fonts.insert_from_file(font_path).unwrap();


    // The image map describing each of our widget->image mappings (in our case, none).
//    let mut image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

//    let mut app_data = gui::FruscoApp::new(&display, &mut image_map);
    // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
//    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();





    // Poll events from the window.
    let mut event_loop = eventloop::EventLoop::new();
    'main: loop {   

        // Handle all events.
        for event in event_loop.next(&mut events_loop) {

//            println!["{:?}", event];

            // Use the `winit` backend feature to convert the winit event to a conrod one.
            match event {
                glium::glutin::Event::WindowEvent{ window_id, .. } => {
                    if window1.check_id(&window_id) {
                        if window1.process_event(event.clone()) {
                            event_loop.needs_update();
                        }
                    }
                },
                _ => (),
            };
            // if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &display) {
            //     ui.handle_event(event);
            //     event_loop.needs_update();
            // }

            match event {
                glium::glutin::Event::WindowEvent {event, .. } => match event {
                    // Break from the loop upon `Escape`.
                    glium::glutin::WindowEvent::Closed |
                    glium::glutin::WindowEvent::KeyboardInput {
                        input: glium::glutin::KeyboardInput {
                            virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => break 'main,
                    _ => (),
                },
                _ => {},
            }
        }
        

        window1.draw();


        // // Instantiate all widgets in the GUI.
        // gui::set_widgets(ui.set_widgets(), ids, &mut app);
        
        // // Render the `Ui` and then display it on the screen.
        // if let Some(primitives) = ui.draw_if_changed() {
        //     renderer.fill(&display, primitives, &image_map);
        //     let mut target = display.draw();
        //     target.clear_color(0.0, 0.0, 0.0, 1.0);
        //     renderer.draw(&display, &mut target, &image_map).unwrap();
        //     target.finish().unwrap();
        
    }
}






    // let mut event_loop = eventloop::EventLoop::new();
    // let mut active = 0;

    // 'main: loop {

    //     for event in event_loop.next(&mut events_loop){
    //         match event {
    //             glium::glutin::Event::WindowEvent{ window_id, .. } => {
    //                 if window1.check_id(&window_id) {
    //                     if window1.process_event(event.clone()) {
    //                         event_loop.needs_update();
    //                     }
    //                 }
    //             },
    //             _ => (),
    //         };

    //         match event {
    //             glium::glutin::Event::WindowEvent{ event, ..} => match event {
    //                 glium::glutin::WindowEvent::Closed |
    //                 glium::glutin::WindowEvent::KeyboardInput {
    //                     input: glium::glutin::KeyboardInput {
    //                         virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
    //                         ..
    //                     },
    //                     ..
    //                 } => break 'main,
    //                 _ => (),
    //             },
    //             _ => (),
    //         }
 
    //         window1.draw();

    //     }
    // }



// fn main() {

//     let mut borehole = Borehole::new();
//     borehole.set_step(0.5)
//             .set_collar(0.0, 0.0, 0.0);

//     borehole.add_survey_obs(0.0,  0.0,  90.0)
//             .add_survey_obs(10.0, 0.0, 95.0)
//             .add_survey_obs(20.0, 0.0, 100.0)
//             .add_survey_obs(30.0, 0.0, 105.0)
//             .add_survey_obs(40.0, 0.0, 110.0)
//             .add_survey_obs(50.0, 0.0, 110.0)
//             .add_bottom_of_hole(60.);


//     let mut target1 = TargetPlane::new();


//     target1.print();
//     target1.up(1.0);
//     target1.move_x(1.0);
//     target1.move_y(1.0)
//             .set_dir(34.0)
//             .set_dip(12.0)
//             .set_width(99.0)
//             .set_length(140.0)
//             .move_dir(-1.5)
//             .move_dip(2.4);
//     target1.print();
    
//     println![""];



//     let rect = Rect3D{top_left: Point3{x:-5.0, y:50.0, z:3.0},
//                       top_right: Point3{x:5.0, y:50.0, z:3.0},
//                       bottom_right: Point3{x:5.0, y:20.0, z:0.0},
//                      };


//     let p = Point3{x: 0.0, y:0.0, z:5.0};



//     target::projection_from_point(&rect, p);

//     // println!["depth: \tX, \tY, \tZ"];
//     for depth in 0..61 {
//         let pt = borehole.get_xyz(depth as f32);

//         match pt {
//             None => println!["No results from borehole.get_xyz for depth {}", depth],
//             Some(pt) => {//println!["{}: \t{:.2}, \t{:.2}, \t{:.2}", depth, pt.x, pt.y, pt.z];
//                          let sd=target::projection_from_point(&rect, Point3{x:pt.x, y:pt.y, z:pt.z});
//                          match sd{
//                             None => (),
//                             Some(sd) => {println!["{},{},{}", depth, pt.z,sd]}
//                          }
//                      },
//         }
//     }
// }

