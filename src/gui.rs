//! A simple demonstration of how to construct and use Canvasses by splitting up the window.

//#[macro_use] 
extern crate conrod;
extern crate glium;
extern crate find_folder;
extern crate winit;
extern crate rand;

//use conrod::backend::glium::glium::{DisplayBuild, Surface};
use std;

use theme::theme;

// Draw the Ui.
pub fn set_widgets(ref mut ui: conrod::UiCell, ids: &mut Ids, app: &mut FruscoApp) {
    use conrod::{color, widget, Colorable, Labelable, Borderable, Positionable, Sizeable, Widget};

    const TITLE_SIZE: conrod::FontSize = 42;
    const SUBTITLE_SIZE: conrod::FontSize = 10;

    const TEXT_SIZE: conrod::FontSize = 18;
    const BEFORE_EDITBOX: conrod::position::Relative = conrod::position::Relative::Scalar(-21.0);
    const BEFORE_TEXT: conrod::position::Relative = conrod::position::Relative::Scalar(-21.0);
    const TEXTBOX_W: f64 = 70.0;
    const TEXTBOX_H: f64 = 21.0;

    // Construct our main `Canvas` tree.
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
                (ids.rg_image, widget::Canvas::new()
                    .border_color(color::GRAY)
                    .border(2.)
                    .length_weight(0.8)
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

    widget::PlotPath::new(min_x, max_x, min_y, max_y, f32::sin)
        .kid_area_w_of(ids.rg_image)
        .color(color::YELLOW)
        .middle_of(ids.rg_image)
        .set(ids.reflector, ui);



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
        .color(theme().background_color.invert())
        .border(theme().border_width)
        .border_color(theme().background_color.invert().plain_contrast())
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

}


//fn labelled_dialer(text: &str, value: f32, nu)


// Generate a unique `WidgetId` for each widget.
widget_ids! {
pub    struct Ids {
        master,
        header,
        radargram,
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

//    rust_logo: conrod::image::Id,
}


impl FruscoApp {
    /// Sensible defaults for the app.
    pub fn new() -> Self {
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
        }
    }

}




