extern crate three;

use std::env;
use std::vec::Vec;
use std::cmp::Ordering::Less;

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
    borehole.add_survey_obs(0.0,  10.0, 10.0);
    borehole.add_survey_obs(10.0, 11.0, 11.0);
    borehole.add_survey_obs(20.0, 12.0, 12.0);
    borehole.add_survey_obs(30.0, 13.0, 13.0);
    borehole.add_survey_obs(15.0, 11.5, 11.5);


    borehole.survey.sort_by(|a, b| a.unwrap().clone().downhole.partial_cmp(&b.unwrap().clone().downhole).unwrap_or(Less));


    for i in borehole.survey{
        println!("{:?}",i);
    }

//    println!("_____{:?}", borehole.survey[1]);


    // for point in borehole{
    //     println!("{:?}", point);
    // }

    //collar.from_xyz(1.,2.,3.);




   //  let map = Hashmap::L

     
   //  let mut args = env::args();
   //  let path = args.nth(1).unwrap_or("test_data/car.obj".to_string());

   //  println!("{}", path);


   //  let v: Vec<_> = vec![1,2,3];
   //  v.assert_eq!(, );

   //  let mut win = three::Window::new("Three-rs obj loading example", "data/shaders");
   //  let cam = win.factory.perspective_camera(60.0, 1.0, 10.0);
   //  let mut controls = three::OrbitControls::new(&cam, [0.0, 2.0, -5.0], [0.0, 0.0, 0.0]);

   //  let mut dir_light = win.factory.directional_light(0xffffff, 0.9);
   //  dir_light.look_at([15.0, 35.0, 35.0], [0.0, 0.0, 2.0], None);
   //  win.scene.add(&dir_light);

   //  let mut root = win.factory.group();
   //  win.scene.add(&root);
   // let (group_map, _meshes) = win.factory.load_obj(&path);
   //  for g in group_map.values() {
   //      root.add(g);
   //  }



   //  while  win.update() && !three::KEY_ESCAPE.is_hit(&win.input){
   //      controls.update(&win.input);
   //      win.render(&cam);
   //  }
}
