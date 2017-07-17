//extern crate three;

//use std::env;
use std::vec::Vec;


mod borehole;
use borehole::SurveyPoint;
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
    borehole.set_step(0.5);

    // borehole.add_survey_obs(0.0, 10.0, 10.0);
    // borehole.add_bottom_of_hole(40.);


    borehole.add_survey_obs(0.0,  0.0,  90.0);
    borehole.add_survey_obs(10.0, 10.0, 0.0);
    borehole.add_survey_obs(20.0, 12.0, 0.0);
    borehole.add_survey_obs(30.0, 13.0, 0.0);
    borehole.add_survey_obs(15.0, 11.5, 0.0);
    borehole.add_bottom_of_hole(40.);
  
    // for i in borehole{
    //      println!("{:?}",i);
    // }


    for i in 0..81 {
      let depth = (i as f32)/2.0;
      println!["{}", depth];
      borehole.interpolate(depth);
      //println!["{} - {:?}", depth, borehole.interpolate(depth)];
    }


}
