use std::f64::consts::PI;
use std::time::{SystemTime, UNIX_EPOCH};

const C: f64 = 3.0;
const SCREEN_WIDTH: usize = 50;
const SCREEN_HEIGHT: usize = 25;

// Cube定义为一个Vec<Vec<[f64; 3]>>，因为长度是可变的
const CUBE: [[[f64; 3]; 4]; 6] = [
    [[-0.5, -0.5, 0.5], [0.5, -0.5, 0.5], [-0.5, 0.5, 0.5], [0.0, 0.0, 1.0]],
    [[-0.5, -0.5, 0.5], [-0.5, -0.5, -0.5], [-0.5, 0.5, 0.5], [-1.0, 0.0, 0.0]],
    [[-0.5, -0.5, 0.5], [-0.5, -0.5, -0.5], [0.5, -0.5, 0.5], [0.0, -1.0, 0.0]],
    [[-0.5, 0.5, 0.5], [0.5, 0.5, 0.5], [-0.5, 0.5, -0.5], [0.0, 1.0, 0.0]],
    [[0.5, -0.5, 0.5], [0.5, -0.5, -0.5], [0.5, 0.5, 0.5], [1.0, 0.0, 0.0]],
    [[-0.5, -0.5, -0.5], [0.5, -0.5, -0.5], [-0.5, 0.5, -0.5], [0.0, 0.0, -1.0]],
];

const FACE: [[[i32; 3]; 3]; 6] = [
    [[0, 0, 0], [0, 1, 0], [0, 0, 0]],
    [[0, 0, 1], [0, 0, 0], [1, 0, 0]],
    [[0, 0, 1], [0, 1, 0], [1, 0, 0]],
    [[1, 0, 1], [0, 0, 0], [1, 0, 1]],
    [[1, 0, 1], [0, 1, 0], [1, 0, 1]],
    [[1, 0, 1], [1, 0, 1], [1, 0, 1]],
];

struct TimerCount {
    start_time: u128,
    frames: u64,
}


fn judge_face(id: usize, x: f64, y: f64) -> i32 {
    FACE[id][(3.0 * y) as usize][(3.0 * x) as usize]
}

fn initialize_cube() -> Vec<Vec<[f64; 3]>> {
    let mut initialized_cube = Vec::new();
    for i in 0..6 {
        let mut face_vertices = Vec::new();
        for j in 0..4 {
            let x = CUBE[i][j][0];
            let y = CUBE[i][j][1];
            let z = CUBE[i][j][2];
            let x_new = (PI.sqrt() / 6.0 + 0.5) * x - PI.sqrt() / 3.0 * y + (-0.5 + PI.sqrt() / 6.0) * z;
            let y_new = (PI.sqrt() / 3.0) * x + (PI.sqrt() / 3.0) * y + (PI.sqrt() / 3.0) * z;
            let z_new = (-0.5 + PI.sqrt() / 6.0) * x - PI.sqrt() / 3.0 * y + (PI.sqrt() / 6.0 + 0.5) * z;
            face_vertices.push([x_new, y_new, z_new]);
        }
        initialized_cube.push(face_vertices);
    }
    initialized_cube
}

fn render_frame() {
    let mut time_stamp = 0.0;
    let initialized_cube = initialize_cube();
    let mut time_rec = TimerCount {
        start_time: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis(),
        frames: 0,
    };
    loop {
        let mut z_buffer: Vec<Vec<f64>> = vec![vec![0.0; SCREEN_WIDTH + 1]; SCREEN_HEIGHT + 1];
        let mut output: Vec<Vec<char>> = vec![vec![' '; SCREEN_WIDTH + 1]; SCREEN_HEIGHT + 1];
        time_stamp += 0.01;
        for i in 0..6 {
            for u in (0..100).step_by(1) {
                for v in (0..100).step_by(1) {
                    calculate_frame_data(
                        i,
                        time_stamp,
                        &initialized_cube,
                        &mut z_buffer,
                        u as f64/100 as f64,
                        v as f64/100 as f64,
                        &mut output,
                    );
                }
            }
        }
        let ok = draw(&output, &mut time_rec);
        if !ok {
            return;
        }
    }
}

fn calculate_frame_data(
    i: usize,
    time_stamp: f64,
    cube: &Vec<Vec<[f64; 3]>>,
    z_buffer: &mut Vec<Vec<f64>>,
    u: f64,
    v: f64,
    output: &mut Vec<Vec<char>>,
) {
    let m_x = cube[i][1][0] - cube[i][0][0];
    let m_y = cube[i][1][1] - cube[i][0][1];
    let m_z = cube[i][1][2] - cube[i][0][2];

    let n_x = cube[i][2][0] - cube[i][0][0];
    let n_y = cube[i][2][1] - cube[i][0][1];
    let n_z = cube[i][2][2] - cube[i][0][2];

    let x = m_x * u + n_x * v + cube[i][0][0];
    let y = m_y * u + n_y * v + cube[i][0][1];
    let z = m_z * u + n_z * v + cube[i][0][2];

    let rotation_x = (time_stamp.cos() * x) - (time_stamp.sin() * z);
    let rotation_y = y;
    let rotation_z = (time_stamp.sin() * x) + (time_stamp.cos() * z);

    let normal_z = (cube[i][3][0] * time_stamp.sin()) + (time_stamp.cos() * cube[i][3][2]);

    let screen_x = ((rotation_x / (1.0 - rotation_z / C) + 1.0) / 2.0 * SCREEN_WIDTH as f64) as usize;
    let screen_y = ((rotation_y / (1.0 - rotation_z / C) + 1.0) / 2.0 * SCREEN_HEIGHT as f64) as usize;
    let screen_z = rotation_z / (1.0 - rotation_z / C);

    let mut l = normal_z;

    if l > 0.0 {
        if z_buffer[screen_y][screen_x] < screen_z {
            z_buffer[screen_y][screen_x] = screen_z;
            if judge_face(i, u, v) == 1 {
                let temp_u = u - (u * 3.0).floor() * (1.0 / 3.0);
                let temp_v = v - (v * 3.0).floor() * (1.0 / 3.0);
                if ((temp_u - 1.0 / 6.0) * (temp_u - 1.0 / 6.0) + (temp_v - 1.0 / 6.0) * (temp_v - 1.0 / 6.0))
                    <= 1.0 / 36.0
                {
                    l = 0.0;
                } else {
                    l = (l + 0.1) * 2.0f64.sqrt();
                }
            } else {
                l = (l + 0.1) * 2.0f64.sqrt();
            }
            let mut luminance_index = (l * 8.0) as usize;
            if luminance_index > 11 {
                luminance_index = 11;
            }
            output[screen_y][screen_x] = b".,-~:;=!*#$@"[luminance_index] as char;
            return;
        }
        return;
    } else {
        if z_buffer[screen_y][screen_x] < screen_z {
            z_buffer[screen_y][screen_x] = screen_z;
        }
        return;
    }
}

fn draw(output: &Vec<Vec<char>>, buffering_frame: &mut TimerCount) -> bool {
    buffering_frame.frames += 1;
    let time_delta = (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
        - buffering_frame.start_time)
        / 1000;

    for row in output.iter().rev() {
        println!("{}", row.iter().collect::<String>());
    }
    println!(
        "FPS: {:.2} FRAMES: {}, RUNTIME: {}",
        (buffering_frame.frames as f64 / time_delta as f64),
        (buffering_frame.frames),
        time_delta
    );
    print!("\x1B[26A"); // ANSI escape code to move cursor up 26 lines
    return true;
}

fn main() {
    render_frame();
}
