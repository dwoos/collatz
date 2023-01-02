import subprocess
import time
import matplotlib.pyplot as plt

MAX = 10_000_000_000
MIN = 1000

BIN_C = "./collatz"
BIN_RS = "./collatzrs/target/release/collatzrs"

def rangem(min, max):
    m = min
    while m <= max:
        yield m
        m *= 10

xs = []
ys_c = []
ys_rs = []
for i in rangem(MIN, MAX):
    xs.append(i)
    start = time.time()
    subprocess.run([BIN_C, str(i)])
    end = time.time()
    ys_c.append(end - start)

    start = time.time()
    subprocess.run([BIN_RS, str(i)])
    end = time.time()
    ys_rs.append(end - start)

fig = plt.figure()
ax1 = fig.add_subplot(111)
ax1.scatter(xs, ys_c, c='b', marker="s", label="C")
ax1.scatter(xs, ys_rs, c='r', marker="o", label="Rust")
plt.legend(loc='upper left')
plt.savefig("times.png")
