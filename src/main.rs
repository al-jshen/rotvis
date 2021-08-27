use arrayvec::ArrayVec;
use kiss3d::{nalgebra::UnitQuaternion, window::Window};

fn main() {
    let mut w = Window::new("rotvis");
    w.set_framerate_limit(None);
    w.set_light(kiss3d::light::Light::StickToCamera);

    let mut c = w.add_cube(0.3, 0.01, 0.1);

    let mut r = linereader::LineReader::new(std::io::stdin());

    while w.render() {
        if let Some(Ok(line)) = r.next_line() {
            let len = line.len();
            let d = std::str::from_utf8(&line[..len - 1]).unwrap();
            let v = d
                .split(" ")
                .map(|x| lexical::parse_lossy(x).unwrap())
                .collect::<ArrayVec<f32, 3>>();
            let rot = UnitQuaternion::from_euler_angles(
                v[1] * std::f32::consts::PI / 180.,
                0.,
                v[0] * std::f32::consts::PI / 180.,
            );
            c.set_local_rotation(rot);
        }
    }
}
