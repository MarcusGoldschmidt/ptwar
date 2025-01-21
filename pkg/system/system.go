package system

import "context"

type CallBackSystem func(ctx context.Context, tick TickMessage)

func (c CallBackSystem) OnTick(ctx context.Context, tick TickMessage) {
	c(ctx, tick)
}
