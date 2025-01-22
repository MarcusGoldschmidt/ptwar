package ptcache

import (
	"context"
	"reflect"
	"strings"
	"sync"
	"unsafe"
)

type CalculateFieldFunc[TRef any, TValue any] func(context.Context, *TRef) (TValue, error)

type Field[TRef any, TValue any] struct {
	Value     TValue
	Calculate CalculateFieldFunc[TRef, TValue]
}

func (c *Field[TRef, TValue]) CalculateValue(ctx context.Context, ref *TRef) error {
	value, err := c.Calculate(ctx, ref)
	if err != nil {
		return err
	}
	c.Value = value
	return nil
}

type GameCacher interface {
	CalculateCache(ctx context.Context)
}
type CacheFields[TRef any] struct{}

var mapCacheProps = make(map[string][]int)
var mapCacheMutex = sync.RWMutex{}

func (c *CacheFields[TRef]) CalculateCache(ctx context.Context) {
	// cast C into TRef
	ref := (*TRef)(unsafe.Pointer(c))

	t := reflect.TypeOf(ref)
	v := reflect.ValueOf(ref)

	if t.Kind() == reflect.Ptr {
		t = t.Elem()
		v = v.Elem()
	}

	if t.Kind() != reflect.Struct {
		return
	}

	var index []int

	mapCacheMutex.RLock()
	if newIndex, ok := mapCacheProps[t.Name()]; ok {
		index = newIndex
		mapCacheMutex.RUnlock()
	} else {
		mapCacheMutex.RUnlock()

		for i := 0; i < t.NumField(); i++ {
			field := t.Field(i)
			if field.Type.Kind() == reflect.Ptr && strings.HasPrefix(field.Type.Elem().Name(), "Field") {
				index = append(index, i)
			}
		}

		mapCacheMutex.Lock()
		mapCacheProps[t.Name()] = index
		mapCacheMutex.Unlock()
	}

	for _, i := range index {
		callValues := []reflect.Value{reflect.ValueOf(ctx), reflect.ValueOf(ref)}
		cacheFieldValue := v.Field(i)
		cacheFieldValue.MethodByName("CalculateValue").Call(callValues)
	}
}
