use lightpack::{Size, Pack, Unpack, byteorder::BigEndian};

#[derive(Size, Pack, Unpack, Debug)]
struct Point {
    x: i16,
    y: i16,
}

fn main() {
    let mut buffer = [0u8; Point::SIZE];
    Point { x: 3, y: 4 }.pack::<BigEndian>(&mut buffer);
    println!("{:?}", buffer); // => [0, 3, 0, 4]

    let point = Point::unpack::<BigEndian>(&[0, 3, 0, 4]);
    println!("{:?}", point); // => Point { x: 3, y: 4 }
}
