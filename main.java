public class Main {
    static final double pi = 3.141592653589793;
    static final int c = 3;
    static final int screen_width = 50;
    static final int screen_height = 25;

    static final double[][][] CUBE = {
            {{-0.5, -0.5, 0.5}, {0.5, -0.5, 0.5}, {-0.5, 0.5, 0.5}, {0.0, 0.0, 1.0}},
            {{-0.5, -0.5, 0.5}, {-0.5, -0.5, -0.5}, {-0.5, 0.5, 0.5}, {-1.0, 0.0, 0.0}},
            {{-0.5, -0.5, 0.5}, {-0.5, -0.5, -0.5}, {0.5, -0.5, 0.5}, {0.0, -1.0, 0.0}},
            {{-0.5, 0.5, 0.5}, {0.5, 0.5, 0.5}, {-0.5, 0.5, -0.5}, {0.0, 1.0, 0.0}},
            {{0.5, -0.5, 0.5}, {0.5, -0.5, -0.5}, {0.5, 0.5, 0.5}, {1.0, 0.0, 0.0}},
            {{-0.5, -0.5, -0.5}, {0.5, -0.5, -0.5}, {-0.5, 0.5, -0.5}, {0.0, 0.0, -1.0}}
    };

    static final int[][][] FACE = {
            {{0, 0, 0}, {0, 1, 0}, {0, 0, 0}},
            {{0, 0, 1}, {0, 0, 0}, {1, 0, 0}},
            {{0, 0, 1}, {0, 1, 0}, {1, 0, 0}},
            {{1, 0, 1}, {0, 0, 0}, {1, 0, 1}},
            {{1, 0, 1}, {0, 1, 0}, {1, 0, 1}},
            {{1, 0, 1}, {1, 0, 1}, {1, 0, 1}}
    };

    static char[][] render_frame(DrawFrame drawer) {
        transform_cube();
        var time_stamp = 0.0;
        while (true) {
            char[][] output = new char[screen_height + 1][screen_width + 1];
            for (var k=0; k<(screen_height + 1);k++){
                for (var m=0; m<(screen_width + 1);m++){
                    output[k][m] = ' ';
                }
            }

            double[][] z_buffer = new double[screen_height + 1][screen_width + 1];
            time_stamp += 0.01;
            for (int i = 0; i < 6; i++) {
                for (double u = 0; u < 1; u += 0.01) {
                    for (double v = 0; v < 1; v += 0.01) {
                        cal_frame_data(i, time_stamp, z_buffer, u, v, output);
                    }
                }
            }
            var f = drawer.count_frame(output);
            if(!f){
                return output;
            }
        }
    }

    static void cal_frame_data(int i, double time_stamp, double[][] z_buffer, double u, double v, char[][] output) {
        double m_x = CUBE[i][1][0] - CUBE[i][0][0];
        double m_y = CUBE[i][1][1] - CUBE[i][0][1];
        double m_z = CUBE[i][1][2] - CUBE[i][0][2];

        double n_x = CUBE[i][2][0] - CUBE[i][0][0];
        double n_y = CUBE[i][2][1] - CUBE[i][0][1];
        double n_z = CUBE[i][2][2] - CUBE[i][0][2];

        double x = m_x * u + n_x * v + CUBE[i][0][0];
        double y = m_y * u + n_y * v + CUBE[i][0][1];
        double z = m_z * u + n_z * v + CUBE[i][0][2];

        double rotationX = Math.cos(time_stamp) * x - Math.sin(time_stamp) * z;
        double rotationZ = Math.sin(time_stamp) * x + Math.cos(time_stamp) * z;

        double normalZ = CUBE[i][3][0] * Math.sin(time_stamp) + Math.cos(time_stamp) * CUBE[i][3][2];

        int screenX = (int) ((rotationX / (1.0 - rotationZ / c) + 1) / 2 * screen_width);
        int screenY = (int) ((y / (1.0 - rotationZ / c) + 1) / 2 * screen_height);
        double screenZ = rotationZ / (1.0 - rotationZ / c);
        double L = normalZ;

        if (L > 0) {
            if (z_buffer[screenY][screenX] < screenZ) {
                z_buffer[screenY][screenX] = screenZ;
                int judge = judge_face(i, u, v);
                if (judge == 1) {
                    double tempU = u - ((int) (u * 3)) * 1.0 / 3;
                    double tempV = v - ((int) (v * 3)) * 1.0 / 3;
                    if (Math.pow(tempU - 1.0 / 6, 2) + Math.pow(tempV - 1.0 / 6, 2) <= 1.0 / 36) {
                        L = 0;
                    } else {
                        L = (L + 0.1) * Math.sqrt(2);
                    }
                } else {
                    L = (L + 0.1) * Math.sqrt(2);
                }

                int luminance_index = (int) (L * 8);
                if (luminance_index > 11) {
                    luminance_index = 11;
                }
                String luminance_chars = ".,-~:;=!*#$@";
                output[screenY][screenX] = luminance_chars.charAt(luminance_index);
            }
        } else {
            if (z_buffer[screenY][screenX] < screenZ) {
                z_buffer[screenY][screenX] = screenZ;
            }
        }
    }

    static void transform_cube() {
        for (int i = 0; i < 6; i++) {
            for (int j = 0; j < 4; j++) {
                var x = CUBE[i][j][0];
                var y = CUBE[i][j][1];
                var z = CUBE[i][j][2];
                CUBE[i][j][0] = (Math.sqrt(3) / 6 + 0.5) * x - Math.sqrt(3) / 3 * y + (-0.5 + Math.sqrt(3) / 6) * z;
                CUBE[i][j][1] = (Math.sqrt(3) / 3) * x + (Math.sqrt(3) / 3) * y + (Math.sqrt(3) / 3) * z;
                CUBE[i][j][2] = (-0.5 + Math.sqrt(3) / 6) * x - (Math.sqrt(3) / 3) * y + (Math.sqrt(3) / 6 + 0.5) * z;
            }
        }
    }

    static int judge_face(int ID, double x, double y) {
        return FACE[ID][(int) (3 * y)][(int) (3 * x)];
    }

    static class DrawFrame {
        long start_time;
        long frames;

        DrawFrame() {
            start_time = System.currentTimeMillis();
            frames = 0;
        }

        boolean count_frame(char[][] output) {
            frames++;
            var time_delta = System.currentTimeMillis() - start_time;
            if (time_delta < 60000) {
                return true;
            }
            for (char[] line : output) {
                System.out.println(String.valueOf(line));
            }
            var fps = (double) frames / (time_delta / 1000.0);
            System.out.printf("FPS: %.2f, FRAMES: %d, RUNTIME: %.2f%n", fps, frames, time_delta / 1000.0);
            return false;
        }
    }

    public static void main(String[] args) {
        DrawFrame drawer = new DrawFrame();
        render_frame(drawer);
    }
}
