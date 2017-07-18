use borehole::Point;


struct Rect3D{
    top_left: Point,
    bottom_right: Point,
}


pub struct TargetPlane{
    dip: f32,
    dir: f32,
    offset: Point,
    width: f32,         // along strike
    length: f32,        // perpendicular to strike, or along dip
    basic_rect: Rect3D,
}


impl TargetPlane{
    pub fn new()->TargetPlane{
        TargetPlane{dip: 0.0,
                    dir: 0.0,
                    offset: Point{x:0.0, y:0.0, z:0.0},
                    width: 100.0,         // along strike
                    length: 100.0,        // perpendicular to strike, or along dip
                    basic_rect: Rect3D{top_left: Point{x:0.0, y:0.0, z:0.0},
                                bottom_right: Point{x:0.0, y:0.0, z:0.0},
                            },
                    }
    } 

    pub fn print(&mut self){
        println!["dip: {:.2},  dir: {:.2}", self.dip, self.dir];
        println!["width: {:.2},  length: {:.2}", self.width, self.length];
        println!["Offset x:{:.2}, y:{:.2}, z:{:.2},", self.offset.x, self.offset.y, self.offset.z, ];

    }

    fn recalc(&mut self)->&mut TargetPlane{
        self.basic_rect = Rect3D{
                    top_left: Point{
                        x:-self.width/2.+self.offset.x, 
                        y:-self.length/2.0+self.offset.y, 
                        z:0.0+self.offset.z,
                    },
                    bottom_right: Point{
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