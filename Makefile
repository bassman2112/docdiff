.PHONY: run build dev install clean check

# Launch the app in dev mode (hot reload)
dev:
	npm run tauri dev

# Alias for dev
run: dev

# Production build
build:
	npm run tauri build

# Install all dependencies
install:
	npm install
	cd src-tauri && cargo fetch

# Type-check Rust backend
check:
	cd src-tauri && cargo check

# Clean build artifacts
clean:
	rm -rf build .svelte-kit
	cd src-tauri && cargo clean
