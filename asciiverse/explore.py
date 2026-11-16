"""Walks a gridbugs game's map for a preview, reading the tmux pane.

  python3 -I explore.py SOCKET TARGET X0 Y0 W H STEPS DELAY [WALLS]

Each step it finds the player ('@') in the map region (X0, Y0, W, H of
the pane), then takes one step along the shortest path to the nearest
known cell that borders unexplored space (blank), walking through doors
and into anything else in the way (which attacks it). WALLS are the
characters it never walks into (default '#%'). Used by drive.sh scripts.
"""
import subprocess, sys, time
from collections import deque

sock, target = sys.argv[1], sys.argv[2]
x0, y0, w, h, steps = (int(a) for a in sys.argv[3:8])
delay = float(sys.argv[8])
walls = set(sys.argv[9]) if len(sys.argv) > 9 else set('#%')
moves = {(0, -1): 'Up', (0, 1): 'Down', (-1, 0): 'Left', (1, 0): 'Right'}
visits = {}

def screen():
    out = subprocess.run(['tmux', '-L', sock, 'capture-pane', '-p', '-t', target],
                         capture_output=True, text=True).stdout.split('\n')
    rows = [(out[y0 + j] if y0 + j < len(out) else '').ljust(x0 + w)[x0:x0 + w] for j in range(h)]
    return rows

for _ in range(steps):
    g = screen()
    me = next(((x, y) for y, r in enumerate(g) for x, c in enumerate(r) if c == '@'), None)
    if me is None:
        time.sleep(delay)
        continue
    visits[me] = visits.get(me, 0) + 1
    def ok(x, y):
        return 0 <= x < w and 0 <= y < h and g[y][x] != ' ' and g[y][x] not in walls
    def frontier(x, y):
        return any(0 <= x + dx < w and 0 <= y + dy < h and g[y + dy][x + dx] == ' '
                   for dx, dy in moves)
    prev = {me: None}
    q = deque([me])
    goal = None
    while q:
        c = q.popleft()
        if c != me and frontier(*c) and visits.get(c, 0) < 2:
            goal = c
            break
        for dx, dy in moves:
            n = (c[0] + dx, c[1] + dy)
            if n not in prev and ok(*n):
                prev[n] = c
                q.append(n)
    if goal is None:
        step = list(moves.values())[sum(visits.values()) % 4]
    else:
        while prev[goal] != me:
            goal = prev[goal]
        step = moves[(goal[0] - me[0], goal[1] - me[1])]
        visits[goal] = visits.get(goal, 0) + 1
    subprocess.run(['tmux', '-L', sock, 'send-keys', '-t', target, step])
    time.sleep(delay)
