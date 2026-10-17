#!/usr/bin/env python3
"""Visualise AoC 2018 day 23 (Experimental Emergency Teleportation).

Reads src/data/day_23_input.txt, plots bot positions (3D + 2D projections),
the strongest bot, its range slice, the Part-2 optimum point and the origin.

Outputs visualisations/day_23.png. Run from the repo root:
    python3 visualisations/make_day_23.py
"""

import pathlib
import re

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np

ROOT = pathlib.Path(__file__).resolve().parent.parent
DATA = ROOT / "src" / "data" / "day_23_input.txt"
OUT = pathlib.Path(__file__).resolve().parent / "day_23.png"

# Part-2 optimum found by the octree search (src/bin/day_23.rs):
# max in-range count, ties broken by distance from origin.
OPT_POINT = np.array([23933222, 26943113, 44664676])
OPT_COUNT = 975
OPT_DIST = int(abs(OPT_POINT).sum())  # 95541011


def load_bots():
    pos, rad = [], []
    pat = re.compile(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)")
    for line in DATA.read_text().splitlines():
        line = line.strip()
        if not line:
            continue
        x, y, z, r = map(int, pat.match(line).groups())
        pos.append((x, y, z))
        rad.append(r)
    return np.array(pos, dtype=np.int64), np.array(rad, dtype=np.int64)


def main():
    pos, rad = load_bots()
    xs, ys, zs = pos[:, 0], pos[:, 1], pos[:, 2]
    n = len(pos)
    si = int(np.argmax(rad))
    sx, sy, sz, sr = xs[si], ys[si], zs[si], rad[si]
    in_range_strongest = int(
        (np.abs(xs - sx) + np.abs(ys - sy) + np.abs(zs - sz) <= sr).sum()
    )  # Part 1: 380

    m = 1e6  # plot in millions
    fig = plt.figure(figsize=(20, 11))
    fig.suptitle(
        "AoC 2018 Day 23 — Experimental Emergency Teleportation  |  "
        f"{n} nanobots  |  strongest r={sr:,}  |  "
        f"Part 1: {in_range_strongest} in range  |  "
        f"Part 2: {OPT_COUNT} in range @ dist {OPT_DIST:,}",
        fontsize=13,
        fontweight="bold",
    )
    gs = fig.add_gridspec(2, 3, wspace=0.25, hspace=0.3, width_ratios=[0.85, 1, 1])

    # ---- 3D scatter (left, spans both rows) ----
    ax3 = fig.add_subplot(gs[:, 0], projection="3d")
    sc = ax3.scatter(
        xs / m, ys / m, zs / m, c=rad, cmap="viridis", s=8, alpha=0.45, depthshade=True
    )
    ax3.scatter([sx / m], [sy / m], [sz / m], c="red", s=160, marker="*",
                edgecolors="black", linewidths=0.8, label="strongest", depthshade=False)
    ax3.scatter([OPT_POINT[0] / m], [OPT_POINT[1] / m], [OPT_POINT[2] / m],
                c="lime", s=120, marker="D", edgecolors="black",
                linewidths=0.8, label="optimum (P2)", depthshade=False)
    ax3.scatter([0], [0], [0], c="black", s=80, marker="+",
                linewidths=2, label="origin")
    ax3.view_init(elev=18, azim=-62)
    ax3.set_xlabel("x (millions)")
    ax3.set_ylabel("y (millions)")
    ax3.set_zlabel("z (millions)")
    ax3.set_title("Bot positions (colour = range)")
    ax3.legend(loc="upper left", fontsize=9)
    fig.colorbar(sc, ax=ax3, shrink=0.6, label="range")

    # ---- 2D projections ----
    projs = [
        (gs[0, 1], xs, ys, sx, sy, "x (millions)", "y (millions)", "XY projection"),
        (gs[0, 2], xs, zs, sx, sz, "x (millions)", "z (millions)", "XZ projection"),
        (gs[1, 1], ys, zs, sy, sz, "y (millions)", "z (millions)", "YZ projection"),
    ]
    # centres bounding box (dotted) and range-expanded box (dashed)
    cx = (xs.min(), xs.max())
    cy = (ys.min(), ys.max())
    cz = (zs.min(), zs.max())
    ex = (xs - rad).min(), (xs + rad).max()
    ey = (ys - rad).min(), (ys + rad).max()
    ez = (zs - rad).min(), (zs + rad).max()
    boxes = [(cx, cy, ex, ey), (cx, cz, ex, ez), (cy, cz, ey, ez)]
    opt2d = [(OPT_POINT[0], OPT_POINT[1]), (OPT_POINT[0], OPT_POINT[2]),
             (OPT_POINT[1], OPT_POINT[2])]
    for (spec, a, b, s1, s2, xl, yl, title), ((c0, c1), (d0, d1),
           (e0, e1), (e2, e3)), (ox, oy) in zip(projs, boxes, opt2d):
        ax = fig.add_subplot(spec)
        ax.scatter(a / m, b / m, c=rad, cmap="viridis", s=6, alpha=0.45)
        # strongest range: Manhattan diamond slice through its centre
        dia_x = np.array([s1 - sr, s1, s1 + sr, s1, s1 - sr]) / m
        dia_y = np.array([s2, s2 + sr, s2, s2 - sr, s2]) / m
        ax.plot(dia_x, dia_y, c="red", ls="--", lw=1.2,
                label="strongest range (centre slice)")
        ax.add_patch(plt.Rectangle((c0 / m, d0 / m), (c1 - c0) / m, (d1 - d0) / m,
                                   fill=False, ec="grey", ls=":", lw=1,
                                   label="centres bbox"))
        ax.add_patch(plt.Rectangle((e0 / m, e2 / m), (e1 - e0) / m, (e3 - e2) / m,
                                   fill=False, ec="grey", ls="--", lw=1,
                                   label="range-expanded bbox"))
        ax.scatter([s1 / m], [s2 / m], c="red", s=120, marker="*",
                   edgecolors="black", linewidths=0.8, zorder=5)
        ax.scatter([ox / m], [oy / m], c="lime", s=80, marker="D",
                   edgecolors="black", linewidths=0.8, zorder=5,
                   label="optimum")
        ax.scatter([0], [0], c="black", s=60, marker="+", linewidths=2, zorder=5)
        ax.set_xlabel(xl)
        ax.set_ylabel(yl)
        ax.set_title(title)
        ax.set_aspect("equal", adjustable="datalim")
        ax.legend(fontsize=7, loc="best")

    # ---- radius histogram + stats ----
    axh = fig.add_subplot(gs[1, 2])
    axh.hist(rad / 1e6, bins=30, color="steelblue", edgecolor="white")
    axh.axvline(sr / 1e6, c="red", ls="--", lw=1.5, label=f"strongest ({sr / 1e6:.1f}M)")
    axh.set_xlabel("range (millions)")
    axh.set_ylabel("bots")
    axh.set_title("Distribution of ranges")
    axh.legend(fontsize=8, loc="lower right")
    dists = np.abs(pos).sum(axis=1)
    stats = (
        f"bots: {n}   |   strongest: ({sx:,}, {sy:,}, {sz:,}) r={sr:,}   |   "
        f"P1 (in range of strongest): {in_range_strongest}   |   "
        f"P2 optimum: ({OPT_POINT[0]:,}, {OPT_POINT[1]:,}, {OPT_POINT[2]:,})"
        f" with {OPT_COUNT} in range @ dist {OPT_DIST:,}   |   "
        f"median bot dist from origin: {np.median(dists):,.0f}"
    )
    fig.text(0.5, 0.01, stats, ha="center", va="bottom", fontsize=8,
             bbox=dict(boxstyle="round", facecolor="wheat", alpha=0.5))

    OUT.parent.mkdir(parents=True, exist_ok=True)
    fig.savefig(OUT, dpi=150, bbox_inches="tight")
    print(f"wrote {OUT} ({OUT.stat().st_size / 1024:.0f} KiB)")


if __name__ == "__main__":
    main()
