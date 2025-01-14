package main

import (
	"context"
	"ptwar/pkg"
)

func main() {
	ctx := context.Background()

	server, err := pkg.MakeDefaultServer(ctx)
	if err != nil {
		return
	}

	defer server.Close(ctx)

	server.Loop(ctx)
}
