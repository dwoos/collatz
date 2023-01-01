import subprocess
import time
import matplotlib.pyplot as plt

MAX = 10_000_000_000
MIN = 1000

def rangem(min, max):
    m = min
    while m <= max:
        yield m
        m *= 10

xs = []
ys = []
for i in rangem(MIN, MAX):
    xs.append(i)
    start = time.time()
    subprocess.run(["./collatz", str(i)])
    end = time.time()
    ys.append(end - start)

plt.scatter(xs, ys)
plt.savefig("times.png")
