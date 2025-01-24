package command

import (
	"github.com/MarcusGoldschmidt/ptwar/pkg/world"
	"github.com/spf13/cobra"
	"image/jpeg"
	"log"
	"os"
)

func showMapCmd() *cobra.Command {
	hexagonRadius := 15
	imageSize := 500

	outputFile := "img.jpg"

	cmd := &cobra.Command{
		Use:   "show-map",
		Short: "Print the version number of flowtool",
		Long:  "All software has versions. This is flowtool's",
		RunE: func(cmd *cobra.Command, args []string) error {

			image, err := world.GenerateRegionTile(hexagonRadius).
				GenerateImage(imageSize)

			if err != nil {
				return err
			}

			// write image to file
			f, err := os.Create("img.jpg")
			if err != nil {
				panic(err)
			}
			defer f.Close()
			if err = jpeg.Encode(f, image, nil); err != nil {
				log.Printf("failed to encode: %v", err)
			}

			return nil
		},
	}

	cmd.Flags().IntVarP(&hexagonRadius, "hexagon-radius", "r", hexagonRadius, "Radius of the hexagons")
	cmd.Flags().IntVarP(&imageSize, "image-size", "s", imageSize, "Size of the image")
	cmd.Flags().StringVarP(&outputFile, "output-file", "o", outputFile, "Output file")

	return cmd
}
