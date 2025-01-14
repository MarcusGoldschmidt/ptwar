package ptwarloop

import (
	"context"
	"github.com/stretchr/testify/require"
	"testing"
	"time"
)

func TestShouldLoopThenStop(t *testing.T) {
	ctx := context.Background()

	config := GameLoopConfig{
		TicketDuration: time.Millisecond,
		EventsBuffer:   2,
		GoRoutineCount: 2,
	}
	gameLoop, err := NewGameLoop(config)
	require.NoError(t, err)

	go func() {
		gameLoop.Loop(ctx)
	}()

	time.Sleep(time.Second)

	err = gameLoop.Stop(ctx)
	require.NoError(t, err)

	require.NotZero(t, gameLoop.ticketCount, "ticketCount is 0")
}
