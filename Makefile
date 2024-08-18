
install:
	pip install --upgrade pip
	pip install -e .

all:
	make c
	make cpp

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

