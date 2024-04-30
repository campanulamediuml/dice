import math
import time

pi = 3.141592653589793
c = 3
screen_width = 50
screen_height = 25

CUBE = [
    [[-0.5, -0.5, 0.5], [0.5, -0.5, 0.5], [-0.5, 0.5, 0.5], [0.0, 0.0, 1.0]],
    [[-0.5, -0.5, 0.5], [-0.5, -0.5, -0.5], [-0.5, 0.5, 0.5], [-1.0, 0.0, 0.0]],
    [[-0.5, -0.5, 0.5], [-0.5, -0.5, -0.5], [0.5, -0.5, 0.5], [0.0, -1.0, 0.0]],
    [[-0.5, 0.5, 0.5], [0.5, 0.5, 0.5], [-0.5, 0.5, -0.5], [0.0, 1.0, 0.0]],
    [[0.5, -0.5, 0.5], [0.5, -0.5, -0.5], [0.5, 0.5, 0.5], [1.0, 0.0, 0.0]],
    [[-0.5, -0.5, -0.5], [0.5, -0.5, -0.5], [-0.5, 0.5, -0.5], [0.0, 0.0, -1.0]],
]

FACE = [
    [[0, 0, 0], [0, 1, 0], [0, 0, 0]],
    [[0, 0, 1], [0, 0, 0], [1, 0, 0]],
    [[0, 0, 1], [0, 1, 0], [1, 0, 0]],
    [[1, 0, 1], [0, 0, 0], [1, 0, 1]],
    [[1, 0, 1], [0, 1, 0], [1, 0, 1]],
    [[1, 0, 1], [1, 0, 1], [1, 0, 1]],
]


def judge_face(ID, x, y):
    return FACE[ID][int(3 * y)][int(3 * x)]


def transform_cube():
    for i in range(6):
        for j in range(4):
            x, y, z = CUBE[i][j]
            CUBE[i][j][0] = (math.sqrt(3) / 6 + 0.5) * x - math.sqrt(3) / 3 * y + (-0.5 + math.sqrt(3) / 6) * z
            CUBE[i][j][1] = (math.sqrt(3) / 3) * x + (math.sqrt(3) / 3) * y + (math.sqrt(3) / 3) * z
            CUBE[i][j][2] = (-0.5 + math.sqrt(3) / 6) * x - (math.sqrt(3) / 3) * y + (math.sqrt(3) / 6 + 0.5) * z


def render_frame(drawer):
    transform_cube()
    time_stamp = 0.0
    while True:
        output = [[' ' for _ in range(screen_width + 1)] for _ in range(screen_height + 1)]
        z_buffer = [[-100 for _ in range(screen_width + 1)] for _ in range(screen_height + 1)]
        time_stamp += 0.01

        for i in range(6):
            for u in [u * 0.01 for u in range(100)]:
                for v in [v * 0.01 for v in range(100)]:
                    cal_frame_data(i, time_stamp, z_buffer, u, v, output)

        r = drawer.count_frame(output)
        if r == False:
            return# Adjust based on desired FPS


def cal_frame_data(i, time_stamp, z_buffer, u, v, output):
    m_x = CUBE[i][1][0] - CUBE[i][0][0]
    m_y = CUBE[i][1][1] - CUBE[i][0][1]
    m_z = CUBE[i][1][2] - CUBE[i][0][2]

    n_x = CUBE[i][2][0] - CUBE[i][0][0]
    n_y = CUBE[i][2][1] - CUBE[i][0][1]
    n_z = CUBE[i][2][2] - CUBE[i][0][2]

    x = m_x * u + n_x * v + CUBE[i][0][0]
    y = m_y * u + n_y * v + CUBE[i][0][1]
    z = m_z * u + n_z * v + CUBE[i][0][2]

    rotationX = math.cos(time_stamp) * x - math.sin(time_stamp) * z
    rotationY = y
    rotationZ = math.sin(time_stamp) * x + math.cos(time_stamp) * z

    normalZ = CUBE[i][3][0] * math.sin(time_stamp) + math.cos(time_stamp) * CUBE[i][3][2]

    screenX = int((rotationX / (1.0 - rotationZ / c) + 1) / 2 * screen_width)
    screenY = int((rotationY / (1.0 - rotationZ / c) + 1) / 2 * screen_height)
    screenZ = rotationZ / (1.0 - rotationZ / c)
    L = normalZ

    if L > 0:
        if z_buffer[screenY][screenX] < screenZ:
            z_buffer[screenY][screenX] = screenZ
            judge = judge_face(i, u, v)
            if judge == 1:
                tempU = u - float(int(u * 3)) * 1 / 3
                tempV = v - float(int(v * 3)) * 1 / 3
                if (tempU - 1.0 / 6) ** 2 + (tempV - 1.0 / 6) ** 2 <= 1.0 / 36:
                    L = 0
                else:
                    L = (L + 0.1) * math.sqrt(2)
            else:
                L = (L + 0.1) * math.sqrt(2)

            luminance_index = int(L * 8)
            if luminance_index > 11:
                luminance_index = 11
            luminance_chars = ".,-~:;=!*#$@"
            output[screenY][screenX] = luminance_chars[luminance_index]
    else:
        if z_buffer[screenY][screenX] < screenZ:
            z_buffer[screenY][screenX] = screenZ


# The calculation logic goes here, following the Go implementation.
# This is a placeholder for the detailed implementation.
# Note: This method involves significant mathematical calculations
#       related to 3D transformations and rotations.

class DrawFrame:
    def __init__(self):
        self.start_time = time.time()
        self.frames = 0

    def count_frame(self, output):
        self.frames += 1
        time_delta = time.time() - self.start_time
        if time_delta < 60:
            return True
        # print(f"\033[2J\033[H", end="")  # Clear the screen and reset cursor position.
        for line in reversed(output):
            print("".join(line))
        if time_delta >= 1:
            print("FPS: %.2f, FRAMES: %s, RUNTIME: %s" % (self.frames / time_delta, self.frames, time_delta))
        return False


if __name__ == "__main__":
    drawer = DrawFrame()
    target = render_frame(drawer)
