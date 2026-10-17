#!/usr/bin/env python3
"""Animated water-fill sequence for AoC 2018 day 17.

Reuses the verified simulator from make_day_17.py, snapshotting the grid
roughly every few hundred newly-wet cells, then renders the sequence as a
GIF: full map with live reached/retained counters.

Outputs visualisations/day_17_fill.gif. Run from the repo root:
    python3 visualisations/make_day_17_anim.py
"""

import pathlib
import sys
import time

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np
from matplotlib.animation import FuncAnimation
from matplotlib.colors import ListedColormap

sys.dont_write_bytecode = True  # keep visualisations/ free of __pycache__
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from make_day_17 import PART_ONE, PART_TWO, SPRING_X, load_clay, simulate

OUT = pathlib.Path(__file__).resolve().parent / "day_17_fill.gif"
FPS = 10
FRAME_EVERY = 300  # newly-wet cells between snapshots (~100 frames total)


def main():
    t0 = time.time()
    clay = load_clay()
    frames = []

    def on_frame(grid):
        # Deep copy: the simulator keeps mutating the live grid.
        frames.append(np.array(grid, dtype=np.uint8))

    grid, min_x, min_y, max_y = simulate(
        clay, on_frame=on_frame, frame_every=FRAME_EVERY
    )
    print(f"simulated {len(frames)} frames in {time.time() - t0:.1f}s")

    arr_last = np.array(grid, dtype=np.uint8)
    assert int(((arr_last[min_y:] == 2).sum() + (arr_last[min_y:] == 3).sum())) == PART_ONE
    assert int((arr_last[min_y:] == 3).sum()) == PART_TWO

    H, W = frames[0].shape
    cmap = ListedColormap(["#e8dcc0", "#4a3728", "#7fb3d5", "#2471a3"])

    fig, ax = plt.subplots(figsize=(5.2, 9.5))
    img = ax.imshow(frames[0], cmap=cmap, vmin=0, vmax=3, interpolation="nearest")
    ax.axhline(min_y - 0.5, c="black", ls="--", lw=0.8)
    ax.set_title("Day 17 fill — frame 0")
    ax.set_xlabel(f"x (offset {min_x})")
    ax.set_ylabel("y (depth)")
    spring = ax.scatter(
        [SPRING_X - min_x], [0], c="red", s=40, marker="+", linewidths=2, zorder=5
    )
    counter = ax.text(
        0.98, 0.02, "", transform=ax.transAxes, ha="right", va="bottom", fontsize=8,
        bbox=dict(boxstyle="round", facecolor="white", alpha=0.8),
    )
    fig.tight_layout()

    def update(k):
        f = frames[k]
        img.set_data(f)
        reached = int(((f[min_y:] == 2).sum() + (f[min_y:] == 3).sum()))
        retained = int((f[min_y:] == 3).sum())
        ax.set_title(f"Day 17 fill — frame {k + 1}/{len(frames)}")
        counter.set_text(f"reached: {reached:,}\nretained: {retained:,}")
        return img, counter

    anim = FuncAnimation(fig, update, frames=len(frames), blit=False)
    t1 = time.time()
    anim.save(OUT, writer="pillow", fps=FPS)
    print(f"wrote {OUT} ({OUT.stat().st_size / 1024:.0f} KiB) in {time.time() - t1:.1f}s")


if __name__ == "__main__":
    main()
