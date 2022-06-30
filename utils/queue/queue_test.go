package queue

import "testing"

func TestQueue(t *testing.T) {
	defer func() {
		if r := recover(); r != nil {
			t.Errorf("panic: %v", r)
		}
	}()

	queue := NewQueue[int]()
	queue.Push(1)
	queue.Push(2)
	queue.Push(3)

	if queue.Size() != 3 {
		t.Errorf("queue size is not 3")
	}

	for i := 1; i <= 3; i++ {
		if queue.Peek() != i {
			t.Errorf("queue peek got %d, expected %d", queue.Peek(), i)
		}
		popped := queue.Pop()
		if popped != i {
			t.Errorf("queue pop got %d, expected %d", popped, i)
		}
	}

}
