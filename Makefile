.PHONY: run build dev install clean check release

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

# Cut a release: make release VERSION=0.2.0
release:
ifndef VERSION
	$(error VERSION is required. Usage: make release VERSION=0.2.0)
endif
	@echo "Releasing v$(VERSION)..."
	sed -i '' 's/"version": ".*"/"version": "$(VERSION)"/' package.json
	sed -i '' 's/^version = ".*"/version = "$(VERSION)"/' src-tauri/Cargo.toml
	git add package.json src-tauri/Cargo.toml
	git commit -m "release: v$(VERSION)"
	git tag "v$(VERSION)"
	git push origin main --tags
	@echo "Pushed v$(VERSION) — GitHub Actions will build the release."
