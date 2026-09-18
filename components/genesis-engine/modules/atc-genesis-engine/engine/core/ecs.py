# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""Genesis Engine — Minimal ECS (Entity-Component-System) Core."""
from __future__ import annotations

import itertools
from collections.abc import Iterator
from dataclasses import dataclass
from typing import Any


class World:
    """Zentrale Verwaltung aller Entities und Components."""

    def __init__(self) -> None:
        self._next_id = itertools.count(1)
        self.entities: set[int] = set()
        self.components: dict[type, dict[int, Any]] = {}
        self.systems: list[System] = []

    def create_entity(self) -> int:
        eid = next(self._next_id)
        self.entities.add(eid)
        return eid

    def destroy_entity(self, entity_id: int) -> None:
        self.entities.discard(entity_id)
        for store in self.components.values():
            store.pop(entity_id, None)

    def add_component(self, entity_id: int, component: Any) -> None:
        ctype = type(component)
        self.components.setdefault(ctype, {})[entity_id] = component

    def get_component(self, entity_id: int, ctype: type) -> Any | None:
        return self.components.get(ctype, {}).get(entity_id)

    def has_component(self, entity_id: int, ctype: type) -> bool:
        return entity_id in self.components.get(ctype, {})

    def query(self, *ctypes: type) -> Iterator[tuple[int, tuple[Any, ...]]]:
        """Liefert Entities, die alle angegebenen Component-Typen besitzen."""
        if not ctypes:
            return
        stores = [self.components.get(ct, {}) for ct in ctypes]
        base = stores[0]
        for eid in base:
            if all(eid in store for store in stores[1:]):
                yield eid, tuple(store[eid] for store in stores)

    def add_system(self, system: System) -> None:
        system.world = self
        self.systems.append(system)

    def update(self, dt: float) -> None:
        for system in self.systems:
            system.update(dt)


class System:
    """Basisklasse fuer Systems. Konkrete Systems ueberschreiben update()."""

    world: World | None = None

    def update(self, dt: float) -> None:
        """Default system hook; concrete systems override this method."""


@dataclass
class Position:
    x: float = 0.0
    y: float = 0.0


@dataclass
class Velocity:
    dx: float = 0.0
    dy: float = 0.0


@dataclass
class Sprite:
    color: tuple[int, int, int] = (255, 255, 255)
    width: int = 16
    height: int = 16


class MovementSystem(System):
    """Bewegt alle Entities mit Position + Velocity."""

    def update(self, dt: float) -> None:
        if self.world is None:
            return
        for _, (pos, vel) in self.world.query(Position, Velocity):
            pos.x += vel.dx * dt
            pos.y += vel.dy * dt
