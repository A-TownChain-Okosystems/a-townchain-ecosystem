# Copyright (c) 2026 Michael Wroblewski / A-TownChain-Okosystems.
"""Aurora control bridge for the Genesis Engine MVP.

Protocol (one command per line, one response per line):
  STATUS
  SPAWN <x> <y>
  DESTROY <entity_id>
  SET_POSITION <entity_id> <x> <y>
  TICK <frames> <dt>
  SNAPSHOT
  RESET
  QUIT

This process owns the mutable engine world. Aurora never imports engine internals;
it controls this process through the narrow allow-listed protocol.
"""
from __future__ import annotations

import sys

from core.ecs import MovementSystem, Position, Sprite, Velocity, World


class GenesisControl:
    def __init__(self) -> None:
        self.world = World()
        self.world.add_system(MovementSystem())
        self.frame = 0

    def handle(self, line: str) -> str:
        parts = line.strip().split()
        if not parts:
            return "ERR INVALID empty-command"
        command = parts[0].upper()
        try:
            if command == "STATUS" and len(parts) == 1:
                return f"OK STATUS entities={len(self.world.entities)} frame={self.frame}"
            if command == "SPAWN" and len(parts) == 3:
                x, y = float(parts[1]), float(parts[2])
                eid = self.world.create_entity()
                self.world.add_component(eid, Position(x=x, y=y))
                self.world.add_component(eid, Sprite())
                return f"OK SPAWN id={eid}"
            if command == "DESTROY" and len(parts) == 2:
                eid = int(parts[1])
                if eid not in self.world.entities:
                    return "ERR NOT_FOUND entity"
                self.world.destroy_entity(eid)
                return f"OK DESTROY id={eid}"
            if command == "SET_POSITION" and len(parts) == 4:
                eid = int(parts[1])
                pos = self.world.get_component(eid, Position)
                if pos is None:
                    return "ERR NOT_FOUND entity"
                pos.x, pos.y = float(parts[2]), float(parts[3])
                return f"OK SET_POSITION id={eid} x={pos.x} y={pos.y}"
            if command == "TICK" and len(parts) == 3:
                frames, dt = int(parts[1]), float(parts[2])
                if frames < 1 or frames > 10_000 or dt <= 0 or dt > 10:
                    return "ERR INVALID tick-range"
                for _ in range(frames):
                    self.world.update(dt)
                    self.frame += 1
                return f"OK TICK frame={self.frame}"
            if command == "SNAPSHOT" and len(parts) == 1:
                rows = []
                for eid in sorted(self.world.entities):
                    pos = self.world.get_component(eid, Position)
                    if pos is not None:
                        rows.append(f"{eid}:{pos.x:.6f},{pos.y:.6f}")
                return "OK SNAPSHOT " + " ".join(rows)
            if command == "RESET" and len(parts) == 1:
                self.__init__()
                return "OK RESET"
            if command == "QUIT" and len(parts) == 1:
                return "OK QUIT"
        except (TypeError, ValueError, OverflowError):
            return "ERR INVALID arguments"
        return "ERR INVALID unsupported-command"


def main() -> int:
    control = GenesisControl()
    for raw in sys.stdin:
        response = control.handle(raw)
        print(response, flush=True)
        if raw.strip().upper() == "QUIT":
            return 0
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
