use arrayvec::ArrayVec;
use kiss3d::{nalgebra::UnitQuaternion, window::Window};

fn main() {
    let mut w = Window::new("rotvis");
    w.set_framerate_limit(None);
    w.set_light(kiss3d::light::Light::StickToCamera);

    let mut c = w.add_cube(0.3, 0.01, 0.1);

    let mut r = linereader::LineReader::new(std::io::stdin());

    'outer: while w.render() {
        if let Some(Ok(line)) = r.next_line() {
            let len = line.len();
            let d = match std::str::from_utf8(&line[..len - 1]) {
                Ok(v) => v,
                Err(_) => continue 'outer,
            };

            let mut v = ArrayVec::<f32, 3>::new();

            for i in d.splitn(3, " ") {
                match lexical::parse_lossy(i) {
                    Ok(x) => v.push(x),
                    Err(_) => continue 'outer,
                }
            }

            if v.len() != 3 {
                continue 'outer;
            }

            let rot = UnitQuaternion::from_euler_angles(v[1], 0., v[0]);
            c.set_local_rotation(rot);
        }
    }
}
