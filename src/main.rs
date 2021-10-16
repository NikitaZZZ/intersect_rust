struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    let rect1 = Rectangle {
        top_left: Point { x: 4.89, y: 2.85 },
        bottom_right: Point { x: 2.59, y: 0.85 }
    };

    let rect2 = Rectangle {
        top_left: Point { x:4.44, y: 4.82 },
        bottom_right: Point { x: 1.0, y: 2.0 }
    };

    intersect(&rect1, &rect2);
}

fn intersect(rect1: &Rectangle, rect2: &Rectangle) {
    if rect1.top_left.x < rect2.top_left.x &&
        rect1.bottom_right.x > rect2.bottom_right.x &&
        rect1.top_left.y > rect2.top_left.y {
            println!("Пересекаются");
    } else if rect1.top_left.x > rect2.top_left.x &&
        rect1.bottom_right.x < rect2.bottom_right.x &&
        rect1.top_left.y < rect2.top_left.y {
            println!("Пересекаются");
    } else {
        println!("Не пересекаются");
    }
}