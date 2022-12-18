// dist(position, sensor) <= dist(sensor, closest_beacon) -> can't be in this sensor range
// cant_be = false;
// for position
//    for every sensor:
//      if dist(position, sensor) <= dist(sensor, closest_beacon)
//          cant_be = true;

struct Sensor {
    x: i32,
    y: i32,
    closest_beacon: Beacon,
}

struct Beacon {
    x: i32,
    y: i32,
}

impl Sensor {
    fn parse(str_line: &str) -> Sensor {
        Sensor {
            x: 0,
            y: 0,
            closest_beacon: Beacon { x: 0, y: 0 },
        }
    }
}

pub fn solve() {}
