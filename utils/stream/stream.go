package stream

import "sync"

type Stream[T any] struct {
	queue []T        // the stream queue
	lock  sync.Mutex // the lock
}

func NewStream[T any]() *Stream[T] {
	return &Stream[T]{
		queue: make([]T, 0),
	}
}

func (s *Stream[T]) Push(t T) {
	s.lock.Lock()
	s.queue = append(s.queue, t)
	s.lock.Unlock()
}

func (s *Stream[T]) Pop() T {
	s.lock.Lock()
	ret := s.queue[0]
	s.queue = s.queue[1:]
	s.lock.Unlock()
	return ret
}
