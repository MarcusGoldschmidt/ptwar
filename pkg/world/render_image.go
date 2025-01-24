package world

import (
	"fmt"
	"image"
	"image/color"
	"math"
	"runtime"
	"sync"
)

func (rt *RegionTile) GenerateImage(imgSize int) (image.Image, error) {
	if imgSize <= 0 {
		return nil, fmt.Errorf("imgSize must be greater than 0")
	}

	hexSize := int(float64(imgSize) / (rt.HexLayout.Radius.X * 3.5))

	img := image.NewRGBA(
		image.Rectangle{
			Max: image.Point{
				X: imgSize,
				Y: imgSize,
			},
		},
	)

	workerChannel := make(chan *Tile, runtime.NumCPU())
	wg := sync.WaitGroup{}

	for i := 0; i < runtime.NumCPU(); i++ {
		go func() {
			for tile := range workerChannel {
				angle := math.Pi / 3 // 60 degrees in radians
				x, y := tile.Position.MapToPixel(hexSize)

				for i := 0; i < 6; i++ {
					centerX := int(x) + imgSize/2
					centerY := int(y) + imgSize/2

					x1 := int(float64(centerX) + float64(hexSize)*math.Cos(float64(i)*angle))
					y1 := int(float64(centerY) + float64(hexSize)*math.Sin(float64(i)*angle))
					x2 := int(float64(centerX) + float64(hexSize)*math.Cos(float64(i+1)*angle))
					y2 := int(float64(centerY) + float64(hexSize)*math.Sin(float64(i+1)*angle))
					drawLine(img, x1, y1, x2, y2, color.RGBA{R: 255, A: 255})
				}

				wg.Done()
			}
		}()
	}

	for _, tile := range rt.Tiles {
		wg.Add(1)
		workerChannel <- tile
	}

	wg.Wait()
	close(workerChannel)

	return img, nil
}

func drawLine(img *image.RGBA, x1, y1, x2, y2 int, col color.Color) {
	dx := int(math.Abs(float64(x2 - x1)))
	dy := int(math.Abs(float64(y2 - y1)))
	sx := -1
	if x1 < x2 {
		sx = 1
	}
	sy := -1
	if y1 < y2 {
		sy = 1
	}
	err := dx - dy

	for {
		img.Set(x1, y1, col)
		if x1 == x2 && y1 == y2 {
			break
		}
		e2 := err * 2
		if e2 > -dy {
			err -= dy
			x1 += sx
		}
		if e2 < dx {
			err += dx
			y1 += sy
		}
	}
}
