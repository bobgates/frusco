
use conrod;
use conrod::backend::glium::glium::{self, Surface};
use conrod::widget::{Canvas, Button, Text, TextBox};
use conrod::{color, widget, Positionable, Sizeable, Colorable, Borderable, Widget};


use std::rc::Rc;
use std;
use std::collections::HashMap;
use glium::glutin::WindowId;
use gui;

pub struct Window {
    pub display : glium::Display,
    pub id : glium::glutin::WindowId,
    pub ui: conrod::Ui,
    pub image_map : conrod::image::Map<glium::texture::Texture2d>,
    pub renderer : conrod::backend::glium::Renderer,
    pub ids: gui::Ids,
    app_data: gui::FruscoApp,

}

impl Window {
    pub fn new( window : glium::glutin::WindowBuilder, 
                width : u32, height: u32,
                context: glium::glutin::ContextBuilder, 
                events_loop: &glium::glutin::EventsLoop,
        ) ->  Window {
            let display = glium::Display::new(window, context, &events_loop).unwrap();
            let id = display.gl_window().window().id().clone();
            let mut ui = conrod::UiBuilder::new([width as f64, height as f64]).build();
            let mut image_map = conrod::image::Map::<glium::texture::Texture2d>::new();
            let renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
            let mut ids = gui::Ids::new(ui.widget_id_generator());
            let app_data = gui::FruscoApp::new(&display, &mut image_map);

            Window {
                display : display,
                id: id,
                ui: ui,
                image_map : image_map,
                renderer : renderer,   
                ids: ids, 
                app_data : app_data, 
            }
    }

    pub fn process_event(&mut self, event: glium::glutin::Event) -> bool {
        // Use the `winit` backend feature to convert the winit event to a conrod one.
        if let Some(event) = conrod::backend::winit::convert_event(event, &self.display) {
            self.ui.handle_event(event.clone());
            true
        } else {
            false
        }
    }

    pub fn draw(&mut self) {

        gui::load_data_to_widgets(self.ui.set_widgets(), &self.ids, &mut self.app_data); 

        if let Some(primitives) = self.ui.draw_if_changed() {
            self.renderer.fill(&self.display, primitives, &self.image_map);
            let mut target = self.display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            self.renderer.draw(&self.display, &mut target, &mut self.image_map).unwrap();
            target.finish().unwrap();
        }
    }

    pub fn get_id(&mut self) -> glium::glutin::WindowId {
        self.id
    }

    pub fn check_id(&mut self, id : &glium::glutin::WindowId) -> bool {
        self.id == *id
    }
}

