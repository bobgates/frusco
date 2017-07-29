//! A simple demonstration of how to construct and use Canvasses by splitting up the window.


extern crate conrod;
extern crate glium;
extern crate find_folder;
extern crate winit;
extern crate rand;
extern crate image;
extern crate petgraph;

//use conrod::backend::glium::glium::{DisplayBuild, Surface};
use std;
//use std::iter::once;

//use conrod::theme::Theme;

use theme;
// Draw the Ui.

use conrod::utils::map_range;


pub fn set_widgets(ref mut ui: conrod::UiCell, ids: &mut Ids, app: &mut FruscoApp){ 
//                    display: &glium::backend::glutin_backend::GlutinFacade) {
    use conrod::{color, widget, Positionable, Sizeable, Colorable, Borderable, Widget};

    const TEXT_SIZE: conrod::FontSize = 16;
    const BEFORE_EDITBOX: conrod::position::Relative = conrod::position::Relative::Scalar(-19.0);
    const BEFORE_TEXT: conrod::position::Relative = conrod::position::Relative::Scalar(-21.0);
    const TEXTBOX_W: f64 = 70.0;
    const TEXTBOX_H: f64 = 19.0;

    //Construct our main `Canvas` tree.
    widget::Canvas::new().flow_down(&[
        (ids.header, widget::Canvas::new()
            .color(color::DARK_GREY)
            .length_weight(0.05)
        ),
        (ids.radargram, widget::Canvas::new()
            .length_weight(0.5)
            .border(2.0)
            .border_color(color::ORANGE)
            .flow_right(&[
                (ids.rg_controls, widget::Canvas::new()
                    .length(84.0)
                ),
                (ids.rg_image, widget::Canvas::new()
                    .border_color(color::GRAY)
                    .pad(10.0)
                    .length_weight(0.8)
                ),
                // (ids.rg_spacer, widget::Canvas::new()
                //     .length_weight(0.05)
                // ),
            ])
        ),
        (ids.target_panel, widget::Canvas::new()
            .length_weight(0.5)
            .border(0.02)
            .border_color(color::ORANGE)
            .flow_right(&[
                (ids.target_controls, widget::Canvas::new()
                    .length(84.)
                    //.color(color::GREY)
                ),
                (ids.target, widget::Canvas::new()
                    .border_color(color::GRAY)
                    .pad(10.)
                    .length_weight(0.8)
                ),
                // (ids.target_spacer, widget::Canvas::new()
                //     .length_weight(0.05)
                // ),
            ])
        ),
//         (ids.footer, widget::Canvas::new()
//             .length_weight(0.05)
// //            .color(color::BLUE)
//             .scroll_kids_vertically()
//         ),
    ]).set(ids.master, ui);
    
//---------------------------------
// Radargram controls
//---------------------------------

    widget::Text::new("Velocity")
        .parent(ids.rg_controls)
        .top_left_with_margin_on(ids.rg_controls, 7.0)
        .font_size(TEXT_SIZE)
        .set(ids.velocity_title, ui);

    let min = 0.0;
    let max = 15.0;
    let decimal_precision = 2;
    for new_velocity in widget::NumberDialer::new(app.velocity*100.0, min, max, decimal_precision)
        .y_position_relative_to(ids.velocity_title,BEFORE_EDITBOX)
        .align_left_of(ids.velocity_title)
        .w_h(TEXTBOX_W, TEXTBOX_H)
        //.label("F R E Q")
        .set(ids.velocity_dialer, ui)
    {
        app.velocity = new_velocity/100.0;
    }

//---------------------------------
// Target controls
//---------------------------------

    widget::Text::new("Dip")
        .parent(ids.target_controls)
        .top_left_with_margin_on(ids.target_controls, 7.0)
        .font_size(TEXT_SIZE)
        .set(ids.dip_title, ui);

    let min = -90.0;
    let max = 90.0;
    let decimal_precision = 1;
    for new_dip in widget::NumberDialer::new(app.dip, min, max, decimal_precision)
        .y_position_relative_to(ids.dip_title,BEFORE_EDITBOX)
        .align_left_of(ids.dip_title)
        .w_h(TEXTBOX_W, TEXTBOX_H)
        //.label("F R E Q")
        .set(ids.dip_dialer, ui)
    {
        app.dip = new_dip;
    }


    widget::Text::new("Strike")
        .parent(ids.target_controls)
        .y_position_relative_to(ids.dip_dialer,BEFORE_TEXT)
        .align_left_of(ids.dip_title)
        .font_size(TEXT_SIZE)
        .set(ids.strike_title, ui);

    let min = 0.0;
    let max = 360.0;
    let decimal_precision = 1;
    for new_strike in widget::NumberDialer::new(app.strike, min, max, decimal_precision)
        .parent(ids.target_controls)
        .y_position_relative_to(ids.strike_title,BEFORE_EDITBOX)
        .align_left_of(ids.dip_title)
        .w_h(TEXTBOX_W, TEXTBOX_H)
        .set(ids.strike_dialer, ui)
    {
        app.strike = new_strike;
    }

    widget::Text::new("Δ vert")
        .parent(ids.target_controls)
        .y_position_relative_to(ids.strike_dialer,BEFORE_TEXT)
        .align_left_of(ids.dip_title)
        .font_size(TEXT_SIZE)
        .set(ids.z_title, ui);

    //app.vo_text = app.vert_offset.to_string();

    for edit in widget::TextBox::new(&app.vo_text)
        .parent(ids.target_controls)
        .y_position_relative_to(ids.z_title,BEFORE_EDITBOX)
        .align_left_of(ids.dip_title)
        .w_h(TEXTBOX_W, TEXTBOX_H)
        .font_size(TEXT_SIZE)
        .set(ids.vert_offset, ui)
    {
            match edit {
                conrod::widget::text_box::Event::Enter => {
                    app.vert_offset = app.vo_text.parse().unwrap_or(0.0);
                    println!["{:?}", app.vert_offset];
                }
                conrod::widget::text_box::Event::Update(text) => {
                    app.vo_text = text;
                }
            }
    }

    widget::Text::new("Δ dip")
        .parent(ids.target_controls)
        .y_position_relative_to(ids.vert_offset,BEFORE_EDITBOX)
        .align_left_of(ids.dip_title)
        .font_size(TEXT_SIZE)
        .set(ids.y_title, ui);

    for edit in widget::TextBox::new(&app.do_text)
        .parent(ids.target_controls)
        .y_position_relative_to(ids.y_title, BEFORE_TEXT)
        .align_left_of(ids.dip_title)
        .w_h(TEXTBOX_W, TEXTBOX_H)
        .font_size(TEXT_SIZE)
        .set(ids.dip_offset, ui)
    {
            match edit {
                conrod::widget::text_box::Event::Enter => {
                    app.dip_offset = app.do_text.parse().unwrap_or(0.0);
                    println!["{:?}", app.dip_offset];
                }
                conrod::widget::text_box::Event::Update(text) => {
                    app.do_text = text;
                }
            }
    }

    widget::Text::new("Δ strike")
        .parent(ids.target_controls)
        .y_position_relative_to(ids.dip_offset,BEFORE_EDITBOX)
        .align_left_of(ids.dip_title)
        .font_size(TEXT_SIZE)
        .set(ids.x_title, ui);

    for edit in widget::TextBox::new(&app.so_text)
        .parent(ids.target_controls)
        .y_position_relative_to(ids.x_title, BEFORE_TEXT)
        .align_left_of(ids.dip_title)
        .w_h(TEXTBOX_W, TEXTBOX_H)
        .font_size(TEXT_SIZE)
        .set(ids.strike_offset, ui)
    {
            match edit {
                conrod::widget::text_box::Event::Enter => {
                    app.strike_offset = app.so_text.parse().unwrap_or(0.0);
                    println!["{:?}", app.strike_offset];
                }
                conrod::widget::text_box::Event::Update(text) => {
                    app.so_text = text;
                }
            }
    }

    widget::Text::new("Width")
        .parent(ids.target_controls)
        .top_left_with_margin_on(ids.rg_controls, 7.0)
        .font_size(TEXT_SIZE)
        .y_position_relative_to(ids.strike_offset, BEFORE_TEXT)
        .set(ids.width_title, ui);

    let min = 0.0;
    let max = 100.0;
    let decimal_precision = 0;
    for new_dip in widget::NumberDialer::new(app.width, min, max, decimal_precision)
        .y_position_relative_to(ids.width_title,BEFORE_EDITBOX)
        .align_left_of(ids.dip_title)
        .w_h(TEXTBOX_W, TEXTBOX_H)
        //.label("F R E Q")
        .set(ids.width_dialer, ui)
    {
        app.width = new_dip;
    }

//---------------------------------
// Target manipulator
//---------------------------------

    // Draw an EnvelopeEditor. (&[Point], x_min, x_max, y_min, y_max).
    for event in widget::EnvelopeEditor::new(&mut app.profile, 0.0, 100.0, -15.0, 15.0)
        //.down(10.0)
        .kid_area_w_of(ids.target)
        .kid_area_h_of(ids.target)
        .parent(ids.target)
//        .skew_y(env_skew_y)
        .color(theme::theme().background_color.invert())
        .border(theme::theme().border_width)
        .border_color(theme::theme().background_color.invert().plain_contrast())
        //.label(&text)
        //.label_color(app.bg_color.invert().plain_contrast().alpha(0.5))
        .point_radius(3.0)
        .middle_of(ids.target)
        .line_thickness(1.5)
        .set(ids.env_editor, ui)
    {
        event.update(&mut app.profile);
//         println!["{:?}", event];
    }

 
//---------------------------------
// Radargram image
//---------------------------------

    // let w = rg_image.element.Properties().width;

    // // println!["{:?}", w];

    widget::Image::new(app.radargram)
        .kid_area_w_of(ids.rg_image)
        .kid_area_h_of(ids.rg_image)
        .middle_of(ids.rg_image)
        .set(ids.radargram_image, ui);

    let visible = ui.visible_area(ids.radargram_image);

    match visible {
        Some(rect) => { 
            let x1 = rect.x.start; 
            let x2 = rect.x.end;
            
            let y1 = rect.y.start; 
            let y2 = rect.y.end;

            let mut mapped : Vec<conrod::Point> = Vec::new();

            for i in app.profile.clone(){
                let xn = map_range(i[0], 0.0, 100.0, x1, x2);
                let yn = map_range(i[1], -15.0, 15.0, y1, y2);
                mapped.push([xn, yn].clone());
            }

            widget::PointPath::abs(mapped)
                .color(color::RED)
                .thickness(2.)
                .align_left_of(ids.rg_image)
                .set(ids.point_path, ui);

        }
        None => (),
    }




}



// Generate a unique `WidgetId` for each widget.
widget_ids! {
pub    struct Ids {
        master,
        header,
        point_path,
        
        radargram,          // The image itself

        radargram_image,
        rg_controls,
        rg_image,
        reflector,
        rg_spacer,

        velocity_title,
        velocity_dialer,

        target_panel,
        target_controls,
        target,
        target_spacer,

        footer,

        dialer_panel,
        dip_title,
        dip_dialer,
        strike_title,
        strike_dialer,
        x_title,
        strike_offset,
        y_title,
        dip_offset,
        z_title,
        vert_offset,
        width_title,
        width_dialer,

        rounded_rectangle,
    

        env_editor,

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


pub struct FruscoApp {
// Radargram and controls
    radargram: conrod::image::Id,

    borehole_left: f32,
    borehole_right: f32,
    min_time: f32,
    max_time: f32,

    view_left: f32,
    view_right: f32,
    view_min_time: f32,
    view_max_time: f32,

    velocity: f32,

// Target and controls
    vert_offset: f32,
    vo_text : String,
    dip_offset: f32,
    do_text : String,
    strike_offset: f32,
    so_text : String,
    dip: f32,
    width: f32,
    strike: f32,
    profile: Vec<conrod::Point>,
}


impl FruscoApp {
    /// Sensible defaults for the app.
    pub fn new(display : &glium::Display, image_map : &mut conrod::image::Map<glium::texture::Texture2d>) -> Self {

        let radargram_image = load_radargram_image(&display, false);
        let (w, h) = (radargram_image.get_width(), radargram_image.get_height().unwrap());
        let radargram_image = image_map.insert(radargram_image);

        println!["radargram (w,h): {}, {}", w, h];

        FruscoApp {
            radargram: radargram_image,

            borehole_left: 0.0,                 // depth of left end of radargram, m
            borehole_right: 100.0,              // depth of right end of radargram, m
            min_time: 0.0,                      // ns
            max_time: 500.0,                    // ns

            view_left: 0.0,                 // depth of left end of view, m
            view_right: 100.0,              // depth of right end of view, m
            view_min_time: 0.0,             // ns
            view_max_time: 500.0,

            velocity: 0.05,                     // m/ns two way

// Borehole view controls

            vert_offset: 0.0,
            vo_text: String::new(),
            dip_offset: 0.0,
            do_text: String::new(),
            strike_offset: 0.0,
            so_text: String::new(),

            dip: 0.0,
            strike: 0.0,
            width: 100.0,
            profile: vec![[0.0, 0.0],        // For debugging only
                          [10.0, -15.0],       // on release: put all 
                          [25.0, 0.0],          // to y=0.0
                          [80.0, 15.0],
                          [100.0, 0.0],
                         ],
        }
    }

}

/// Loads a radargram from a location, or the rust logo for test purposes.
fn load_radargram_image(display: &glium::Display, rust_image: bool) -> glium::texture::Texture2d {
    
    let assets = if rust_image {
        find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap()
    } else { 
        find_folder::Search::ParentsThenKids(3, 3).for_folder("test_data").unwrap()
    };

    let path = if rust_image {

        assets.join("images/rust.png")
    } else {
        
        assets.join("hole_partial_image.jpg")
    };

    let rgba_image = image::open(&std::path::Path::new(&path)).unwrap().to_rgba();
    
    let image_dimensions = rgba_image.dimensions();
    //println!["image dimensions: {:?}", image_dimensions];

    let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(rgba_image.into_raw(), image_dimensions);
    let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
    texture
}



