package main

import (
	"fmt"
	"math"
	"time"
)

const (
	pi           = 3.14159265358979323846
	c            = 3
	screenWidth  = 50
	screenHeight = 25
)

var (
	cube = [6][4][3]float64{
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
	}

	face = [6][3][3]int{
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
	}
	BufferingFrame int64 = 0
	startTime            = time.Now().UnixMilli()
)

func judgeFace(ID int, x, y float64) int {
	return face[ID][int(3*y)][int(3*x)]
}

func ini() {
	for i := 0; i < 6; i++ {
		for j := 0; j < 4; j++ {
			x := cube[i][j][0]
			y := cube[i][j][1]
			z := cube[i][j][2]
			cube[i][j][0] = (math.Sqrt(3)/6+0.5)*x - math.Sqrt(3)/3*y + (-0.5+math.Sqrt(3)/6)*z
			cube[i][j][1] = (math.Sqrt(3)/3)*x + (math.Sqrt(3)/3)*y + (math.Sqrt(3)/3)*z
			cube[i][j][2] = (-0.5+math.Sqrt(3)/6)*x + (-math.Sqrt(3)/3)*y + (math.Sqrt(3)/6+0.5)*z
		}
	}
}

func renderFrame() [][screenHeight + 1][screenWidth + 1]byte {
	ini()
	timeStmp := 0.0
	zBuffer := make([][]float64, screenHeight+1, screenHeight+1)
	FrameCache := [][screenHeight + 1][screenWidth + 1]byte{}
	for {
		output := [screenHeight + 1][screenWidth + 1]byte{}
		timeStmp += 0.01
		for i := 0; i <= screenHeight; i++ {
			zBuffer[i] = make([]float64, screenWidth+1, screenWidth+1)
			for j := 0; j <= screenWidth; j++ {
				zBuffer[i][j] = -100
			}
		}
		for i := 0; i <= screenHeight; i++ {
			for j := 0; j <= screenWidth; j++ {
				output[i][j] = ' '
			}
		}
		for i := 0; i < 6; i++ {
			for u := 0.0; u < 1.0; u += 0.01 {
				for v := 0.0; v < 1.0; v += 0.01 {
					calFrameData(i, timeStmp, zBuffer, u, v, &output)
				}
			}
		}
		//for k, v := range FrameCache {
		//	if equal(v, output) == true {
		//		fmt.Println(k)
		//		return FrameCache[k:]
		//	}
		//}
		//FrameCache = append(FrameCache, output)
		ok := Draw(output)
		if !ok {
			return FrameCache
		}
	}
}

func calFrameData(i int, timeStmp float64, zBuffer [][]float64, u float64, v float64, output *[screenHeight + 1][screenWidth + 1]byte) {
	mX := cube[i][1][0] - cube[i][0][0]
	mY := cube[i][1][1] - cube[i][0][1]
	mZ := cube[i][1][2] - cube[i][0][2]

	nX := cube[i][2][0] - cube[i][0][0]
	nY := cube[i][2][1] - cube[i][0][1]
	nZ := cube[i][2][2] - cube[i][0][2]

	x := mX*u + nX*v + cube[i][0][0]
	y := mY*u + nY*v + cube[i][0][1]
	z := mZ*u + nZ*v + cube[i][0][2]

	rotationX := math.Cos(timeStmp)*x - math.Sin(timeStmp)*z
	rotationY := y
	rotationZ := math.Sin(timeStmp)*x + math.Cos(timeStmp)*z

	_ = cube[i][3][0]*math.Cos(timeStmp) - math.Sin(timeStmp)*cube[i][3][2]
	_ = cube[i][3][1]
	normalZ := cube[i][3][0]*math.Sin(timeStmp) + math.Cos(timeStmp)*cube[i][3][2]

	screenX := int((rotationX/(1.0-rotationZ/c) + 1) / 2 * screenWidth)
	screenY := int((rotationY/(1.0-rotationZ/c) + 1) / 2 * screenHeight)
	screenZ := rotationZ / (1.0 - rotationZ/c)
	L := normalZ
	if L > 0 {
		if zBuffer[screenY][screenX] < screenZ {
			zBuffer[screenY][screenX] = screenZ
			if judgeFace(i, u, v) == 1 {
				tempU := u - float64(int(u*3))*1/3
				tempV := v - float64(int(v*3))*1/3
				if (tempU-1.0/6)*(tempU-1.0/6)+(tempV-1.0/6)*(tempV-1.0/6) <= 1.0/36 {
					L = 0
				} else {
					L = (L + 0.1) * math.Sqrt(2)
				}
			} else {
				L = (L + 0.1) * math.Sqrt(2)
			}
			luminanceIndex := int(L * 8)
			if luminanceIndex > 11 {
				luminanceIndex = 11
			}
			(*output)[screenY][screenX] = ".,-~:;=!*#$@"[luminanceIndex]
		}
	} else {
		if zBuffer[screenY][screenX] < screenZ {
			zBuffer[screenY][screenX] = screenZ
		}
	}
}

func equal(arr1, arr2 [screenHeight + 1][screenWidth + 1]byte) bool {
	for i := range arr1 {
		for j := range arr1[i] {
			if arr1[i][j] != arr2[i][j] {
				return false
			}
		}
	}
	return true
}

func Draw(output [screenHeight + 1][screenWidth + 1]byte) bool {
	BufferingFrame++
	timeDelta := ((time.Now().UnixMilli() - startTime) / 1000)
	//if timeDelta < 60 {
	//	return true
	//}
	for j := screenHeight; j >= 0; j-- {
		fmt.Println(string(output[j][:]))
	}
	if timeDelta > 0 {
		fmt.Printf("FPS: %.2f FRAMES: %v, RUNTIME: %v \n", float64(BufferingFrame)/float64(timeDelta), BufferingFrame, timeDelta)
	}
	fmt.Printf("\033[26A")
	return true
}

func main() {
	//_ = renderFrame()
	cacheBuffer := renderFrame()
	BufferingFrame = 0
	startTime = time.Now().UnixMilli()
	for {
		for _, v := range cacheBuffer {
			Draw(v)
			//time.Sleep(time.Second / 60)
		}
	}

}
