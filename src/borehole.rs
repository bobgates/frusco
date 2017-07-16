//extern crate nalgebra as na;
//use na::{Vector3, Rotation3};

extern crate rust_3d;


//use rust_3d::point_3d::Point3D as Point;

#[derive(Debug, Clone, Copy)]
pub struct SurveyPoint{
    pub downhole: f32,
    pub azimuth: f32,
    pub inclination: f32,
}

pub struct Borehole{
    pub survey: Vec<Option<SurveyPoint>>,
    stepsize: Option<f32>,
    stepcount: u32,
}

impl Borehole{
    pub fn new()->Borehole{
        Borehole{
            survey: Vec::new(),
            stepsize: None,
            stepcount: 0,
        }
    }
    pub fn add_survey_obs(&mut self, downhole: f32, azimuth: f32, inclination: f32)->&mut Borehole{
        self.survey.push(Some(SurveyPoint{downhole:downhole, azimuth:azimuth, inclination:inclination}));
        self
    }
    pub fn add_point(&mut self, p: SurveyPoint)->&mut Borehole{
        self.survey.push(Some(p));
        self
    }
    pub fn set_step(&mut self, step: f32)->&mut Borehole{
        self.stepsize = Some(step);
        self
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



}

impl Iterator for Borehole{
    type Item = SurveyPoint;

    fn next(&mut self) -> Option<Self::Item> {
        self.stepcount += 1;

        if self.stepcount < 10 {
            Some(SurveyPoint{downhole:0.0, azimuth:0.0, inclination: self.stepcount as f32})
        } else {
            None
        }


    }
}

// }

// impl Borehole{
//     pub fn new()->Borehole{

//     }


//     pub fn generate(&mut self, collar: Coord, start: Coord){




//     }
// }