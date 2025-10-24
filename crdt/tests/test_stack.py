import pytest
from crdt.stack import Stack


def test_push_single_value():
    stack = Stack(node="node1")
    assert stack.is_empty(), "Stack should be empty initially"
    assert len(stack) == 0, "Stack length should be 0 initially"

    stack.push("x")
    assert not stack.is_empty(), "Stack should not be empty after push"
    assert len(stack) == 1, "Stack length should be 1 after one push"

    top = stack.peek()
    assert top == "x", "Peek should return the pushed value"

    popped = stack.pop()
    assert popped == "x", "Popped value should match the pushed value"

    assert stack.is_empty(), "Stack should be empty after popping the only item"
    assert stack.pop() is None, "Popping from empty stack should return None"
    assert stack.peek() is None, "Peeking into empty stack should return None"


def test_push_multiple_values_and_pop_order():
    stack = Stack(node="n")
    stack.push(1, 2, 3)

    assert list(stack.values()) == [1, 2, 3]

    assert stack.pop() == 3
    assert stack.pop() == 2
    assert stack.pop() == 1

    assert stack.pop() is None
    assert stack.is_empty()


def test_item_ids_and_clock_increment():
    stack = Stack(node="X")
    stack.push("a", "b")

    assert len(stack) == 2
    assert stack.items[0].id.node == "X", "Node ID should match"
    assert stack.items[1].id.node == "X", "Node ID should match"

    assert stack.items[0].id.clock < stack.items[1].id.clock, "Clock should increment for each item"
    assert stack.items[0].id.clock >= 1, "Clock should start at 1"
    assert stack.items[1].id.clock >= 1, "Clock should start at 1"


def test_merge_adds_missing_items_and_sorts():

    rick = Stack[str](node="Rick")
    rick.push("Morty", "Summer")

    sanchez = Stack[str](node="Rick")
    sanchez.push("Morty", "Summer", "Beth")

    rick.merge(sanchez)

    assert rick.values() == ["Morty", "Summer", "Beth"], \
        "Merged stack should contain all unique items sorted"


def test_merge_propagates_deletion():
    rick = Stack[str](node="Rick")
    rick.push("Morty", "Summer")

    sanchez = Stack[str](node="Rick")
    sanchez.push("Morty", "Summer")
    sanchez.pop()

    # Pre-Merge

    assert len(rick) == 2
    assert rick.peek() == "Summer"

    # Post-Merge

    rick.merge(sanchez)

    assert len(rick) == 1
    assert rick.peek() == "Morty"


def test_merge_different_nodes_raises():
    s1 = Stack(node="A")
    s2 = Stack(node="B")
    with pytest.raises(ValueError):
        s1.merge(s2)