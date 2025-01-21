package system

import (
	"context"
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
	OnTick(ctx context.Context, tick TickMessage)
}

type GetSystems interface {
	Systems(ctx context.Context) []SystemOrder
}

type SystemOrder struct {
	Order  Order
	System System
}

func NewSystemOrder(order Order, system System) SystemOrder {
	return SystemOrder{
		Order:  order,
		System: system,
	}
}
