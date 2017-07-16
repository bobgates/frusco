//extern crate nalgebra as na;
//use na::{Vector3, Rotation3};

#[derive(Debug)]
pub struct Point{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Borehole{
    collar: Option<Point>,
    end: Option<Point>,
    stepsize: Option<f32>,
    stepcount: u32,
}

impl Borehole{
    pub fn new()->Borehole{
        Borehole{
            collar: None,
            end: None,
            stepsize: None,
            stepcount: 0,
        }
    }
    pub fn set_collar(&mut self, x: f32, y: f32, z: f32)->&mut Borehole{
        self.collar=Some(Point{x:x, y:y, z:z});
        self
    }
    pub fn set_collar_point(&mut self, p: Point)->&mut Borehole{
        self.collar=Some(p);
        self
    }
    pub fn set_end(&mut self, x: f32, y: f32, z: f32)->&mut Borehole{
        self.end=Some(Point{x:x, y:y, z:z});
        self
    }
    pub fn set_end_point(&mut self, p: Point)->&mut Borehole{
        self.end=Some(p);
        self
    }
    pub fn set_step(&mut self, step: f32)-> &mut Borehole{
        self.stepsize=Some(step);
        self
    }
}

impl Iterator for Borehole{
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        self.stepcount += 1;

        if self.stepcount < 10 {
            Some(Point{x:0.0, y:0.0, z: self.stepcount as f32})
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