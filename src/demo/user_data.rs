use graph1::primitives::point::Point;

pub struct Bouncy{
    pub x: i32,
    pub y: i32,
    pub dx: i32,
    pub dy: i32,
}

pub struct Ghosts{
    pub direction: Point<i32>,
    pub current_point: Point<i32>,
}

pub struct DemoUserData {
    pub bouncy: Bouncy,
    pub ghosts: Ghosts,

}
