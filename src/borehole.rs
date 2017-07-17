//extern crate nalgebra as na;
//use na::{Vector3, Rotation3};

use std::cmp::Ordering::Less;


#[derive(Debug, Clone, Copy)]
pub struct SurveyPoint{
    pub downhole: f32,
    pub azimuth: f32,
    pub inclination: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Point{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Coord{
    depth: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Borehole{
    survey: Vec<SurveyPoint>,
    coords: Vec<Coord>,
    collar: Option<Point>,
    stepsize: Option<f32>,
    stepcount: usize,
}

impl Borehole{
    pub fn new()->Borehole{
        Borehole{
            survey: Vec::new(),
            coords: Vec::new(),
            collar: None,
            stepsize: None,
            stepcount: 0,
        }
    }

    pub fn set_collar(&mut self, x : f32, y: f32, z: f32)->&mut Borehole{
        self.collar = Some(Point{x: x, y: y, z: z});
        self
    }

    fn sort_observations(&mut self){
        self.survey.sort_by(|a, b| a.clone().downhole.partial_cmp(&b.clone().downhole).unwrap_or(Less));
    }

// The deepest entry in the survey is the bottom of hole. If the hole extends
// beyond the deepest survey position, then use add_bottom_of_hole, which will
// fake the azimuth and inclination by copying those of the previous survey position.


    // Create a parallel coords struct that contains depth, x, y, z
    // for each observation point in the survey record.
    fn make_coords(&mut self)->&mut Borehole {
            // The division was valid
        match self.collar {
            Some(col) => {
                println!["entering make_coords Some match"];
                self.coords.clear();
                let mut coord = Coord{depth: 0.0, 
                                      x:self.collar.unwrap().x,
                                      y:self.collar.unwrap().y,
                                      z:self.collar.unwrap().z,
                                };
                self.coords.push(coord);

                 






            },
            None    => println!("Collar not yet defined in make_coords"),
        }
        self
    }

    pub fn add_survey_obs(&mut self, downhole: f32, azimuth: f32, inclination: f32)->&mut Borehole{
        self.add_survey_point(SurveyPoint{downhole:downhole, azimuth:azimuth.to_radians(), inclination:inclination.to_radians()})
    }

    pub fn add_survey_point(&mut self, p: SurveyPoint)->&mut Borehole {
        self.survey.push(p);
        self.sort_observations();
        self.make_coords();
        self
    }

    pub fn add_bottom_of_hole(&mut self, bottom_depth: f32)->&mut Borehole {
        //  Shouldn't allow adding bottom of hole if hole has no coordinates...
        if self.survey.len()>0{
            let last_survey_point = self.survey[self.survey.len()-1];
            self.add_survey_point(SurveyPoint{downhole:bottom_depth,
                                       azimuth:last_survey_point.azimuth, 
                                       inclination:last_survey_point.inclination});
        }
        self
    }

    pub fn set_step(&mut self, step: f32)->&mut Borehole {
        self.stepsize = Some(step);
        self
    }

    pub fn depth(&mut self)-> f32{
        self.survey[self.survey.len()-1].downhole
    }

    fn next_from_here(coord : Point, here: SurveyPoint, next: SurveyPoint)->Coord{

            let mut a1 = here.azimuth;
            let mut a2 = next.azimuth;
            let mut i1 = here.inclination;
            let mut i2 = next.inclination;
            let mut md = next.downhole - here.downhole;
            let mut eps = (depth - here.downhole)/md;

// This code from http://www.drillingformulas.com/minimum-curvature-method/ 
// calculates the X, Y, Z of the bottom station based on the position of the top one
            let beta = ((i2-i1).cos() - (i1.sin()*i2.sin()*(1.0-(a2-a1).cos()))).acos();
            let rf = if beta == 0.0 { 1.0 } else { 2./beta * (beta/2.0).tan() };
            let north = md/2.0*(i1.sin()*a1.cos() +i2.sin()*a2.cos())*rf;
            let east = md/2.0*(i1.sin()*a1.sin() +i2.sin()*a2.sin())*rf;
            let depth = md/2.0*(i1.cos()+i2.cos())*rf;

        Coord{depth: next.downhole, 
                x: coord.x + east, 
                y: coord.y + north, 
                z: coord.z + depth}
    }


    pub fn interpolate(&mut self, depth : f32)->Option<Point>{
        if depth>self.depth() || depth <0.0{
            None
        } else {

            let mut here_point = SurveyPoint{downhole: 0.0, azimuth: 0.0, inclination: 0.0};
            let mut next = here.clone();
            

            for i in 0..self.survey.len()-1{
                let above = self.survey[i].downhole;
                let below = self.survey[i+1].downhole;
                //println!["***{}  {}   {}", depth, above, below];
                if depth>=above && depth <=below{
                    here = self.survey[i];
                    next = self.survey[i+1];


                    //println!["***{}  {}   {}", depth, above, below];
                    break;
                }

            }
            let next_point = next_from_here(here_point, here, next);

            println!["{:?}", next_point];





// To calculate the coords at an arbitrary position use Black and Clark 91.Clark
// Black and Clarke use gamma where the previous section uses Beta
// Given the first station is P1, the second station is P2, and I want
// position somewhere between, at P3, then: 
//
//  P3 = P1 + L/gamma * tan(eps.gamma/2)*[(k1+1)t1 + k2t2]
//
// where cos(gamma) = t1.t2
//          t1, t2 are tangent vectors at the two stations
//          L is the length of arc between thte two stations
//          k1 = cos(eps.gamma) -cos(gamma).cos((1-eps).gamma)/
//                    sin^2(gamma)
//          k2 = sin(eps.gamma)/sin(gamma)


            let k1 = if beta==0.0 {1.0-eps} else {((eps*beta).cos()-beta.cos()*((1.0-eps)*beta).cos())/beta.sin()/beta.sin()};
            let k2 = if beta==0.0 { eps } else {(eps*beta).sin()/beta.sin()};

            let north = md/2.0*(i1.sin()*a1.cos()*(1.0-k1) +i2.sin()*a2.cos()*(k2))*rf;
            let east = md/2.0*(i1.sin()*a1.sin()*(1.0-k1) +i2.sin()*a2.sin()*(k2))*rf;
            let depth = md/2.0*(i1.cos()*(1.0-k1)+i2.cos()*(k2))*rf;

            Some(Point{x:east, y:north, z:depth})
            
        }
    }

}


// dmd = Distance2 - Distance1
// B = acos(cos(i2 - i1) - (sin(i1)*sin(i2)*(1-cos(a2-a1))))
// rf = 2 / B * tan(B / 2)
// dX = dmd/2 * (sin(i1)*sin(a1) + sin(i2)*sin(a2))*rf
// dY = dmd/2 * (sin(i1)*cos(a1) + sin(i2)*cos(a2))*rf
// dZ = dmd/2 * (cos(i1) + cos(i2))*rf

// X2 = X1 + dX
// Y2 = Y1 + dX
// Z2 = Z1 + dX



impl Iterator for Borehole {
    type Item = SurveyPoint;

    fn next(&mut self) -> Option<Self::Item> {
        self.stepcount += 1;
        if self.stepcount <= self.survey.len(){
            Some(self.survey[self.stepcount-1])
        } else {
            None
        }
    }
}

