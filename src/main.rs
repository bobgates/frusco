//extern crate three;

//use std::env;
use std::vec::Vec;


mod borehole;
use borehole::SurveyObservation;
use borehole::Borehole;

fn main() {


    // let mut collar : SurveyPoint = SurveyPoint{downhole:0.0, azimuth:0.0, inclination:0.0};
    // let da = -10./180.0*3.1415926;
    // let di = 8./180.0*3.1415926;
    // let mut end : SurveyPoint = SurveyPoint{downhole:100.0, azimuth:da, inclination:di};

    // let mut borehole = Borehole::new();

    // borehole.add_point(collar)
    //         .add_point(end)
    //         .set_step(0.5);



    let mut borehole = Borehole::new();
    borehole.set_step(0.5)
            .set_collar(0.0, 0.0, 0.0);

    // borehole.add_survey_obs(0.0, 10.0, 10.0);
    // borehole.add_bottom_of_hole(40.);


    borehole.add_survey_obs(0.0,  0.0,  90.0);
    borehole.add_survey_obs(10.0, 10.0, 0.0);
    borehole.add_survey_obs(20.0, 180.0, 90.0);
    borehole.add_survey_obs(30.0, 13.0, 0.0);
    borehole.add_survey_obs(40.0, 90.0, 90.0);
    borehole.add_survey_obs(50.0, 90.0, 0.0);
    borehole.add_bottom_of_hole(60.);
  

    for i in 0..5{
        println!["{}: {}, {}, {}, {}", i, borehole.coords[i].depth, borehole.coords[i].x, borehole.coords[i].y, borehole.coords[i].z,]
    }


   for i in 0..121 {
     let depth = (i as f32)/2.0;
     let pt = borehole.interpolate(depth).unwrap();
     println!["{}: \t{}, \t{}, \t{}", depth, pt.x, pt.y, pt.z];
   }


}
