//extern crate three;

//use std::env;
//use std::vec::Vec;


mod target;
use target::TargetPlane;





mod borehole;
//use borehole::SurveyObservation;
use borehole::Borehole;

fn main() {

    let mut borehole = Borehole::new();
    borehole.set_step(0.5)
            .set_collar(0.0, 0.0, 0.0);

    borehole.add_survey_obs(0.0,  0.0,  90.0)
            .add_survey_obs(10.0, 0.0, 95.0)
            .add_survey_obs(20.0, 0.0, 100.0)
            .add_survey_obs(30.0, 0.0, 105.0)
            .add_survey_obs(40.0, 0.0, 110.0)
            .add_survey_obs(50.0, 0.0, 110.0)
            .add_bottom_of_hole(60.);


    let mut target1 = TargetPlane::new();


    target1.print();
    target1.up(1.0);
    target1.move_x(1.0);
    target1.move_y(1.0)
            .set_dir(34.0)
            .set_dip(12.0)
            .set_width(99.0)
            .set_length(140.0)
            .move_dir(-1.5)
            .move_dip(2.4);
    target1.print();
    

    // println!["depth: \tX, \tY, \tZ"];
    // for depth in 0..61 {
    //     let pt = borehole.get_xyz(depth as f32);

    //     match pt {
    //         None => println!["No results from borehole.get_xyz for depth {}", depth],
    //         Some(pt) => println!["{}: \t{:.2}, \t{:.2}, \t{:.2}", depth, pt.x, pt.y, pt.z],
    //     }
    // }
}
