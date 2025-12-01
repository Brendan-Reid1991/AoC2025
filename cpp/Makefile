.PHONY: test build clean format

test: build
	./build/tests

build:
	cmake --build ../build

clean:
	rm -rf build/*

format:
	@echo "Formatting header_files..."
	find include/ -iname '*.h' | xargs clang-format -i
	@echo "Formatting implementation files..."
	find src/ -iname '*.cpp' | xargs clang-format -i
	@echo "Formatting tests..."
	find tests/ -iname '*.cpp' -o -iname '*.hpp' | xargs clang-format -i