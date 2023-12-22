use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3D {
    fn add_axis(&self, axis: Axis, amount: i32) -> Self {
        match axis {
            Axis::X => Point3D { x: self.x + amount, y: self.y, z: self.z },
            Axis::Y => Point3D { x: self.x, y: self.y + amount, z: self.z },
            Axis::Z => Point3D { x: self.x, y: self.y, z: self.z + amount },
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Brick {
    axis: Axis,
    start: Point3D,
    length: i32,
}

impl Brick {
    fn contains_any_point(&self, points: &[Point3D]) -> bool {
        (0..self.length).any(|p| points.contains(&self.start.add_axis(self.axis, p)))
    }

    fn get_bottom_points(&self) -> Vec<Point3D> {
        if self.axis == Axis::Z {
            vec![self.start]
        } else {
            (0..self.length).map(|p| self.start.add_axis(self.axis, p)).collect::<Vec<_>>()
        }
    }

    fn get_supporting_bricks(&self, others: &[Brick]) -> usize {
        let bottom_points = Brick {
            axis: self.axis,
            start: self.start.add_axis(Axis::Z, -1),
            length: self.length,
        }.get_bottom_points();
        others.iter().filter(|&b| b != self && b.contains_any_point(&bottom_points)).count()
    }

    fn move_down_if_possible(&self, others: &[Brick]) -> (Brick, bool) {
        if self.get_supporting_bricks(others) == 0 && self.start.z > 1 {
            (Brick {
                axis: self.axis,
                start: self.start.add_axis(Axis::Z, -1),
                length: self.length,
            }, true)
        } else {
            (*self, false)
        }
    }

    fn is_supported_by(&self, other: Brick) -> bool {
        self.get_supporting_bricks(&[other]) == 1
    }

    fn can_disintegrate(&self, others: &[Brick]) -> bool {
        for brick in others {
            if brick.is_supported_by(*self) && brick.get_supporting_bricks(others) == 1 {
                return false;
            }
        }
        true
    }
}

fn parse_input(input: String) -> Vec<Brick> {
    input.lines().map(|line| {
        let (c1, c2) = line.split_once('~').unwrap();
        let coords1 = c1.split(',').map(|c| i32::from_str(c).unwrap()).collect::<Vec<_>>();
        let coords2 = c2.split(',').map(|c| i32::from_str(c).unwrap()).collect::<Vec<_>>();
        let (x1, y1, z1) = (coords1[0], coords1[1], coords1[2]);
        let (x2, y2, z2) = (coords2[0], coords2[1], coords2[2]);

        let (axis, length) = if x1 == x2 && y1 == y2 {
            (Axis::Z, z2.abs_diff(z1) as i32 + 1)
        } else if x1 == x2 && z1 == z2 {
            (Axis::Y, y2.abs_diff(y1) as i32 + 1)
        } else {
            (Axis::X, x2.abs_diff(x1) as i32 + 1)
        };

        Brick {
            axis,
            start: Point3D { x: x1, y: y1, z: z1 },
            length,
        }
    }).collect::<Vec<Brick>>()
}

fn fall_bricks(bricks: &mut [Brick]) -> usize {
    let mut fell = vec![false; bricks.len()];
    let mut moved_any = true;
    while moved_any {
        moved_any = false;
        for i in 0..bricks.len() {
            let (brick, moved) = bricks[i].move_down_if_possible(&bricks);
            if moved {
                moved_any = true;
                fell[i] = true;
                bricks[i] = brick;
            }
        }
    }
    fell.into_iter().filter(|&b| b).count()
}

pub(crate) fn part1(input: String) {
    let mut bricks = parse_input(input);
    fall_bricks(&mut bricks);
    println!("{}", bricks.iter().filter(|&b| b.can_disintegrate(&bricks)).count());
}

pub(crate) fn part2(input: String) {
    let mut bricks = parse_input(input);
    fall_bricks(&mut bricks);

    let mut total_fell = 0;
    for i in 0..bricks.len() {
        println!("processing brick {}/{}", i + 1, bricks.len());
        let mut b = bricks.clone();
        b.remove(i);
        total_fell += fall_bricks(&mut b);
    }

    println!("{total_fell}");
}
