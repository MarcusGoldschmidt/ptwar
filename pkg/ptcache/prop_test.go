package ptcache

import (
	"context"
	"testing"
)

type DummyCache struct {
	CacheFields[DummyCache]
	Age *Field[DummyCache, int]
}

func TestCache(t *testing.T) {
	d := DummyCache{
		Age: &Field[DummyCache, int]{
			Value: 0,
			Calculate: func(ctx context.Context, ref *DummyCache) (int, error) {
				return ref.Age.Value + 1, nil
			},
		},
	}

	ctx := context.Background()

	d.CalculateCache(ctx)

	if d.Age.Value != 1 {
		t.Error("Expected 1, got ", d.Age.Value)
	}
}
