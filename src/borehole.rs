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
pub struct Coord{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Borehole{
    pub survey: Vec<SurveyPoint>,
    stepsize: Option<f32>,
    stepcount: usize,
}

impl Borehole{
    pub fn new()->Borehole{
        Borehole{
            survey: Vec::new(),
            stepsize: None,
            stepcount: 0,
        }
    }

    fn sort_observations(&mut self){
        self.survey.sort_by(|a, b| a.clone().downhole.partial_cmp(&b.clone().downhole).unwrap_or(Less));
    }

// The deepest entry in the survey is the bottom of hole. If the hole extends
// beyond the deepest survey position, then use add_bottom_of_hole, which will
// fake the azimuth and inclination by copying those of the previous survey position.

    pub fn add_survey_obs(&mut self, downhole: f32, azimuth: f32, inclination: f32)->&mut Borehole{
        self.add_survey_point(SurveyPoint{downhole:downhole, azimuth:azimuth.to_radians(), inclination:inclination.to_radians()})
    }

    pub fn add_survey_point(&mut self, p: SurveyPoint)->&mut Borehole {
        self.survey.push(p);
        self.sort_observations();
        self
    }

    pub fn add_bottom_of_hole(&mut self, bottom_depth: f32)->&mut Borehole {
        //  Shouldn't allow adding bottom of hole if hole has no coordinates...
        if self.survey.len()>0{
            let last_coord = self.survey[self.survey.len()-1];
            self.add_survey_point(SurveyPoint{downhole:bottom_depth,
                                       azimuth:last_coord.azimuth, 
                                       inclination:last_coord.inclination});
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


    pub fn interpolate(&mut self, depth : f32)->Option<Coord>{
        if depth>self.depth() || depth <0.0{
            None
        } else {


            let mut A1 = 0.0;
            let mut A2 = 0.0;
            let mut I1 = 0.0;
            let mut I2 = 0.0;
            let mut MD = 0.0;
            let mut eps = 0.0;
            

            for i in 0..self.survey.len()-1{
                let above = self.survey[i].downhole;
                let below = self.survey[i+1].downhole;
                //println!["***{}  {}   {}", depth, above, below];
                if depth>=above && depth <=below{
                    A1 = self.survey[i].azimuth;
                    A2 = self.survey[i+1].azimuth;
                    I1 = self.survey[i].inclination;
                    I2 = self.survey[i+1].inclination;
                    MD = self.survey[i+1].downhole - self.survey[i].downhole;
                    eps = (depth - self.survey[i].downhole)/MD;
                    //println!["***{}  {}   {}", depth, above, below];
                    break;
                }

            }

// This code from http://www.drillingformulas.com/minimum-curvature-method/ 
// calculates the X, Y, Z of the bottom station based on the position of the top one
            let beta = ((I2-I1).cos() - (I1.sin()*I2.sin()*(1.0-(A2-A1).cos()))).acos();
            let RF = if beta == 0.0 { 1.0 } else { 2./beta * (beta/2.0).tan() };
            let North = MD/2.0*(I1.sin()*A1.cos() +I2.sin()*A2.cos())*RF;
            let East = MD/2.0*(I1.sin()*A1.sin() +I2.sin()*A2.sin())*RF;
            let Depth = MD/2.0*(I1.cos()+I2.cos())*RF;


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

            let N3 = MD/2.0*(I1.sin()*A1.cos()*(1.0-k1) +I2.sin()*A2.cos()*(k2))*RF;
            let E3 = MD/2.0*(I1.sin()*A1.sin()*(1.0-k1) +I2.sin()*A2.sin()*(k2))*RF;
            let D3 = MD/2.0*(I1.cos()*(1.0-k1)+I2.cos()*(k2))*RF;

            Some(Coord{x:East, y:North, z:Depth})
            
        }
    }

}


// dMD = Distance2 - Distance1
// B = acos(cos(I2 - I1) - (sin(I1)*sin(I2)*(1-cos(A2-A1))))
// RF = 2 / B * tan(B / 2)
// dX = dMD/2 * (sin(I1)*sin(A1) + sin(I2)*sin(A2))*RF
// dY = dMD/2 * (sin(I1)*cos(A1) + sin(I2)*cos(A2))*RF
// dZ = dMD/2 * (cos(I1) + cos(I2))*RF

// X2 = X1 + dX
// Y2 = Y1 + dX
// Z2 = Z1 + dX



impl Iterator for Borehole{
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

