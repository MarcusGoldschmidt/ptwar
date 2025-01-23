package datastructures

import "github.com/MarcusGoldschmidt/ptwar/pkg/shared"

type BinaryTree[TKey shared.Comparable, TValue any] struct {
	Key   *TKey
	Value *TValue

	Left  *BinaryTree[TKey, TValue]
	Right *BinaryTree[TKey, TValue]
}

func NewBinaryTree[TKey shared.Comparable, TValue any]() *BinaryTree[TKey, TValue] {
	return &BinaryTree[TKey, TValue]{
		Key:   nil,
		Value: nil,
	}
}

func (b *BinaryTree[TKey, TValue]) Insert(key TKey, value TValue) {
	if b.Key == nil {
		b.Key = &key
		b.Value = &value
		return
	}

	if key.IsGreaterThan(*b.Key) {
		if b.Left == nil {
			b.Left = NewBinaryTree[TKey, TValue]()
		}
		b.Left.Insert(key, value)
	} else {
		if b.Right == nil {
			b.Right = NewBinaryTree[TKey, TValue]()
		}
		b.Right.Insert(key, value)
	}
}

func (b *BinaryTree[TKey, TValue]) Search(key TKey) *TValue {
	if b.Key == nil {
		return nil
	}

	if key.IsEqualTo(*b.Key) {
		return b.Value
	}

	if key.IsGreaterThan(*b.Key) {
		if b.Left == nil {
			return nil
		}
		return b.Left.Search(key)
	}

	if b.Right == nil {
		return nil
	}
	return b.Right.Search(key)
}
