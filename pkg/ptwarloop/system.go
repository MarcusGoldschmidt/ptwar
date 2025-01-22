package ptwarloop

import (
	"context"
	"github.com/MarcusGoldschmidt/ptwar/pkg/system"
)

func (gl *GameLoop) sendEvents(ctx context.Context, systems map[system.Order][]system.System) {
	for order, events := range systems {
		for _, event := range events {
			order := order
			event := event

			go func() {
				gl.mapEventOrder[order] <- event
			}()
		}
	}
}

func (gl *GameLoop) AddSetup(ctx context.Context, order system.Order, sys system.System) {
	gl.rw.Lock()
	defer gl.rw.Unlock()

	gl.setupEvents[order] = append(gl.setupEvents[order], sys)
}

func (gl *GameLoop) AddSystem(ctx context.Context, order system.Order, sys system.System) {
	gl.rw.Lock()
	defer gl.rw.Unlock()

	gl.systems[order] = append(gl.setupEvents[order], sys)
}

func (gl *GameLoop) AddSystems(ctx context.Context, getSystems system.GetSystems) {
	gl.rw.Lock()
	defer gl.rw.Unlock()

	for _, sys := range getSystems.Systems(ctx) {
		gl.systems[sys.Order] = append(gl.systems[sys.Order], sys.System)
	}
}
