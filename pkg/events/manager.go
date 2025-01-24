package events

import (
	"context"
	"go.uber.org/zap"
	"sync"
	"sync/atomic"
)

type HubClient interface {
	Execute(ctx context.Context, message any) error
}

type message struct {
	ctx     context.Context
	message any
}

type HubManager interface {
	SendMessage(ctx context.Context, topic string, data any)
	ShutDown(ctx context.Context)
	AddClient(ctx context.Context, topic string, client HubClient)
	RemoveClient(ctx context.Context, topic string, client HubClient)
	Stats() map[string]HubStats
}

// HubManagerImpl is a simple implementation of HubManager
// Use goroutines, should be used with care
// For game events we should use a more complex implementation
type HubManagerImpl struct {
	mutex  sync.RWMutex
	topics map[string]*Hub
	logger *zap.Logger
}

func NewHubManagerImpl(logger *zap.Logger) *HubManagerImpl {
	return &HubManagerImpl{
		mutex:  sync.RWMutex{},
		topics: map[string]*Hub{},
		logger: logger,
	}
}

// ShutDown and remove all clients
func (hm *HubManagerImpl) ShutDown(ctx context.Context) {
	hm.mutex.Lock()
	defer hm.mutex.Unlock()

	wg := sync.WaitGroup{}
	wg.Add(len(hm.topics))

	for _, hub := range hm.topics {
		go func(hub *Hub) {
			hub.Close(ctx)
			wg.Done()
		}(hub)
	}
	wg.Wait()

	hm.topics = map[string]*Hub{}
}

func (hm *HubManagerImpl) SendMessage(ctx context.Context, topic string, data any) {
	hm.mutex.RLock()
	hub, ok := hm.topics[topic]
	hm.mutex.RUnlock()

	if !ok {
		hub = hm.createTopic(ctx, topic)
	}

	go func(hub *Hub) {
		hub.Broadcast(ctx, data)
	}(hub)
}

func (hm *HubManagerImpl) AddClient(ctx context.Context, topic string, client HubClient) {
	hm.createTopic(ctx, topic).Register(ctx, client)
}

func (hm *HubManagerImpl) RemoveClient(ctx context.Context, topic string, client HubClient) {
	hm.mutex.Lock()
	defer hm.mutex.Unlock()

	if hub, ok := hm.topics[topic]; ok {
		hub.Unregister(ctx, client)
	}
}

func (hm *HubManagerImpl) Stats() map[string]HubStats {
	hm.mutex.RLock()
	defer hm.mutex.RUnlock()

	stats := make(map[string]HubStats, len(hm.topics))

	for topic, hub := range hm.topics {
		stats[topic] = hub.Stats()
	}

	return stats
}

func (hm *HubManagerImpl) createTopic(ctx context.Context, topicName string) *Hub {
	hm.mutex.Lock()
	defer hm.mutex.Unlock()

	hub, ok := hm.topics[topicName]

	if !ok {
		hub = NewHub(hm.logger)
		hm.topics[topicName] = hub
		go hm.topics[topicName].Run(ctx)
	}

	return hub
}

type HubStats struct {
	Clients           int32
	TotalMessagesSent int32
	WaitingMessages   int32
}

type Hub struct {
	lock sync.RWMutex

	// Stats
	stats HubStats

	// Registered clients.
	clients map[HubClient]bool

	// Inbound messages from the clients.
	broadcast chan message

	// Register requests from the clients.
	register chan HubClient

	// Unregister requests from clients.
	unregister chan HubClient

	logger *zap.Logger

	closed chan bool
}

func NewHub(logger *zap.Logger, bufferSize ...int) *Hub {
	size := 256

	if len(bufferSize) > 0 {
		size = bufferSize[0]
	}

	return &Hub{
		lock:       sync.RWMutex{},
		broadcast:  make(chan message, size),
		clients:    make(map[HubClient]bool),
		register:   make(chan HubClient),
		unregister: make(chan HubClient),
		closed:     make(chan bool),
		logger:     logger,
		stats:      HubStats{},
	}
}

func (h *Hub) Register(ctx context.Context, client HubClient) {
	h.register <- client
}

func (h *Hub) Unregister(ctx context.Context, client HubClient) {
	h.unregister <- client
}

func (h *Hub) Broadcast(ctx context.Context, data any) {
	h.broadcast <- message{ctx, data}
}

func (h *Hub) Stats() HubStats {
	return HubStats{
		Clients:           atomic.LoadInt32(&h.stats.Clients),
		TotalMessagesSent: atomic.LoadInt32(&h.stats.TotalMessagesSent),
		WaitingMessages:   atomic.LoadInt32(&h.stats.WaitingMessages),
	}
}

func (h *Hub) Close(context.Context) {
	h.closed <- true
}

func (h *Hub) Run(ctx context.Context) {
	for {
		select {
		case <-h.closed:
			return
		case <-ctx.Done():
			return
		case client := <-h.register:
			atomic.AddInt32(&h.stats.Clients, 1)

			h.lock.Lock()
			h.clients[client] = true
			h.lock.Unlock()
		case client := <-h.unregister:
			atomic.AddInt32(&h.stats.Clients, -1)

			h.lock.Lock()
			if _, ok := h.clients[client]; ok {
				delete(h.clients, client)
			}
			h.lock.Unlock()
		case message := <-h.broadcast:
			h.lock.RLock()
			for client := range h.clients {
				atomic.AddInt32(&h.stats.TotalMessagesSent, 1)
				atomic.AddInt32(&h.stats.WaitingMessages, 1)

				go func(client HubClient) {
					defer atomic.AddInt32(&h.stats.WaitingMessages, -1)

					err := client.Execute(message.ctx, message.message)
					if err != nil {
						h.logger.Error("Error executing event client", zap.Error(err))
					}
				}(client)
			}
			h.lock.RUnlock()
		}
	}
}
