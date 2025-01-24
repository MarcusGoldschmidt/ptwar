package pkg

import (
	"context"
	"github.com/MarcusGoldschmidt/ptwar/pkg/ptwarloop"
	"github.com/MarcusGoldschmidt/ptwar/pkg/world"
	"go.uber.org/zap"
	"runtime"
	"time"
)

type PtwarGameServer struct {
	logger   *zap.Logger
	gameLoop *ptwarloop.GameLoop
}

func MakeDefaultServer(ctx context.Context) (*PtwarGameServer, error) {
	config := ptwarloop.GameLoopConfig{
		TickDuration:   time.Millisecond * 100,
		EventsBuffer:   1024,
		GoRoutineCount: runtime.NumCPU(),
	}
	gameLoop, err := ptwarloop.NewGameLoop(config)
	if err != nil {
		return nil, err
	}

	state := world.NewState(gameLoop.Logger())

	gameLoop.AddSystems(ctx, state)

	return &PtwarGameServer{
		logger:   gameLoop.Logger(),
		gameLoop: gameLoop,
	}, nil
}

func (s *PtwarGameServer) Logger() *zap.Logger {
	return s.logger
}

func (s *PtwarGameServer) Start(ctx context.Context) error {

	s.logger.Info("Starting server")
	s.logger.Info("App version info", zap.Any("version", s.Version()))

	go func() {
		s.gameLoop.Loop(ctx)
	}()

	return nil
}

func (s *PtwarGameServer) Stop(ctx context.Context) error {
	err := s.gameLoop.Stop(ctx)
	if err != nil {
		return err
	}

	return nil
}

func (s *PtwarGameServer) Close(ctx context.Context) error {
	err := s.gameLoop.Close(ctx)
	if err != nil {
		return err
	}

	return nil
}
