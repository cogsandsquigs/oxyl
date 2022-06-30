package queue

import "sync"

// a thread-safe queue implementation
type Queue[T any] struct {
	items []T
	lock  sync.Mutex
}

func NewQueue[T any]() *Queue[T] {
	return &Queue[T]{
		items: make([]T, 0),
		lock:  sync.Mutex{},
	}
}

func (q *Queue[T]) Push(item T) {
	q.lock.Lock()
	q.items = append(q.items, item)
	q.lock.Unlock()
}

func (q *Queue[T]) Pop() T {
	q.lock.Lock()
	item := q.items[0]
	q.items = q.items[1:]
	q.lock.Unlock()
	return item
}

func (q *Queue[T]) Peek() T {
	q.lock.Lock()
	item := q.items[0]
	q.lock.Unlock()
	return item
}

func (q *Queue[T]) Size() int {
	q.lock.Lock()
	size := len(q.items)
	q.lock.Unlock()
	return size
}
