.PHONY: default deps clean

default: times.png

clean:
	rm -rf times.png
	rm -rf collatz

deps:
	pip3 install matplotlib

collatz:
	gcc -O3 collatz.c -o collatz

times.png: collatz times.py
	python3 times.py

