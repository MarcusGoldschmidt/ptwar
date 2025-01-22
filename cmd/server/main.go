package main

import (
	"context"
	"github.com/MarcusGoldschmidt/ptwar/pkg"
	"go.uber.org/zap"
	"os"
	"os/signal"
	"time"
)

func main() {
	ctx := context.Background()

	server, err := pkg.MakeDefaultServer(ctx)
	if err != nil {
		return
	}

	defer server.Close(ctx)

	go func() {
		server.Loop(ctx)
	}()

	c := make(chan os.Signal, 1)
	signal.Notify(c, os.Interrupt)
	<-c

	var cancel context.CancelFunc
	ctx, cancel = context.WithTimeout(ctx, time.Second*30)
	defer cancel()

	err = server.Stop(ctx)
	if err != nil {
		server.Logger().Error("error stopping server", zap.Error(err))
		os.Exit(1)
	}

	os.Exit(0)
}
