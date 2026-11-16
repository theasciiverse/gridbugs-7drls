"""Walks a gridbugs game's map for a preview, reading the tmux pane.

  python3 -I explore.py SOCKET TARGET X0 Y0 W H STEPS DELAY [WALLS]

Each step it finds the player ('@') in the map region (X0, Y0, W, H of
the pane; where several '@' show, the one nearest the middle) and takes
one step along the shortest path to the nearest cell that borders
unexplored space (blank) and that it has not stood on, walking through
doors and into anything else in the way (which attacks it or talks to
it). With nothing unexplored in view it heads for the farthest reachable
cell it has not stood on. WALLS are the characters it never walks into
(default '#%'). With the environment variable GOAL set to some
characters, a visible one of those is the goal instead, and it stops on
reaching it with exit status 3.

It keeps its own idea of where the player is in the world, so it also
works where the view scrolls with the player: after each step it checks
whether the player moved on the screen or the map moved under the
player. KEYLOG names a file to append each key sent to, so a walk found
once can be replayed exactly. Used by the drive.sh scripts.
"""
import os, subprocess, sys, time
from collections import deque

sock, target = sys.argv[1], sys.argv[2]
x0, y0, w, h, steps = (int(a) for a in sys.argv[3:8])
delay = float(sys.argv[8])
walls = set(sys.argv[9]) if len(sys.argv) > 9 else set('#%')
goals = set(os.environ.get('GOAL', ''))
moves = {(0, -1): 'Up', (0, 1): 'Down', (-1, 0): 'Left', (1, 0): 'Right'}
seen = set()        # world cells stood on
pos = (0, 0)        # the player's world position
far = None          # a world cell to head for when nothing is unexplored

def screen():
    out = subprocess.run(['tmux', '-L', sock, 'capture-pane', '-p', '-t', target],
                         capture_output=True, text=True).stdout.split('\n')
    return [(out[y0 + j] if y0 + j < len(out) else '').ljust(x0 + w)[x0:x0 + w] for j in range(h)]

def player(g):
    ats = [(x, y) for y, r in enumerate(g) for x, c in enumerate(r) if c == '@']
    return min(ats, key=lambda p: abs(p[0] - w // 2) + abs(p[1] - h // 2)) if ats else None

def shifted(a, b, d):
    """True when screen b looks like screen a moved by -d (the view followed the player)."""
    same = moved = 0
    for y in range(1, h - 1):
        for x in range(1, w - 1):
            if a[y][x] in ' @' or b[y][x] in ' @':
                continue
            same += a[y][x] == b[y][x]
            yy, xx = y + d[1], x + d[0]
            if 0 <= yy < h and 0 <= xx < w:
                moved += a[yy][xx] == b[y][x]
    return moved > same * 1.2 and moved > 20

g = screen()
for _ in range(steps):
    me = player(g)
    if me is None:
        time.sleep(delay)
        g = screen()
        continue
    seen.add(pos)
    world = lambda c: (c[0] - me[0] + pos[0], c[1] - me[1] + pos[1])
    def ok(x, y):
        return 0 <= x < w and 0 <= y < h and g[y][x] != ' ' and g[y][x] not in walls
    def frontier(x, y):
        return any(0 <= x + dx < w and 0 <= y + dy < h and g[y + dy][x + dx] == ' '
                   for dx, dy in moves)
    prev = {me: None}
    q = deque([me])
    goal = edge = None
    while q:
        c = q.popleft()
        if c != me:
            if goals and g[c[1]][c[0]] in goals:
                goal = c
                break
            if edge is None and frontier(*c) and world(c) not in seen:
                edge = c
                if not goals:
                    break
        for dx, dy in moves:
            n = (c[0] + dx, c[1] + dy)
            if n not in prev and ok(*n):
                prev[n] = c
                q.append(n)
    goal = goal or edge
    if goal is None:
        cells = {world(c): c for c in prev}
        if far is None or far not in cells or far in seen:
            fresh = [wc for wc in cells if wc not in seen]
            far = fresh[-1] if fresh else None
        goal = cells.get(far)
    if goal is None or goal == me:
        d = list(moves)[len(seen) % 4]
    else:
        while prev[goal] != me:
            goal = prev[goal]
        d = (goal[0] - me[0], goal[1] - me[1])
    arrive = goal is not None and g[goal[1]][goal[0]] in goals
    subprocess.run(['tmux', '-L', sock, 'send-keys', '-t', target, moves[d]])
    if os.environ.get('KEYLOG'):
        with open(os.environ['KEYLOG'], 'a') as f:
            f.write(moves[d] + '\n')
    time.sleep(delay)
    g2 = screen()
    me2 = player(g2)
    if me2 is not None and me2 != me:
        pos = (pos[0] + me2[0] - me[0], pos[1] + me2[1] - me[1])
    elif me2 is not None and shifted(g, g2, d):
        pos = (pos[0] + d[0], pos[1] + d[1])
    g = g2
    if arrive:
        sys.exit(3)
