//! A simple demonstration of how to construct and use Canvasses by splitting up the window.

#[macro_use] 
extern crate conrod;
extern crate glium;
extern crate find_folder;
extern crate winit;
extern crate rand;

use conrod::backend::glium::glium::{DisplayBuild, Surface};

pub fn main() {

    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;




    // Build the window.
    let display = glium::glutin::WindowBuilder::new()
        .with_vsync()
        .with_dimensions(WIDTH, HEIGHT)
        .with_title("First pass target GUI")
        .with_multisampling(4)
        .build_glium()
        .unwrap();


    // construct our `Ui`.
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();



    let mut app = DemoApp::new();

    // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    // Instantiate the generated list of widget identifiers.
    let ids = &mut Ids::new(ui.widget_id_generator());

    // Poll events from the window.
    let mut event_loop = EventLoop::new();
    'main: loop {   

        // Handle all events.
        for event in event_loop.next(&display) {

//            println!["{:?}", event];

            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = conrod::backend::winit::convert(event.clone(), &display) {
 //               println!["__{:?}", event];
                ui.handle_event(event);

                event_loop.needs_update();
            }

            match event {
                // Break from the loop upon `Escape`.
                glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Escape)) |
                glium::glutin::Event::Closed =>
                    break 'main,
                _ => {},
            }
        }

        // Instantiate all widgets in the GUI.
        set_widgets(ui.set_widgets(), ids, &mut app);

        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}

// Draw the Ui.
fn set_widgets(ref mut ui: conrod::UiCell, ids: &mut Ids, app: &mut DemoApp) {
    use conrod::{color, widget, Colorable, Labelable, Borderable, Positionable, Sizeable, Widget};

    const MARGIN: conrod::Scalar = 30.0;
    const SHAPE_GAP: conrod::Scalar = 50.0;
    const TITLE_SIZE: conrod::FontSize = 42;
    const SUBTITLE_SIZE: conrod::FontSize = 10;


    // Construct our main `Canvas` tree.
    widget::Canvas::new().flow_down(&[
        (ids.header, widget::Canvas::new().color(color::BLUE).length_weight(0.05)),
        (ids.radargram, widget::Canvas::new().length_weight(0.6)
                .color(color::GREY)
                //.border(10.)
                .border_color(color::GREY)
                .flow_right(&[
                        (ids.rg_controls, widget::Canvas::new()
                            .color(color::LIGHT_ORANGE)
                            .length_weight(0.1)
                            //.border(0.)
                            .border_color(color::GREY)
                        ),
                        (ids.rg_image, widget::Canvas::new()
                            .color(color::ORANGE)
                            .length_weight(0.8)
                        ),
                ])
        ),
        (ids.target, widget::Canvas::new().length_weight(0.4).color(color::YELLOW)),
        (ids.footer, widget::Canvas::new().length_weight(0.05).color(color::BLUE).scroll_kids_vertically()),
    ]).set(ids.master, ui);
    
    let shapes_canvas_rect = ui.rect_of(ids.target).unwrap();
    let w = shapes_canvas_rect.w() * 5.0 / 6.0;
    let h = shapes_canvas_rect.h() * 5.0 / 6.0;
    let radius = 10.0;

    widget::RoundedRectangle::fill([w, h], radius)
        .color(conrod::color::CHARCOAL.alpha(0.25))
        .middle_of(ids.target)
        .set(ids.rounded_rectangle, ui);

//*********************************************************************************
//*********************************************************************************

    let ball_x_range = ui.kid_area_of(ids.target).unwrap().w();
    let ball_y_range = ui.kid_area_of(ids.target).unwrap().h();

    let min_x = -ball_x_range / 2.2;
    let max_x = ball_x_range / 2.2;
    let min_y = -ball_y_range / 2.5;
    let max_y = ball_y_range / 2.5;
    let side = 200.0;

    for (x, y) in widget::XYPad::new(app.ball_xy[0], min_x, max_x,
                                     app.ball_xy[1], min_y, max_y)
        //.label("BALL XY")
        .color(conrod::color::rgba(0.0, 0.0, 0.0, 1.0))
        .w_h(ball_x_range/1.1, ball_y_range/1.25)
        .middle_of(ids.target)
        //.mid_left_of(ids.target)
        .parent(ids.target)
        .set(ids.xy_pad, ui)
    {
        app.ball_xy = [x, y];
    }

    let ball_x = app.ball_xy[0];
    let ball_y = app.ball_xy[1];// - max_y - side * 0.5 - MARGIN - 20.0;

    widget::Circle::fill(10.0)
        .color(app.ball_color)
        .x_y_relative_to(ids.target, ball_x, ball_y)
        .set(ids.ball, ui);

//************************
// Number Dialer 
//************************
    // Use a `NumberDialer` widget to adjust the frequency of the sine wave below.



    let min = -90.0;
    let max = 90.0;
    let decimal_precision = 1;
    for new_dip in widget::NumberDialer::new(app.dip, min, max, decimal_precision)
        .down(60.0)
        .parent(ids.rg_controls)
        //.middle_of(ids.rg_controls)
        .w_h(60.0, 30.0)
        //.label("F R E Q")
        .set(ids.dip_dialer, ui)
//        .parent(ids.rg_controls)
    {
        app.dip = new_dip;
    }

    let min = 0.0;
    let max = 360.0;
    let decimal_precision = 1;
    for new_strike in widget::NumberDialer::new(app.strike, min, max, decimal_precision)
        .down(60.0)
        //.middle_of(ids.rg_controls)
        .w_h(70.0, 30.0)
        //.label("F R E Q")
        .set(ids.strike_dialer, ui)
//        .parent(ids.rg_controls)
    {
        app.strike = new_strike;
    }

    widget::Canvas::new().flow_down(&[new_strike]).set(ids.dialer_panel);


    // let button = widget::Button::new().color(color::RED).w_h(30.0, 30.0);
    // for _click in button.clone().middle_of(ids.floating_a).set(ids.bing, ui) {
    //     println!("Bing!");
    // }
    // for _click in button.middle_of(ids.floating_b).set(ids.bong, ui) {
    //     println!("Bong!");
    // }

}


// Generate a unique `WidgetId` for each widget.
widget_ids! {
    struct Ids {
        master,
        header,
        radargram,
        target,
        rg_controls,
        rg_image,
        footer,
        footer_scrollbar,
        floating_a,
        floating_b,
        tabs,
        tab_foo,
        tab_bar,
        tab_baz,

        title,
        subtitle,
        top_left,
        bottom_right,
        foo_label,
        bar_label,
        baz_label,
        button_matrix,
        bing,
        bong,

        dialer_panel,
        dip_dialer,
        strike_dialer,

        rounded_rectangle,
    

        button_title,
        button,
        xy_pad,
        ball,
        toggle,


    }



}

pub struct EventLoop {
    ui_needs_update: bool,
    last_update: std::time::Instant,
}

impl EventLoop {

    pub fn new() -> Self {
        EventLoop {
            last_update: std::time::Instant::now(),
            ui_needs_update: true,
        }
    }

    /// Produce an iterator yielding all available events.
    pub fn next(&mut self, display: &glium::Display) -> Vec<glium::glutin::Event> {
        // We don't want to loop any faster than 60 FPS, so wait until it has been at least 16ms
        // since the last yield.
        let last_update = self.last_update;
        let sixteen_ms = std::time::Duration::from_millis(16);
        let duration_since_last_update = std::time::Instant::now().duration_since(last_update);
        if duration_since_last_update < sixteen_ms {
            std::thread::sleep(sixteen_ms - duration_since_last_update);
        }

        // Collect all pending events.
        let mut events = Vec::new();
        events.extend(display.poll_events());

        // If there are no events and the `Ui` does not need updating, wait for the next event.
        if events.is_empty() && !self.ui_needs_update {
            events.extend(display.wait_events().next());
        }

        self.ui_needs_update = false;
        self.last_update = std::time::Instant::now();

        events
    }

    /// Notifies the event loop that the `Ui` requires another update whether or not there are any
    /// pending events.
    ///
    /// This is primarily used on the occasion that some part of the `Ui` is still animating and
    /// requires further updates to do so.
    pub fn needs_update(&mut self) {
        self.ui_needs_update = true;
    }

}


pub struct DemoApp {
    ball_xy: conrod::Point,
    ball_color: conrod::Color,
    dip: f32,
    strike: f32,
//    rust_logo: conrod::image::Id,
}


impl DemoApp {

    /// Simple constructor for the `DemoApp`.
    pub fn new() -> Self {
        DemoApp {
            ball_xy: [0.0, 0.0],
            ball_color: conrod::color::WHITE,
            dip: 0.0,
            strike: 0.0,
 //           rust_logo: conrod::image::Id::new(),
        }
    }

}

