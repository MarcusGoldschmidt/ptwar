package world

import (
	"context"
	"go.uber.org/zap"
	"ptwar/pkg/system"
	"sync"
	"time"
)

// State is a struct that holds the state of the application
type State struct {
	rw       sync.RWMutex
	Logger   *zap.Logger
	gameTime time.Time
}

// NewState creates a new State
func NewState(logger *zap.Logger) *State {
	return &State{
		Logger:   logger,
		rw:       sync.RWMutex{},
		gameTime: time.Now(),
	}
}

func (s *State) GameTime() time.Time {
	return s.gameTime
}

func (s *State) IncreaseTime(duration time.Duration) {
	s.rw.Lock()
	defer s.rw.Unlock()

	s.gameTime = s.gameTime.Add(duration)
}

func (s *State) Systems(ctx context.Context) []system.SystemOrder {
	response := make([]system.SystemOrder, 0)

	increaseTimeSystem := system.NewSystemOrder(
		system.First,
		system.CallBackSystem(func(ctx context.Context, tick system.TickMessage) {
			s.IncreaseTime(time.Hour)
		}),
	)

	response = append(response, increaseTimeSystem)

	return response
}
