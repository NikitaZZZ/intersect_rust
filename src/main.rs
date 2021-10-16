struct Point {
    x: f32,
    y: f32,
}

struct Straight {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    let straight1 = Straight {
        top_left: Point { x: 4.89, y: 2.85 },
        bottom_right: Point { x: 2.59, y: 0.85 }
    };

    let straight2 = Straight {
        top_left: Point { x:4.44, y: 4.82 },
        bottom_right: Point { x: 1.0, y: 2.0 }
    };

    intersect(&straight1, &straight2);
}

fn intersect(straight1: &Straight, straight2: &Straight) {
    if straight1.top_left.x < straight2.top_left.x &&
        straight1.bottom_right.x > straight2.bottom_right.x &&
        straight1.top_left.y > straight2.top_left.y {
            println!("Пересекаются");
    } else if straight1.top_left.x > straight2.top_left.x &&
        straight1.bottom_right.x < straight2.bottom_right.x &&
        straight1.top_left.y < straight2.top_left.y {
            println!("Пересекаются");
    } else {
        println!("Не пересекаются");
    }
}