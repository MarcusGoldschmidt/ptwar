package pkg

import (
	"context"
	"errors"
	"github.com/MarcusGoldschmidt/ptwar/pkg/ptwarloop"
	"github.com/MarcusGoldschmidt/ptwar/pkg/world"
	"go.uber.org/zap"
	"runtime"
	"time"
)

type PtwarGameServer struct {
	logger   *zap.Logger
	gameLoop *ptwarloop.GameLoop
	cancel   context.CancelFunc
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
		cancel:   nil,
	}, nil
}

func (s *PtwarGameServer) Logger() *zap.Logger {
	return s.logger
}

func (s *PtwarGameServer) Start(ctx context.Context) error {
	ctx, s.cancel = context.WithCancel(ctx)

	s.logger.Info("Starting server")
	s.logger.Info("App version info", zap.Any("version", s.Version()))

	go func() {
		s.gameLoop.Loop(ctx)
	}()

	return nil
}

func (s *PtwarGameServer) Stop(ctx context.Context) error {
	if s.cancel == nil {
		return errors.New("server not started")
	}

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
