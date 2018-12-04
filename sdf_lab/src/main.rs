use std::ops;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: f32,
    y: f32,
}

fn p(x: f32, y: f32) -> Point {
    Point { x, y }
}

impl ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        p(self.x + other.x, self.y + other.y)
    }
}

#[derive(Clone, Copy, Debug)]
struct Line {
    p0: Point,
    p1: Point,
}

fn l(p0: Point, p1: Point) -> Line {
    Line { p0, p1 }
}

fn sd_l_p(l: &Line, pq: &Point) -> f32 {
    let x0 = l.p0.x;
    let y0 = l.p0.y;
    let x1 = l.p1.x;
    let y1 = l.p1.y;
    let xq = pq.x;
    let yq = pq.y;

    // Let v_a = p1 - p0
    // Let v_b = [[0 1][-1 0]]v_a (perpendicular, rot 90 clock).
    // Solve p0 + a*v_a = pq + b*v_b.
    // lmagsq = (x1 - x0)^2 + (y1 - y0)^2.
    // a*lmagsq = (xq - x0)*(x1 - x0) + (yq - y0)*(y1 - y0)
    // b*lmagsq = (yq - y0)*(x1 - x0) - (xq - x0)*(y1 - y0)
    // if a > 1 || a < 0 then pq is closest to p1 or p0 respectively, not the line.
    // if b > 0, pq lies on the left, if b < 0 pq lies on the right.

    // No need to actually divide a by lmagsq, and b not yet.
    let a = (xq - x0)*(x1 - x0) + (yq - y0)*(y1 - y0);
    let b = (yq - y0)*(x1 - x0) - (xq - x0)*(y1 - y0);
    let lmagsq = (x1 - x0).powi(2) + (y1 - y0).powi(2);

    if a > lmagsq {
        // Closest to p1.
        ((xq - x1).powi(2) + (yq - y1).powi(2)).sqrt()*b.signum()
    } else if a < 0.0 {
        // Closest to p0.
        ((xq - x0).powi(2) + (yq - y0).powi(2)).sqrt()*b.signum()
    } else {
        // On one of the sides between p0 and p1.
        b/lmagsq.sqrt()
    }
}

fn closed_polygon_to_lines(points: &[Point]) -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::with_capacity(points.len());
    for window in points.windows(2) {
        lines.push(l(window[0], window[1]));
    }
    if points.len() > 0 {
        lines.push(l(*points.last().unwrap(), *points.first().unwrap()));
    }
    lines
}

fn offset_lines(lines: &mut [Line], offset: Point) {
    for line in lines.iter_mut() {
        *line = l(line.p0 + offset, line.p1 + offset);
    }
}

fn main() {
    // Square.
    // let edges = closed_polygon_to_lines(&[
    //     p(0.0, 2.0),
    //     p(2.0, 2.0),
    //     p(2.0, 4.0),
    //     p(0.0, 4.0),
    // ]);

    // Heart.
    let mut edges = closed_polygon_to_lines(&[
        p( 6.0,  0.0),
        p( 7.0,  0.0),
        p(13.0,  6.0),
        p(13.0, 11.0),
        p(10.0, 14.0),
        p( 8.0, 14.0),
        p( 6.5, 12.5),
        p( 5.0, 14.0),
        p( 3.0, 14.0),
        p( 0.0, 11.0),
        p( 0.0,  6.0),
    ]);

    let b = 5;
    offset_lines(&mut edges[..], p(b as f32, b as f32));

    let w = 13 + b*2;
    let h = 14 + b*2;

    let mut values: Vec<f32> = Vec::with_capacity(w*h);

    for ir in 0..h {
        let y = (h - 1 - ir) as f32 + 0.5f32;
        for ic in 0..w {
            let x = ic as f32 + 0.5;
            let mut sd = std::f32::INFINITY;
            for l in &edges {
                let new_sd = sd_l_p(l, &p(x, y));
                if new_sd.abs() < sd.abs() {
                    sd = new_sd;
                }
            }
            values.push(sd);
        }
    }

    fn unicode_shade(level: u32) -> &'static str {
        match level {
            0 => "\u{2588}",
            1 => "\u{2593}",
            2 => "\u{2592}",
            3 => "\u{2591}",
            _ => " ",
        }
    }

    for ir in 0..h {
        for ic in 0..w {
            let v = values[ir*w + ic];
            let level = v as u32;
            print!("{}", unicode_shade(level));
            if ic == w - 1 {
                println!();
            }
            // print!("{:>7.4}", values[ir*w + ic]);
            // if ic == w - 1 {
            //     println!("");
            // } else {
            //     print!(" ");
            // }
        }
    }
}
