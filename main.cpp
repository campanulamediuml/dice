#include <iostream>
#include <cmath>
#include <chrono>
#include <vector>
#include <atomic>

constexpr auto c = 3;
constexpr auto screenWidth = 50;
constexpr auto screenHeight = 25;
std::atomic<int64_t> BufferingFrame(0);
auto startTime = std::chrono::duration_cast<std::chrono::milliseconds>(
        std::chrono::system_clock::now().time_since_epoch()
).count();


double cube[6][4][3] = {
        {
                {-0.5, -0.5, 0.5}, {0.5, -0.5, 0.5}, {-0.5, 0.5, 0.5}, {0.0, 0.0, 1.0},
        },
        {
                {-0.5, -0.5, 0.5}, {-0.5, -0.5, -0.5}, {-0.5, 0.5, 0.5}, {-1.0, 0.0, 0.0},
        },
        {
                {-0.5, -0.5, 0.5}, {-0.5, -0.5, -0.5}, {0.5, -0.5, 0.5}, {0.0, -1.0, 0.0},
        },
        {
                {-0.5, 0.5, 0.5}, {0.5, 0.5, 0.5}, {-0.5, 0.5, -0.5}, {0.0, 1.0, 0.0},
        },
        {
                {0.5, -0.5, 0.5}, {0.5, -0.5, -0.5}, {0.5, 0.5, 0.5}, {1.0, 0.0, 0.0},
        },
        {
                {-0.5, -0.5, -0.5}, {0.5, -0.5, -0.5}, {-0.5, 0.5, -0.5}, {0.0, 0.0, -1.0},
        },
};

int face[6][3][3] = {
        {
                {0, 0, 0}, {0, 1, 0}, {0, 0, 0},
        },
        {
                {0, 0, 1}, {0, 0, 0}, {1, 0, 0},
        },
        {
                {0, 0, 1}, {0, 1, 0}, {1, 0, 0},
        },
        {
                {1, 0, 1}, {0, 0, 0}, {1, 0, 1},
        },
        {
                {1, 0, 1}, {0, 1, 0}, {1, 0, 1},
        },
        {
                {1, 0, 1}, {1, 0, 1}, {1, 0, 1},
        },
};

auto judgeFace(int ID, double x, double y) {
    return face[ID][static_cast<int>(3*y)][static_cast<int>(3*x)];
}

auto ini() {
    for (auto & i : cube) {
        for (auto & j : i) {
            auto x = j[0];
            auto y = j[1];
            auto z = j[2];
            j[0] = (std::sqrt(3)/6+0.5)*x - std::sqrt(3)/3*y + (-0.5+std::sqrt(3)/6)*z;
            j[1] = (std::sqrt(3)/3)*x + (std::sqrt(3)/3)*y + (std::sqrt(3)/3)*z;
            j[2] = (-0.5+std::sqrt(3)/6)*x + (-std::sqrt(3)/3)*y + (std::sqrt(3)/6+0.5)*z;
        }
    }
}

auto Draw(const std::vector<std::vector<char>>& output)  {
    auto timeDelta = double(std::chrono::duration_cast<std::chrono::milliseconds>(
            std::chrono::system_clock::now().time_since_epoch()
    ).count() - startTime) / 1000;
//    if (timeDelta < 60) {
//        return true;
//    }
    for (int j = screenHeight; j >= 0; j--) {
        for (int i = 0; i <= screenWidth; i++) {
            std::cout << output[j][i];
        }
        std::cout << std::endl;
    }
    std::cout << "FPS: " << static_cast<double>(BufferingFrame) / static_cast<double>(timeDelta) << " FRAMES: " << BufferingFrame << " RUNTIME:" << timeDelta << std::endl;
    std::cout << "\033[26A";
    return true;
}

template <typename T>
auto calFrameData(int i, T timeStmp, std::vector<std::vector<T>>& zBuffer, T u, T v, std::vector<std::vector<char>>& output) {
    auto mX = cube[i][1][0] - cube[i][0][0];
    auto mY = cube[i][1][1] - cube[i][0][1];
    auto mZ = cube[i][1][2] - cube[i][0][2];

    auto nX = cube[i][2][0] - cube[i][0][0];
    auto nY = cube[i][2][1] - cube[i][0][1];
    auto nZ = cube[i][2][2] - cube[i][0][2];

    auto x = mX*u + nX*v + cube[i][0][0];
    auto y = mY*u + nY*v + cube[i][0][1];
    auto z = mZ*u + nZ*v + cube[i][0][2];

    auto rotationX = std::cos(timeStmp)*x - std::sin(timeStmp)*z;
    auto rotationY = y;
    auto rotationZ = std::sin(timeStmp)*x + std::cos(timeStmp)*z;

    auto normalZ = cube[i][3][0]*std::sin(timeStmp) + std::cos(timeStmp)*cube[i][3][2];

    auto screenX = static_cast<int>((rotationX/(1.0-rotationZ/c) + 1) / 2 * screenWidth);
    auto screenY = static_cast<int>((rotationY/(1.0-rotationZ/c) + 1) / 2 * screenHeight);
    auto screenZ = rotationZ / (1.0 - rotationZ/c);
    auto L = normalZ;

    if (L > 0) {
        if (zBuffer[screenY][screenX] < screenZ) {
            zBuffer[screenY][screenX] = screenZ;
            if (judgeFace(i, u, v) == 1) {
                auto tempU = u - static_cast<double>(static_cast<int>(u*3))*1/3;
                auto tempV = v - static_cast<double>(static_cast<int>(v*3))*1/3;
                if ((tempU-1.0/6)*(tempU-1.0/6)+(tempV-1.0/6)*(tempV-1.0/6) <= 1.0/36) {
                    L = 0;
                } else {
                    L = (L + 0.1) * std::sqrt(2);
                }
            } else {
                L = (L + 0.1) * std::sqrt(2);
            }
            auto luminanceIndex = static_cast<int>(L * 8);
            if (luminanceIndex > 11) {
                luminanceIndex = 11;
            }
            output[screenY][screenX] = ".,-~:;=!*#$@"[luminanceIndex];
        }
    } else {
        if (zBuffer[screenY][screenX] < screenZ) {
            zBuffer[screenY][screenX] = screenZ;
        }
    }
}

[[noreturn]] auto renderFrame() {
    ini();
    auto timeStmp = 0.0;
    while (true) {
        timeStmp += 0.01;
        std::vector<std::vector<char>> output(screenHeight + 1, std::vector<char>(screenWidth + 1, ' '));
        std::vector<std::vector<double>> zBuffer(screenHeight + 1, std::vector<double>(screenWidth + 1, -100));
        for (auto i = 0; i < 6; i++) {
            for (auto u = 0; u < 100; u++) {
                for (auto v = 0; v < 100; v++) {
                    calFrameData(i, timeStmp, zBuffer, double (u)/double (100), double(v)/double(100), output);
                }
            }
        }
        BufferingFrame++;
        Draw(output);
    }
}


int main() {
    renderFrame();
}