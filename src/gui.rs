//! A simple demonstration of how to construct and use Canvasses by splitting up the window.


extern crate conrod;
extern crate glium;
extern crate find_folder;
extern crate winit;
extern crate rand;
extern crate image;
extern crate petgraph;

use conrod::backend::glium::glium::{DisplayBuild, Surface};
use std;
use std::iter::once;

use conrod::theme::Theme;

use theme;
// Draw the Ui.

pub fn set_widgets(ref mut ui: conrod::UiCell, ids: &mut Ids, app: &mut FruscoApp, 
                    display: &glium::backend::glutin_backend::GlutinFacade) {
    use conrod::{color, widget, Positionable, Sizeable, Colorable, Borderable, Widget};
    use conrod::backend::glium::glium;
    use conrod::backend::glium::glium::{DisplayBuild, Surface};
   use glium::backend::glutin_backend::GlutinFacade;

    const TITLE_SIZE: conrod::FontSize = 42;
    const SUBTITLE_SIZE: conrod::FontSize = 10;

    const TEXT_SIZE: conrod::FontSize = 18;
    const BEFORE_EDITBOX: conrod::position::Relative = conrod::position::Relative::Scalar(-21.0);
    const BEFORE_TEXT: conrod::position::Relative = conrod::position::Relative::Scalar(-21.0);
    const TEXTBOX_W: f64 = 70.0;
    const TEXTBOX_H: f64 = 21.0;

    let rg_image = widget::Canvas::new()
                  .border_color(color::GRAY)
                    .border(2.)
                    .length_weight(0.8);

    //Construct our main `Canvas` tree.
    widget::Canvas::new().flow_down(&[
        (ids.header, widget::Canvas::new()
//            .color(color::BLUE)
            .length_weight(0.05)
        ),
        (ids.radargram, widget::Canvas::new()
            .length_weight(0.5)
//            .color(color::GREY)
            //.border(10.)
//            .border_color(color::GREY)
            .flow_right(&[
                (ids.rg_controls, widget::Canvas::new()
//                    .color(color::LIGHT_ORANGE)
                    .length(84.0)
                    //.border(0.)
//                    .border_color(color::GREY)
                ),
                (ids.rg_image, rg_image
                ),
                (ids.rg_spacer, widget::Canvas::new()
                    .length_weight(0.05)
                ),
            ])
        ),
        (ids.target, widget::Canvas::new()
            .length_weight(0.5)
//            .color(color::YELLOW)
        ),
//         (ids.footer, widget::Canvas::new()
//             .length_weight(0.05)
// //            .color(color::BLUE)
//             .scroll_kids_vertically()
//         ),
    ]).set(ids.master, ui);
    
//---------------------------------
// Radargram itself
//---------------------------------
    // Use the `PlotPath` widget to display a sine wave.
    let min_x = 0.0;
    let max_x = 100.0;
    let min_y = -12.0;
    let max_y = 12.0;

    // widget::PlotPath::new(min_x, max_x, min_y, max_y, f32::sin)
    //     .kid_area_w_of(ids.rg_image)
    //     .color(color::YELLOW)
    //     .middle_of(ids.rg_image)
    //     .set(ids.reflector, ui);

   // widget::Image::new(radargram_image).w_h(w as f64, h as f64).middle().set(ids.radargram_image, ui);


//---------------------------------
// Radargram controls
//---------------------------------

    widget::Text::new("Dip")
        .parent(ids.rg_controls)
        .top_left_with_margin_on(ids.rg_controls, 7.0)
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
        .parent(ids.rg_controls)
        .y_position_relative_to(ids.dip_dialer,BEFORE_TEXT)
        .align_left_of(ids.dip_title)
        .font_size(TEXT_SIZE)
        .set(ids.strike_title, ui);

    let min = 0.0;
    let max = 360.0;
    let decimal_precision = 1;
    for new_strike in widget::NumberDialer::new(app.strike, min, max, decimal_precision)
        .parent(ids.rg_controls)
        .y_position_relative_to(ids.strike_title,BEFORE_EDITBOX)
        .align_left_of(ids.dip_title)
        .w_h(TEXTBOX_W, TEXTBOX_H)
        .set(ids.strike_dialer, ui)
    {
        app.strike = new_strike;
    }



    widget::Text::new("Δ vert")
        .parent(ids.rg_controls)
        .y_position_relative_to(ids.strike_dialer,BEFORE_TEXT)
        .align_left_of(ids.dip_title)
        .font_size(TEXT_SIZE)
        .set(ids.z_title, ui);

    //app.vo_text = app.vert_offset.to_string();

    for edit in widget::TextBox::new(&app.vo_text)
        .parent(ids.rg_controls)
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
        .parent(ids.rg_controls)
        .y_position_relative_to(ids.vert_offset,BEFORE_EDITBOX)
        .align_left_of(ids.dip_title)
        .font_size(TEXT_SIZE)
        .set(ids.y_title, ui);

    for edit in widget::TextBox::new(&app.do_text)
        .parent(ids.rg_controls)
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
        .parent(ids.rg_controls)
        .y_position_relative_to(ids.dip_offset,BEFORE_EDITBOX)
        .align_left_of(ids.dip_title)
        .font_size(TEXT_SIZE)
        .set(ids.x_title, ui);

    for edit in widget::TextBox::new(&app.so_text)
        .parent(ids.rg_controls)
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
        .parent(ids.rg_controls)
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
    for event in widget::EnvelopeEditor::new(&mut app.profile, 0.0, 100.0, -10.0, 10.0)
        .down(10.0)
        .w_h(700.0, 200.0)
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

    let left = [-40.0, -40.0];
    let top = [0.0, 40.0];
    let right = [40.0, -40.0];
    let points = once(left).chain(once(top)).chain(once(right));

    let left = app.profile[0];
    let mid = app.profile[1];
    let right = app.profile[2];
    //let points = once(left).chain(once(mid)).chain(once(right));
    //let points = once(left).chain(mid).chain(right);


    let points = app.profile.clone();

    widget::PointPath::centred(points)
        .middle_of(ids.radargram_image)
        .color(color::YELLOW)
        //.down(80.0)
        .set(ids.point_path, ui);

}



// Generate a unique `WidgetId` for each widget.
widget_ids! {
pub    struct Ids {
        master,
        header,
        radargram,
        radargram_image,
        point_path,
        target,
        rg_controls,
        rg_image,
        reflector,
        rg_spacer,
        footer,
        footer_scrollbar,
        floating_a,
        floating_b,
        tabs,
        tab_foo,
        tab_bar,
        tab_baz,

        // title,
        // subtitle,
        // top_left,
        // bottom_right,
        // foo_label,
        // bar_label,
        // baz_label,
        // button_matrix,
        // bing,
        // bong,


        dip_title,
        strike_title,
        dialer_panel,
        dip_dialer,
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
    ball_xy: conrod::Point,
    ball_color: conrod::Color,
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
    radargram: conrod::image::Id,


//    radargram_image: conrod::image::Id,
}


impl FruscoApp {
    /// Sensible defaults for the app.
    pub fn new(display : &glium::Display, image_map : &mut conrod::image::Map<glium::texture::Texture2d>) -> Self {

        // Create our `conrod::image::Map` which describes each of our widget->image mappings.
        // In our case we only have one image, however the macro may be used to list multiple.
        let radargram_image = load_radargram_image(&display, true);
        let (w, h) = (radargram_image.get_width(), radargram_image.get_height().unwrap());
        //let mut image_map = conrod::image::Map::new();
        let radargram_image = image_map.insert(radargram_image);
    //println!["rust logo: {:?}", radargram_image];


        FruscoApp {

            vert_offset: 0.0,
            vo_text: String::new(),
            dip_offset: 0.0,
            do_text: String::new(),
            strike_offset: 0.0,
            so_text: String::new(),

            ball_xy: [0.0, 0.0],
            ball_color: conrod::color::WHITE,
            dip: 0.0,
            strike: 0.0,
            width: 100.0,
            profile: vec![[0.0, 0.0],           // For debugging only
                          [10.0, -10.0],        // on release: put all 
                          [25.0, 0.0],          // to y=0.0
                          [80.0, 10.0],
                          [100.0, 0.0],
                         ],
            radargram: radargram_image,
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

    println!["ASSETS: {:?}", assets];


    let path = if rust_image {

        assets.join("images/rust.png")
    } else {
        
        assets.join("hole_partial_image.jpg")
    };

    println!["PATH: {:?}", path];


    let rgba_image = image::open(&std::path::Path::new(&path)).unwrap().to_rgba();
    
    let image_dimensions = rgba_image.dimensions();
    //println!["image dimensions: {:?}", image_dimensions];

    let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(rgba_image.into_raw(), image_dimensions);
    let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
    texture
}



