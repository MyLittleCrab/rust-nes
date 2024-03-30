import random

row = 0x20
n_rows = 24
grid_size = row * n_rows

def get_tile(r):
    if r % 4 == 0:
        return b"\1"
    if r % 41 == 1:
        return b"\2"
    return b"\0"
data = b"".join(
    get_tile(random.randint(0,100))
    for _ in range(grid_size)
)
with open("src/test_level.dat", "wb") as f:
    f.write(data)