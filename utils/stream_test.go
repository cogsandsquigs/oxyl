package utils

import "testing"

func TestStream(t *testing.T) {
	s := NewStream[int]()

	if len(s.queue) != 0 {
		t.Errorf("Expected 0, got %d", len(s.queue))
	}

	s.Push(1)
	s.Push(2)
	s.Push(3)

	if len(s.queue) != 3 {
		t.Errorf("Expected 3, got %d", len(s.queue))
	}

	if s.Pop() != 1 {
		t.Error("Expected 1")
	}
	if s.Pop() != 2 {
		t.Error("Expected 2")
	}
	if s.Pop() != 3 {
		t.Error("Expected 3")
	}

	if len(s.queue) != 0 {
		t.Errorf("Expected 0, got %d", len(s.queue))
	}
}
