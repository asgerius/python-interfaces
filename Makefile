
install:
	pip install --upgrade pip
	pip install -e .

all:
	make clean
	make c
	make cpp
	make rust

c:
	mkdir -p build
	gcc -o build/libc.so -fPIC -O3 -shared python_interfaces/libc.c

cpp:
	mkdir -p build
	gcc -o build/libcpp.so -fPIC -O3 -shared python_interfaces/libcpp.cpp

rust:
	mkdir -p build
	cd librust && cargo build --release
	cp librust/target/release/liblibrust.so build/librust.so

clean:
	rm -rf build
	rm -rf librust/target
