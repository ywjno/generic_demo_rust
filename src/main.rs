#[derive(Debug)]
struct Rectangle<T, V> {
    width: T,
    height: V,
}

impl<T, V> Rectangle<T, V> {
    fn width(&self) -> &T {
        &self.width
    }
}

impl Rectangle<u16, u16> {
    fn perimeter(&self) -> u16 {
        2 * (self.width + self.height)
    }
}

// <T: PartialOrd> 相当于 java 的 <T extends Comparable<T>>
fn get_max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    let rect = Rectangle {
        width: 8_u16,
        height: 6.7f64,
    };
    println!("{:?}", rect);
    println!("{:?}", rect.width());
    let rect = Rectangle {
        width: 10u16,
        height: 20_u16,
    };
    println!("{:?}", rect.perimeter());
    println!("{:?}", get_max(32, 64));
}
