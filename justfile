# Yume dev commands (Swing 0.2)
#
# Usage: just <target>
#   just           → show help (default)
#   just dev       → start local dev stack
#   just stop      → stop dev stack
#   just test      → run all tests
#   just build     → build backend
#   just lint      → clippy + fmt check

default:
    @just --list

# ---------------------------------------------------------------------------
# Docker Compose (infrastructure services only — backend runs locally)
# ---------------------------------------------------------------------------

# Start all services (Docker: opencode+qdrant+postgres+keydb) + backend locally
dev:
    @echo "Starting all services..."
    docker compose -f docker-compose.dev.yml up -d
    @sleep 2
    @echo "OpenCode  → http://localhost:4096"
    @echo "Qdrant    → http://localhost:6334"
    @echo "Postgres  → localhost:5432"
    @echo "KeyDB     → localhost:6379"
    @echo ""
    @echo "Starting backend (port 3000)..."
    @echo "  Press Ctrl-C to stop"
    RUST_LOG=info cargo run -p yume-backend

# Start Docker services only (no backend)
infra-up:
    docker compose -f docker-compose.dev.yml up -d

# Stop dev stack and remove volumes
stop:
    docker compose -f docker-compose.dev.yml down -v

# Rebuild all Docker images and restart
rebuild:
    docker compose -f docker-compose.dev.yml up -d --build

# Build OpenCode Docker image only
build-opencode:
    docker build -f infra/opencode/Dockerfile -t yume-opencode .

# View Docker service logs
logs:
    docker compose -f docker-compose.dev.yml logs -f

# ---------------------------------------------------------------------------
# Rust / Cargo
# ---------------------------------------------------------------------------

# Run all tests across workspace
test:
    cargo test --workspace

# Run only contract tests
test-contracts:
    cargo test -p yume-contracts

# Run only backend tests
test-backend:
    cargo test -p yume-backend

# Build backend (debug)
build:
    cargo build -p yume-backend

# Build backend (release)
build-release:
    cargo build --release -p yume-backend

# Run clippy across workspace
lint:
    cargo clippy --workspace -- -D warnings

# Format check
fmt-check:
    cargo fmt --check

# Format all code
fmt:
    cargo fmt

# Full CI check (fmt + lint + test)
check:
    cargo fmt --check
    cargo clippy --workspace -- -D warnings
    cargo test --workspace

# ---------------------------------------------------------------------------
# Database (sqlx)
# ---------------------------------------------------------------------------

# Create sqlx offline data for CI builds
sqlx-prepare:
    cargo sqlx prepare --workspace --check

# Run migrations (set DATABASE_URL in .env or pass explicitly)
migrate db_url="postgres://yume:yume_dev@localhost:5432/yume":
    sqlx migrate run --database-url "{{ db_url }}"

# ---------------------------------------------------------------------------
# Utilities
# ---------------------------------------------------------------------------

# Start OpenCode container
opencode-up:
    docker compose -f docker-compose.dev.yml --profile opencode up -d opencode

# View OpenCode logs
opencode-logs:
    docker compose -f docker-compose.dev.yml logs -f opencode

# ---------------------------------------------------------------------------
# FFI / UniFFI
# ---------------------------------------------------------------------------

# Generate Kotlin bindings from UniFFI UDL → android/app/src/main/java/com/yume/rust/
ffi-bindings:
    @echo "Generating Kotlin bindings from yume-ffi..."
    cargo build -p yume-ffi
    mkdir -p android/app/src/main/java/com/yume/rust
    uniffi-bindgen generate \
        crates/yume-ffi/src/yume.udl \
        --language kotlin \
        --out-dir android/app/src/main/java/com/yume/rust/

# ---------------------------------------------------------------------------
# Utilities
# ---------------------------------------------------------------------------

# Clean all build artifacts
clean:
    cargo clean
    rm -rf android/.gradle android/app/build android/build

# ---------------------------------------------------------------------------
# Android
# ---------------------------------------------------------------------------

# Build Android debug APK
apk-debug:
    cd android && ./gradlew assembleDebug

# Build Android release APK
apk-release:
    cd android && ./gradlew assembleRelease

# Install debug APK on connected device/emulator
apk-install: apk-debug
    adb install -r android/app/build/outputs/apk/debug/app-debug.apk

# Build debug APK, install, and launch
apk-run: apk-install
    adb shell am start -n com.yume/.MainActivity

# Show connected Android devices
android-devices:
    adb devices -l

# Watch Android logs for the Yume app
android-logs:
    adb logcat -s "AndroidRuntime:E" "com.yume:*" "*:E"

# Run Android lint check
android-lint:
    cd android && ./gradlew lint
