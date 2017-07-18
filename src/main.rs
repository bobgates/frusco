//extern crate three;

//use std::env;
//use std::vec::Vec;


mod borehole;
//use borehole::SurveyObservation;
use borehole::Borehole;

fn main() {

    let mut borehole = Borehole::new();
    borehole.set_step(0.5)
            .set_collar(0.0, 0.0, 0.0);

    borehole.add_survey_obs(0.0,  0.0,  90.0)
            .add_survey_obs(10.0, 0.0, 0.0)
            .add_survey_obs(20.0, 180.0, 90.0)
            .add_survey_obs(30.0, 0.0, 0.0)
            .add_survey_obs(40.0, 90.0, 90.0)
            .add_survey_obs(50.0, 90.0, 0.0)
            .add_bottom_of_hole(60.);

    println!["depth: \tX, \tY, \tZ"];
    for depth in 0..61 {
        let pt = borehole.get_xyz(depth as f32);

        match pt {
            None => println!["No results from borehole.get_xyz for depth {}", depth],
            Some(pt) => println!["{}: \t{:.2}, \t{:.2}, \t{:.2}", depth, pt.x, pt.y, pt.z],
        }
    }
}
