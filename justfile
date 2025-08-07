# AQIO - Norwegian Aquaculture Events Platform
# Development commands for easy workflow management

# Default recipe - show available commands
default:
    @just --list

# Setup database and run migrations
setup-db:
    @echo "🗄️  Setting up database..."
    DATABASE_URL="sqlite:./aqio.db" sqlx database create
    DATABASE_URL="sqlite:./aqio.db" sqlx migrate run --source aqio-database/migrations
    @echo "✅ Database setup complete!"

# Build all projects
build:
    @echo "🔨 Building all projects..."
    cargo build
    @echo "✅ Build complete!"

# Run the API server
api:
    @echo "🚀 Starting API server on http://127.0.0.1:3000..."
    DATABASE_URL="sqlite:./aqio.db" cargo run --bin aqio-api

# Run the frontend development server
frontend:
    @echo "🎨 Starting frontend development server on http://127.0.0.1:8080..."
    cd aqio-frontend && dx serve --port 8080

# Run both frontend and API in parallel
dev:
    @echo "🚀 Starting full development environment..."
    @echo "API: http://127.0.0.1:3000"
    @echo "Frontend: http://127.0.0.1:8080"
    just api frontend

# Stop running dev servers
stop:
    @echo "🛑 Stopping development servers..."
    @-pkill -f "aqio-api" 2>/dev/null || true
    @-pkill -f "dx serve" 2>/dev/null || true
    @-lsof -ti:3000 | xargs -r kill -9 2>/dev/null || true
    @-lsof -ti:8080 | xargs -r kill -9 2>/dev/null || true
    @echo "✅ Development servers stopped!"

# Restart dev servers
restart: stop
    @echo "🔄 Restarting development servers..."
    @sleep 1
    just dev

# Build frontend for production (WASM)
build-frontend:
    @echo "📦 Building frontend for production..."
    cd aqio-frontend && cargo build --target wasm32-unknown-unknown --release
    @echo "✅ Frontend build complete!"

# Run tests
test:
    @echo "🧪 Running tests..."
    cargo test
    @echo "✅ Tests complete!"

# Format code
fmt:
    @echo "🎨 Formatting code..."
    cargo fmt
    @echo "✅ Code formatted!"

# Run clippy lints
clippy:
    @echo "📎 Running clippy lints..."
    cargo clippy -- -D warnings
    @echo "✅ Clippy checks passed!"

# Run all checks (format, clippy, test, build)
check: fmt clippy test build
    @echo "✅ All checks passed!"

# Clean build artifacts
clean:
    @echo "🧹 Cleaning build artifacts..."
    cargo clean
    @echo "✅ Clean complete!"

# Database operations
db-reset: clean-db setup-db
    @echo "🔄 Database reset complete!"

clean-db:
    @echo "🗑️  Removing database..."
    rm -f aqio.db
    @echo "✅ Database removed!"

# Show database schema
db-schema:
    @echo "📋 Database schema:"
    sqlite3 aqio.db ".schema"

# Add some test data to the database
seed-db:
    @echo "🌱 Seeding database with test data..."
    sqlite3 aqio.db "INSERT INTO companies (id, name, org_number, location, industry_type, created_at, updated_at) VALUES ('550e8400-e29b-41d4-a716-446655440000', 'AquaNorway AS', '123456789', 'Bergen, Norway', 'Salmon', datetime('now'), datetime('now'));"
    sqlite3 aqio.db "INSERT INTO users (id, keycloak_id, email, name, company_id, created_at, updated_at) VALUES ('550e8400-e29b-41d4-a716-446655440001', 'test-user-123', 'test@aquanorway.no', 'Test User', '550e8400-e29b-41d4-a716-446655440000', datetime('now'), datetime('now'));"
    sqlite3 aqio.db "INSERT INTO events (id, title, description, event_type, start_date, end_date, location, organizer_id, max_attendees, created_at, updated_at) VALUES ('550e8400-e29b-41d4-a716-446655440002', 'Norwegian Salmon Farming Conference 2024', 'Annual conference discussing the latest in sustainable salmon farming practices', 'Conference', datetime('2024-06-15 09:00:00'), datetime('2024-06-15 17:00:00'), 'Bergen Convention Centre, Norway', '550e8400-e29b-41d4-a716-446655440001', 200, datetime('now'), datetime('now'));"
    sqlite3 aqio.db "INSERT INTO events (id, title, description, event_type, start_date, end_date, location, organizer_id, max_attendees, created_at, updated_at) VALUES ('550e8400-e29b-41d4-a716-446655440003', 'Sustainable Aquaculture Workshop', 'Hands-on workshop covering environmental best practices in aquaculture', 'Workshop', datetime('2024-07-20 10:00:00'), datetime('2024-07-20 16:00:00'), 'Trondheim Aquaculture Center', '550e8400-e29b-41d4-a716-446655440001', 50, datetime('now'), datetime('now'));"
    @echo "✅ Test data added to database!"

# Install required dependencies
install-deps:
    @echo "📥 Installing dependencies..."
    @echo "Installing SQLx CLI..."
    cargo install sqlx-cli
    @echo "Installing Dioxus CLI..."
    cargo install dioxus-cli
    @echo "Adding WASM target..."
    rustup target add wasm32-unknown-unknown
    @echo "✅ All dependencies installed!"

# Show current status
status:
    @echo "📊 AQIO Development Status"
    @echo "=========================="
    @echo "Database file: $(if test -f aqio.db; then echo '✅ Present'; else echo '❌ Missing'; fi)"
    @echo "API server: $(if lsof -i :3000 >/dev/null 2>&1; then echo '🟢 Running on port 3000'; else echo '🔴 Not running'; fi)"
    @echo "Frontend server: $(if lsof -i :8080 >/dev/null 2>&1; then echo '🟢 Running on port 8080'; else echo '🔴 Not running'; fi)"
    @echo ""
    @echo "📋 Available commands:"
    @just --list

# View logs (if running in background)
logs-api:
    @echo "📜 API logs (if running in background):"
    @echo "Use: just api > api.log 2>&1 & to run with logging"

logs-frontend:
    @echo "📜 Frontend logs (if running in background):"
    @echo "Use: just frontend > frontend.log 2>&1 & to run with logging"