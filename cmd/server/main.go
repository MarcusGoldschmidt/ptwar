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
		panic(err)
		return
	}

	defer func(server *pkg.PtwarGameServer, ctx context.Context) {
		err := server.Close(ctx)
		if err != nil {
			server.Logger().Error("error closing server", zap.Error(err))
		}
	}(server, ctx)

	err = server.Start(ctx)
	if err != nil {
		server.Logger().Error("error starting server", zap.Error(err))
		return
	}

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
