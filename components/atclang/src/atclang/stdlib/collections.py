# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
ATCLang Stdlib — ATC::Collections
Datenstrukturen für ATCLang Smart Contracts.
ATC-94 | Sprint 2.5
"""

from typing import Any


class ATCCollections:
    """ATC::Collections — Map, Array, Set, Queue, Stack."""

    # ── Map (Ordered Dict) ───────────────────────

    @staticmethod
    def map_new() -> dict:
        """Create empty Map. Gas: 10"""
        return {}

    @staticmethod
    def map_get(m: dict, key: Any) -> Any:
        """Get value by key. Gas: 5"""
        return m.get(key)

    @staticmethod
    def map_set(m: dict, key: Any, value: Any) -> dict:
        """Set key-value. Gas: 5"""
        m[key] = value
        return m

    @staticmethod
    def map_delete(m: dict, key: Any) -> dict:
        """Delete key. Gas: 5"""
        m.pop(key, None)
        return m

    @staticmethod
    def map_contains(m: dict, key: Any) -> bool:
        """Check key exists. Gas: 5"""
        return key in m

    @staticmethod
    def map_keys(m: dict) -> list:
        """Get all keys. Gas: 5"""
        return list(m.keys())

    @staticmethod
    def map_values(m: dict) -> list:
        """Get all values. Gas: 5"""
        return list(m.values())

    @staticmethod
    def map_size(m: dict) -> int:
        """Map size. Gas: 2"""
        return len(m)

    # ── Array (List) ─────────────────────────────

    @staticmethod
    def array_new() -> list:
        """Create empty Array. Gas: 10"""
        return []

    @staticmethod
    def array_push(arr: list, value: Any) -> list:
        """Append value. Gas: 5"""
        arr.append(value)
        return arr

    @staticmethod
    def array_pop(arr: list) -> Any:
        """Remove and return last. Gas: 5"""
        if not arr:
            return None
        return arr.pop()

    @staticmethod
    def array_get(arr: list, idx: int) -> Any:
        """Get by index. Gas: 3"""
        if 0 <= idx < len(arr):
            return arr[idx]
        return None

    @staticmethod
    def array_set(arr: list, idx: int, value: Any) -> list:
        """Set by index. Gas: 3"""
        if 0 <= idx < len(arr):
            arr[idx] = value
        return arr

    @staticmethod
    def array_len(arr: list) -> int:
        """Array length. Gas: 2"""
        return len(arr)

    @staticmethod
    def array_contains(arr: list, value: Any) -> bool:
        """Check value exists. Gas: 5"""
        return value in arr

    @staticmethod
    def array_slice(arr: list, start: int, end: int) -> list:
        """Slice array. Gas: 5"""
        return arr[start:end]

    @staticmethod
    def array_reverse(arr: list) -> list:
        """Reverse array. Gas: 10"""
        return arr[::-1]

    @staticmethod
    def array_sort(arr: list, reverse: bool = False) -> list:
        """Sort array. Gas: 20"""
        return sorted(arr, reverse=reverse)

    # ── Set ──────────────────────────────────────

    @staticmethod
    def set_new() -> set:
        """Create empty Set. Gas: 10"""
        return set()

    @staticmethod
    def set_add(s: set, value: Any) -> set:
        """Add value. Gas: 5"""
        s.add(value)
        return s

    @staticmethod
    def set_contains(s: set, value: Any) -> bool:
        """Check value exists. Gas: 5"""
        return value in s

    @staticmethod
    def set_remove(s: set, value: Any) -> set:
        """Remove value. Gas: 5"""
        s.discard(value)
        return s

    @staticmethod
    def set_size(s: set) -> int:
        """Set size. Gas: 2"""
        return len(s)

    @staticmethod
    def set_union(a: set, b: set) -> set:
        """Union. Gas: 10"""
        return a | b

    @staticmethod
    def set_intersection(a: set, b: set) -> set:
        """Intersection. Gas: 10"""
        return a & b

    @staticmethod
    def set_difference(a: set, b: set) -> set:
        """Difference. Gas: 10"""
        return a - b

    # ── Queue (FIFO) ─────────────────────────────

    @staticmethod
    def queue_new() -> list:
        """Create empty Queue. Gas: 10"""
        return []

    @staticmethod
    def queue_enqueue(q: list, value: Any) -> list:
        """Add to back. Gas: 5"""
        q.append(value)
        return q

    @staticmethod
    def queue_dequeue(q: list) -> Any:
        """Remove from front. Gas: 5"""
        if not q:
            return None
        return q.pop(0)

    @staticmethod
    def queue_peek(q: list) -> Any:
        """Peek front. Gas: 3"""
        return q[0] if q else None

    @staticmethod
    def queue_size(q: list) -> int:
        """Queue size. Gas: 2"""
        return len(q)

    # ── Stack (LIFO) ─────────────────────────────

    @staticmethod
    def stack_new() -> list:
        """Create empty Stack. Gas: 10"""
        return []

    @staticmethod
    def stack_push(s: list, value: Any) -> list:
        """Push onto stack. Gas: 5"""
        s.append(value)
        return s

    @staticmethod
    def stack_pop(s: list) -> Any:
        """Pop from stack. Gas: 5"""
        if not s:
            return None
        return s.pop()

    @staticmethod
    def stack_peek(s: list) -> Any:
        """Peek top. Gas: 3"""
        return s[-1] if s else None

    @staticmethod
    def stack_size(s: list) -> int:
        """Stack size. Gas: 2"""
        return len(s)
