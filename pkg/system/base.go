package system

import (
	"context"
	"ptwar/pkg/world"
	"time"
)

type Order uint8

const (
	First Order = iota
	Second
	Third
	Fourth
	Last
)

type TickMessage struct {
	Ticket uint64
	Delta  time.Duration
}

type System interface {
	OnTick(ctx context.Context, tick TickMessage, state *world.State)
}
