//use borehole::Point;

use cgmath::prelude::*;
use cgmath::{Point3};//, Vector3};



pub struct Rect3D{
    pub top_left: Point3<f32>,
    pub top_right: Point3<f32>,
    pub bottom_right: Point3<f32>,
}


pub struct TargetPlane{
    dip: f32,
    dir: f32,
    offset: Point3<f32>,
    width: f32,         // along strike
    length: f32,        // perpendicular to strike, or along dip
    basic_rect: Rect3D,
}

pub fn projection_from_point(rect : & Rect3D, point : Point3<f32>)->Option<f32>{

    const DEBUG : bool = false;

    // From Eric Bainville on Stackoverflow:
    // Assuming the rectangle is represented by three points A,B,C, 
    // with AB and BC perpendicular, you only need to check the 
    // projections of the query point M on AB and BC:

    // 0 <= dot(AB,AM) <= dot(AB,AB) &&
    // 0 <= dot(BC,BM) <= dot(BC,BC)

    let a = Point3::new(rect.top_left.x,     rect.top_left.y,     rect.top_left.z);
    let b = Point3::new(rect.top_right.x,    rect.top_right.y,    rect.top_right.z);
    let c = Point3::new(rect.bottom_right.x, rect.bottom_right.y, rect.bottom_right.z);

    let ab = b-a;
    let bc = c-b;

    if DEBUG {println!["\n\n\nProjection_from_point\nAB: {:?}", ab]};
    if DEBUG {println!["BC: {:?}", bc]};

    let n = bc.cross(ab);
    let n = n/n.magnitude();
    if DEBUG {println!["normal: {:?}", n]};

    // let n = Vector3::new(0,1,0);
    // plane is Ax +By = Cz + d =0
    // substitute a  point on the plane, like 10, 10, 10:
    // 0*10 + 1*10 + 0*10 + D = 0

    // D = 0-normal.pt use C1 as point:

    let d = 0.0 - a.dot(n);
    if DEBUG {println!["D of plane equation: {:?}", d]};

    // scalar distance point to plane = n.p +d
    let m = point;
    let p = m.to_vec();

    let sd = n.dot(p)+d;

    if DEBUG {println!["scalar distance:\n{:?}", sd]};

    // Then to find projection of p onto plane, 
    // add scalar distance to p, in the direction of None

    let pop = p-sd*n;

    if DEBUG {println!["point on plane:   {:?}", pop]};


    let am = m-a;
    let bm = m-b;


    let l1 = ab.dot(am);
    let l2 = bc.dot(bm);
    

    let point_inside = (0.0 <= l1)&& (l1 <= ab.dot(ab)) &&
                       (0.0 <= l2)&& (l2 <= bc.dot(bc));


    if DEBUG {println!["Inside now on: {}", point_inside]};

    if point_inside {
        Some(sd)
    } else {
        None
    }
}

impl TargetPlane{

    pub fn new()->TargetPlane{
        TargetPlane{dip: 0.0,
                    dir: 0.0,
                    offset: Point3{x:0.0, y:0.0, z:0.0},
                    width: 100.0,         // along strike
                    length: 100.0,        // perpendicular to strike, or along dip
                    basic_rect: Rect3D{top_left: Point3{x:-5.0, y:5.0, z:3.0},
                                       top_right: Point3{x:5.0, y:5.0, z:3.0},
                                       bottom_right: Point3{x:5.0, y:-5.0, z:0.0},
                                      }
        }
    }    

    pub fn print(&mut self){
        println!["dip: {:.2},  dir: {:.2}", self.dip, self.dir];
        println!["width: {:.2},  length: {:.2}", self.width, self.length];
        println!["Offset x:{:.2}, y:{:.2}, z:{:.2},", self.offset.x, self.offset.y, self.offset.z, ];

    }

    fn recalc(&mut self)->&mut TargetPlane{
        self.basic_rect = Rect3D{
                    top_left: Point3{
                        x:-self.width/2.+self.offset.x, 
                        y:-self.length/2.0+self.offset.y, 
                        z:0.0+self.offset.z,
                    },
                    top_right: Point3{
                        x:self.width/2.+self.offset.x, 
                        y:-self.length/2.0+self.offset.y, 
                        z:0.0+self.offset.z,
                    },
                    bottom_right: Point3{
                        x:self.width/2.0+self.offset.x, 
                        y:self.length/2.0+self.offset.y, 
                        z:0.0+self.offset.z,
                    },
                };
        self
    }

    // Move plane upwards or downwards:
    pub fn up(&mut self, up: f32)->&mut TargetPlane{
        self.offset.z += up;
        self.recalc()
    }

    // Move plane upwards or downwards:
    pub fn move_x(&mut self, x: f32)->&mut TargetPlane{
        self.offset.x += x;
        self.recalc()
    }

    // Move plane upwards or downwards:
    pub fn move_y(&mut self, y: f32)->&mut TargetPlane{
        self.offset.y += y;
        self.recalc()
    }

    pub fn move_dir(&mut self, dir: f32)->&mut TargetPlane{
        self.dir += dir;
        self.recalc()
    }

    pub fn move_dip(&mut self, dip: f32)->&mut TargetPlane{
        self.dip += dip;
        self.recalc()
    }


    pub fn set_dir(&mut self, dir: f32)->&mut TargetPlane{
        self.dir = dir;
        self.recalc()
    }

    pub fn set_dip(&mut self, dip: f32)->&mut TargetPlane{
        self.dip = dip;
        self.recalc()
    }

    pub fn set_width(&mut self, val : f32)->&mut TargetPlane{
        self.width = val;
        self.recalc()
    }

    pub fn set_length(&mut self, val : f32)->&mut TargetPlane{
        self.length = val;
        self.recalc()
    }
}