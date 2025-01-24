package main

import (
	"context"
	"fmt"
	"github.com/MarcusGoldschmidt/ptwar/cmd/tool/command"
	"os"
)

func main() {
	ctx := context.Background()

	err := command.Execute(ctx)
	if err != nil {
		fmt.Println(err.Error())
		os.Exit(1)
	}
	os.Exit(0)
}
