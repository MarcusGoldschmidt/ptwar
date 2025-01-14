package world

import "go.uber.org/zap"

// State is a struct that holds the state of the application
type State struct {
	Logger *zap.Logger
}

// NewState creates a new State
func NewState(logger *zap.Logger) *State {
	return &State{
		Logger: logger,
	}
}
