.PHONY: default deps clean collatzrs

default: times.png

clean:
	rm -rf times.png
	rm -rf collatz

deps:
	pip3 install matplotlib

collatzrs:
	cd collatzrs && cargo build --release

collatz:
	gcc -O3 collatz.c -o collatz

times.png: collatz collatzrs times.py
	python3 times.py

