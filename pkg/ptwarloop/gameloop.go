package ptwarloop

import (
	"context"
	"errors"
	"github.com/MarcusGoldschmidt/ptwar/pkg/system"
	"github.com/MarcusGoldschmidt/ptwar/pkg/world"
	"go.uber.org/zap"
	"sync"
	"sync/atomic"
	"time"
)

type GameLoopConfig struct {
	TickDuration   time.Duration
	EventsBuffer   uint
	GoRoutineCount int
}

type GameLoop struct {
	logger        *zap.Logger
	rw            sync.RWMutex
	TickCount     uint64
	TickDuration  time.Duration
	mapEventOrder map[system.Order]chan system.System
	setupEvents   map[system.Order][]system.System
	systems       map[system.Order][]system.System

	// Config
	goRoutineCount int

	// Runtime
	lastTickTime time.Time
	ticker       *time.Ticker
	cancel       context.CancelFunc
	wgWorkers    sync.WaitGroup

	// World
	state *world.State
}

func NewGameLoop(config GameLoopConfig) (*GameLoop, error) {
	if config.GoRoutineCount <= 0 {
		return nil, errors.New("invalid goRoutineCount")
	}

	if config.EventsBuffer <= 0 {
		return nil, errors.New("invalid eventsBuffer")
	}

	if config.TickDuration <= 0 {
		return nil, errors.New("invalid TickDuration")
	}

	logger, err := zap.NewProduction()
	if err != nil {
		return nil, err
	}

	gl := &GameLoop{
		logger:         logger,
		rw:             sync.RWMutex{},
		TickCount:      0,
		TickDuration:   config.TickDuration,
		mapEventOrder:  make(map[system.Order]chan system.System),
		setupEvents:    make(map[system.Order][]system.System),
		systems:        make(map[system.Order][]system.System),
		goRoutineCount: config.GoRoutineCount,
		wgWorkers:      sync.WaitGroup{},
		state:          world.NewState(logger),
		lastTickTime:   time.Now(),
	}

	for i := system.First; i <= system.Last; i++ {
		gl.mapEventOrder[i] = make(chan system.System, config.EventsBuffer)
	}

	return gl, nil
}

func (gl *GameLoop) Stop(ctx context.Context) error {
	if gl.cancel == nil {
		return errors.New("game loop is not running")
	}

	gl.cancel()

	gl.wgWorkers.Wait()
	return nil
}

func (gl *GameLoop) Loop(ctx context.Context) {
	ctx, gl.cancel = context.WithCancel(ctx)

	gl.logger.Info("Starting game loop")

	gl.ticker = time.NewTicker(gl.TickDuration)
	defer gl.ticker.Stop()

	workerChannel := make(chan func(ctx context.Context), gl.goRoutineCount)

	for i := range gl.goRoutineCount {
		gl.wgWorkers.Add(1)
		go func() {
			gl.logger.Info("worker started", zap.Int("id", i))
			defer func() {
				gl.logger.Info("worker stopped", zap.Int("id", i))
				gl.wgWorkers.Done()
			}()

			for {
				select {
				case <-ctx.Done():
					return
				case work, ok := <-workerChannel:
					if ok {
						work(ctx)
					}
				}
			}
		}()
	}

	gl.sendEvents(ctx, gl.setupEvents)

	gl.lastTickTime = time.Now()

	for {
		select {
		case <-ctx.Done():
			return
		case <-gl.ticker.C:
			gl.TickCount++

			start := time.Now()

			gl.sendEvents(ctx, gl.systems)

			loopEvents(ctx, gl, workerChannel)

			loopDuration := time.Since(start)

			if loopDuration > gl.TickDuration {
				gl.logger.Warn("tick took too long", zap.Duration("loopDuration", loopDuration))
			}

			if gl.TickCount%100 == 0 {
				gl.logger.Info(
					"Tick duration",
					zap.Uint64("Tick", gl.TickCount),
					zap.Duration("duration", loopDuration),
				)
			}

			gl.lastTickTime = time.Now()
		}
	}
}

func loopEvents(ctx context.Context, gl *GameLoop, workerChannel chan func(context.Context)) {
	runningEvents := int64(0)
	for {
		var event system.System

		select {
		case <-ctx.Done():
			return
		case event = <-gl.mapEventOrder[system.First]:
		case event = <-gl.mapEventOrder[system.Second]:
		case event = <-gl.mapEventOrder[system.Third]:
		case event = <-gl.mapEventOrder[system.Fourth]:
		case event = <-gl.mapEventOrder[system.Last]:
		default:
			if atomic.LoadInt64(&runningEvents) == 0 {
				return
			}
		}

		if event != nil {
			atomic.AddInt64(&runningEvents, 1)

			workerChannel <- func(ctx context.Context) {
				defer atomic.AddInt64(&runningEvents, -1)

				onTick := system.TickMessage{
					Delta: time.Since(gl.lastTickTime),
					Tick:  gl.TickCount,
				}

				event.OnTick(ctx, onTick)
			}
		}
	}
}

func (gl *GameLoop) Close(ctx context.Context) error {
	err := gl.logger.Sync()
	if err != nil {
		return err
	}
	return nil
}

func (gl *GameLoop) Logger() *zap.Logger {
	return gl.logger
}
