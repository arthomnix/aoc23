use std::fmt::Debug;
use std::str::FromStr;
use z3::{Config, Context, Solver};
use z3::ast::{Ast, Int};

fn intersect_2d(p1: (f64, f64), v1: (f64, f64), p2: (f64, f64), v2: (f64, f64)) -> Option<(f64, f64)> {
    let (px1, py1) = p1;
    let (px2, py2) = p2;
    let (vx1, vy1) = v1;
    let (vx2, vy2) = v2;

    let m1 = vy1 / vx1;
    let c1 = py1 - m1 * px1;

    let m2 = vy2 / vx2;
    let c2 = py2 - m2 * px2;

    if m1 == m2 {
        return None;
    }

    let x = (c1 - c2) / (m2 - m1);
    let y = m1 * x + c1;
    Some((x, y))
}

fn to_2d(v: (f64, f64, f64)) -> (f64, f64) {
    (v.0, v.1)
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Hailstone<T: Copy + Clone + Debug + PartialEq> {
    pos: (T, T, T),
    vel: (T, T, T),
}

fn check_intersection_2d(a: Hailstone<f64>, b: Hailstone<f64>, min: f64, max: f64) -> bool {
    let p1 = to_2d(a.pos);
    let p2 = to_2d(b.pos);
    let v1 = to_2d(a.vel);
    let v2 = to_2d(b.vel);

    if let Some((x, y)) = intersect_2d(p1, v1, p2, v2) {
        (x - p1.0).signum() == v1.0.signum()
            && (y - p1.1).signum() == v1.1.signum()
            && (x - p2.0).signum() == v2.0.signum()
            && (y - p2.1).signum() == v2.1.signum()
            && x >= min && x <= max
            && y >= min && y <= max
    } else {
        false
    }
}

fn parse_input<T>(input: String) -> Vec<Hailstone<T>>
where
    T: Copy + Clone + Debug + PartialEq + FromStr,
    <T as FromStr>::Err: Debug,
{
    input.lines().map(|line| {
        let (p, v) = line.split_once(" @ ").unwrap();
        let positions = p.split(", ").map(|s| T::from_str(s.trim()).unwrap()).collect::<Vec<_>>();
        let pos = (positions[0], positions[1], positions[2]);

        let velocities = v.split(", ").map(|s| T::from_str(s.trim()).unwrap()).collect::<Vec<_>>();
        let vel = (velocities[0], velocities[1], velocities[2]);

        Hailstone { pos, vel }
    }).collect::<Vec<_>>()
}

pub(crate) fn part1(input: String) {
    let mut sum = 0;
    let hailstones = parse_input(input);
    for i in 0..hailstones.len() {
        for j in (i + 1)..hailstones.len() {
            if check_intersection_2d(hailstones[i], hailstones[j], 200000000000000.0, 400000000000000.0) {
                sum += 1;
            }
        }
    }

    println!("{sum}");
}

pub(crate) fn part2(input: String) {
    let hailstones = parse_input(input);
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let px = Int::new_const(&ctx, "px");
    let py = Int::new_const(&ctx, "py");
    let pz = Int::new_const(&ctx, "pz");
    let vx = Int::new_const(&ctx, "vx");
    let vy = Int::new_const(&ctx, "vy");
    let vz = Int::new_const(&ctx, "vz");

    for hailstone in hailstones {
        let pxn = Int::from_i64(&ctx, hailstone.pos.0);
        let pyn = Int::from_i64(&ctx, hailstone.pos.1);
        let pzn = Int::from_i64(&ctx, hailstone.pos.2);
        let vxn = Int::from_i64(&ctx, hailstone.vel.0);
        let vyn = Int::from_i64(&ctx, hailstone.vel.1);
        let vzn = Int::from_i64(&ctx, hailstone.vel.2);
        let tn = Int::fresh_const(&ctx, "t");

        solver.assert(&(&pxn + &vxn * &tn)._eq(&(&px + &vx * &tn)));
        solver.assert(&(&pyn + &vyn * &tn)._eq(&(&py + &vy * &tn)));
        solver.assert(&(&pzn + &vzn * &tn)._eq(&(&pz + &vz * &tn)));
    }

    solver.check();
    let model = solver.get_model().unwrap();
    let x = model.get_const_interp(&px).unwrap().as_i64().unwrap();
    let y = model.get_const_interp(&py).unwrap().as_i64().unwrap();
    let z = model.get_const_interp(&pz).unwrap().as_i64().unwrap();
    println!("{}", x + y + z);
}
