package pkg

import (
	"context"
	"github.com/MarcusGoldschmidt/ptwar/pkg/ptwarloop"
	"github.com/MarcusGoldschmidt/ptwar/pkg/world"
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

	state := world.NewState(gameLoop.Logger())

	gameLoop.AddSystems(ctx, state)

	return gameLoop, nil
}
