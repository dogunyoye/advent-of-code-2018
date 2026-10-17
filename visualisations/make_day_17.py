#!/usr/bin/env python3
"""Visualise AoC 2018 day 17 (Reservoir Research).

Simulates the water flow with the verified depth-first settle algorithm
(cross-checked against src/bin/day_17.rs: Part 1 29063,
Part 2 23811, and the AoC example 57/29), then plots the final state:
full-map overview, two zoomed details, and a per-row water profile.

Outputs visualisations/day_17.png. Run from the repo root:
    python3 visualisations/make_day_17.py
"""

import pathlib
import re
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np
from matplotlib.colors import ListedColormap
from matplotlib.lines import Line2D
from matplotlib.patches import Patch

ROOT = pathlib.Path(__file__).resolve().parent.parent
DATA = ROOT / "src" / "data" / "day_17_input.txt"
OUT = pathlib.Path(__file__).resolve().parent / "day_17.png"

SAND, CLAY, FLOW, SETTLED = 0, 1, 2, 3
SPRING_X = 500

# Part 1/2 answers from src/bin/day_17.rs (verified by this simulation).
PART_ONE = 29063
PART_TWO = 23811


def load_clay():
    clay = set()
    pat = re.compile(r"([xy])=([\d.]+)")  # value may be "599" or "1555..1570"
    for line in DATA.read_text().splitlines():
        line = line.strip()
        if not line:
            continue
        (a, av), (b, bv) = pat.findall(line)

        def span(s):
            return tuple(int(v) for v in s.split("..")) if ".." in s else (int(s), int(s))

        if a == "x":
            x = int(av)
            lo, hi = span(bv)
            for y in range(lo, hi + 1):
                clay.add((x, y))
        else:
            y = int(av)
            lo, hi = span(bv)
            for x in range(lo, hi + 1):
                clay.add((x, y))
    return clay


def simulate(clay, margin=10, on_frame=None, frame_every=300):
    """Depth-first flow with settle-on-contained-rows. Returns grid (bytearray
    rows), x-offset, min_y, max_y. Mutates nothing outside the grid.

    If on_frame is given, it is called as on_frame(grid) once at the start,
    roughly every frame_every newly-wet cells, and once at the end, so a
    caller can snapshot frames for an animation.
    """
    sys.setrecursionlimit(1000000)
    min_y = min(y for _, y in clay)
    max_y = max(y for _, y in clay)
    min_x = min(x for x, _ in clay) - margin
    max_x = max(x for x, _ in clay) + margin
    W = max_x - min_x + 1
    grid = [bytearray(W) for _ in range(max_y + 1)]
    for x, y in clay:
        grid[y][x - min_x] = CLAY
    visiting = set()
    overflow = False
    wet_cells = 0
    next_frame_at = 0
    if on_frame is not None:
        on_frame(grid)

    def note_frame(n=1):
        nonlocal wet_cells, next_frame_at
        if on_frame is None:
            return
        wet_cells += n
        if wet_cells >= next_frame_at:
            next_frame_at += frame_every
            on_frame(grid)

    def drip(xi, y):
        nonlocal overflow
        if xi <= 0 or xi >= W - 1:
            overflow = True
            return False
        if y > max_y:
            return False
        c = grid[y][xi]
        if c == CLAY or c == SETTLED:
            return True
        if (xi, y) in visiting:
            return False
        visiting.add((xi, y))
        try:
            grid[y][xi] = FLOW
            note_frame()
            # Settle what is below first -- but only unprocessed (SAND)
            # cells; FLOW below is final (fully processed, uncontained).
            if y + 1 <= max_y and grid[y + 1][xi] == SAND:
                drip(xi, y + 1)
            if y + 1 > max_y or grid[y + 1][xi] not in (CLAY, SETTLED):
                return False
            lx = xi
            while grid[y][lx - 1] in (SAND, FLOW) and grid[y + 1][lx - 1] in (
                CLAY,
                SETTLED,
            ):
                lx -= 1
            rx = xi
            while grid[y][rx + 1] in (SAND, FLOW) and grid[y + 1][rx + 1] in (
                CLAY,
                SETTLED,
            ):
                rx += 1
            if lx <= 0 or rx >= W - 1:
                overflow = True
                return False
            left_wall = grid[y][lx - 1] == CLAY
            right_wall = grid[y][rx + 1] == CLAY
            row = grid[y]
            for i in range(lx, rx + 1):
                if row[i] == SAND:
                    row[i] = FLOW
                    note_frame()
            if left_wall and right_wall:
                for i in range(lx, rx + 1):
                    row[i] = SETTLED
                note_frame(rx - lx + 1)
                return True
            if not left_wall:
                drip(lx - 1, y)
            if not right_wall:
                drip(rx + 1, y)
            return False
        finally:
            visiting.discard((xi, y))

    drip(SPRING_X - min_x, 0)
    assert not overflow, "water reached the simulation margin; widen it"
    if on_frame is not None:
        on_frame(grid)
    return grid, min_x, min_y, max_y


def main():
    clay = load_clay()
    grid, min_x, min_y, max_y = simulate(clay)
    arr = np.array(grid, dtype=np.uint8)  # rows 0..max_y
    H, W = arr.shape

    counted = (arr[min_y:] == FLOW).sum() + (arr[min_y:] == SETTLED).sum()
    settled = int((arr[min_y:] == SETTLED).sum())
    assert counted == PART_ONE, (counted, PART_ONE)
    assert settled == PART_TWO, (settled, PART_TWO)

    per_row_flow = (arr == FLOW).sum(axis=1)
    per_row_settled = (arr == SETTLED).sum(axis=1)

    # Zoom windows: A = densest settled 120x120 window; B = spring entry.
    win = 130
    settled_map = (arr == SETTLED).astype(np.int32)
    from numpy.lib.stride_tricks import sliding_window_view

    density = sliding_window_view(settled_map, (win, win)).sum(axis=(2, 3))
    by, bx = np.unravel_index(density.argmax(), density.shape)
    zoom_a = (bx, by, win, win)  # x0, y0, w, h in cell coords
    zoom_b = (
        max(0, (SPRING_X - min_x) - 75),
        0,
        150,
        220,
    )

    cmap = ListedColormap(["#e8dcc0", "#4a3728", "#7fb3d5", "#2471a3"])
    legend_items = [
        Patch(facecolor="#e8dcc0", edgecolor="grey", label="sand"),
        Patch(facecolor="#4a3728", edgecolor="black", label="clay"),
        Patch(facecolor="#7fb3d5", edgecolor="grey", label="flowing"),
        Patch(facecolor="#2471a3", edgecolor="black", label="settled"),
        Line2D([0], [0], color="red", marker="+", linestyle="None",
               markersize=8, markeredgewidth=2, label="spring (500, 0)"),
        Line2D([0], [0], color="black", linestyle="--", linewidth=1,
               label="counted y-band"),
    ]

    fig = plt.figure(figsize=(15, 17))
    fig.suptitle(
        "AoC 2018 Day 17 — Reservoir Research  |  "
        f"Part 1 (reached): {PART_ONE:,}  |  Part 2 (retained): {PART_TWO:,}",
        fontsize=13,
        fontweight="bold",
    )
    gs = fig.add_gridspec(
        3, 2, wspace=0.18, hspace=0.35, width_ratios=[1.15, 1], height_ratios=[1, 1, 0.85]
    )

    # ---- full map (left, spans all rows) ----
    axm = fig.add_subplot(gs[:, 0])
    axm.imshow(arr, cmap=cmap, vmin=0, vmax=3, interpolation="nearest", aspect="equal")
    axm.axhline(min_y - 0.5, c="black", ls="--", lw=1)
    axm.axhline(max_y + 0.5, c="black", ls="--", lw=1)
    axm.scatter(
        [SPRING_X - min_x], [0], c="red", s=70, marker="+", linewidths=2.5, zorder=5
    )
    for x0, y0, w, h in (zoom_a, zoom_b):
        axm.add_patch(
            plt.Rectangle(
                (x0 - 0.5, y0 - 0.5), w, h, fill=False, ec="red", lw=1.2, ls="--"
            )
        )
    axm.set_title("Full map (clay + water)")
    axm.set_xlabel(f"x (offset {min_x})")
    axm.set_ylabel("y (depth)")
    axm.legend(handles=legend_items, fontsize=8, loc="lower right")

    # ---- zooms ----
    for spec, (x0, y0, w, h), title in zip(
        (gs[0, 1], gs[1, 1]),
        (zoom_a, zoom_b),
        ("Zoom A — settled basin (densest '~' window)", "Zoom B — spring entry"),
    ):
        ax = fig.add_subplot(spec)
        ax.imshow(
            arr, cmap=cmap, vmin=0, vmax=3, interpolation="nearest", aspect="equal"
        )
        ax.set_xlim(x0 - 0.5, x0 + w - 0.5)
        ax.set_ylim(y0 + h - 0.5, y0 - 0.5)
        ax.set_title(title, fontsize=10)
        ax.set_xlabel("x (cells)")
        ax.set_ylabel("y (cells)")

    # ---- per-row profile (shares y with nothing, but same row order) ----
    axp = fig.add_subplot(gs[2, 1])
    yy = np.arange(H)
    axp.barh(yy, per_row_flow, height=1, color="#7fb3d5", label="flowing")
    axp.barh(
        yy, per_row_settled, left=per_row_flow, height=1, color="#2471a3",
        label="settled",
    )
    axp.invert_yaxis()
    axp.axhline(min_y - 0.5, c="black", ls="--", lw=1)
    axp.set_title("Water tiles per row (full height)")
    axp.set_xlabel("tiles")
    axp.set_ylabel("y (depth)")
    axp.legend(fontsize=8)

    stats = (
        f"clay cells: {len(clay):,}   |   x span: "
        f"{min(x for x, _ in clay)}..{max(x for x, _ in clay)}   |   "
        f"y span (counted): {min_y}..{max_y}   |   "
        f"spring: ({SPRING_X}, 0)   |   "
        f"retained / reached: {PART_TWO / PART_ONE:.1%}"
    )
    fig.text(
        0.5, 0.01, stats, ha="center", va="bottom", fontsize=8.5,
        bbox=dict(boxstyle="round", facecolor="wheat", alpha=0.5),
    )

    OUT.parent.mkdir(parents=True, exist_ok=True)
    fig.savefig(OUT, dpi=130, bbox_inches="tight")
    print(f"wrote {OUT} ({OUT.stat().st_size / 1024:.0f} KiB)")


if __name__ == "__main__":
    main()
