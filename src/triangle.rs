
#[derive(Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point{
            x,
            y,
        }
    }
}

#[derive(Clone)]
pub struct Triangle {
    p0: Point,
    p1: Point,
    p2: Point,
}

impl Triangle {
    pub fn new(p0: Point, p1: Point, p2: Point) -> Self {
        Triangle {
            p0,
            p1,
            p2,
        }
    }

    // blatant copy paste
    // https://stackoverflow.com/questions/2049582/how-to-determine-if-a-point-is-in-a-2d-triangle
    pub fn contains(&self, p: &Point) -> bool {
        let p0 = &self.p0;
        let p1 = &self.p1;
        let p2 = &self.p2;
        let a = 0.5 * (-p1.y * p2.x + p0.y * (-p1.x + p2.x) + p0.x * (p1.y - p2.y) + p1.x * p2.y) as f64;
        let sign = if a < 0.0 {
            -1
        } else {
            1
        } as f64;
        let s = (p0.y * p2.x - p0.x * p2.y + (p2.y - p0.y) * p.x + (p0.x - p2.x) * p.y) as f64 * sign;
        let t = (p0.x * p1.y - p0.y * p1.x + (p0.y - p1.y) * p.x + (p1.x - p0.x) * p.y) as f64 * sign;

        return s > 0.0 && t > 0.0 && (s + t) < 2.0 * a * sign;
    }
}
