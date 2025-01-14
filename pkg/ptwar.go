package pkg

import (
	"context"
	"ptwar/pkg/ptwarloop"
	"runtime"
	"time"
)

func MakeDefaultServer(ctx context.Context) (*ptwarloop.GameLoop, error) {
	config := ptwarloop.GameLoopConfig{
		TicketDuration: time.Millisecond * 100,
		EventsBuffer:   1024,
		GoRoutineCount: runtime.NumCPU(),
	}
	gameLoop, err := ptwarloop.NewGameLoop(config)
	if err != nil {
		return nil, err
	}

	return gameLoop, nil
}
