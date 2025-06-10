# Variables
postgres_password := "postgres"
postgres_user := "postgres"
postgres_db := "testdb"
postgres_port := "5432"
redis_port := "6379"

# Default command when running 'just'
default:
    @just --list

# Run migrations
migrate-up:
    DATABASE_URL="postgres://{{postgres_user}}:{{postgres_password}}@localhost:{{postgres_port}}/{{postgres_db}}" sea-orm-cli migrate up

# Start PostgreSQL container
start-postgres:
    docker run --name gymtracker-postgres \
    -e POSTGRES_PASSWORD={{postgres_password}} \
    -e POSTGRES_USER={{postgres_user}} \
    -e POSTGRES_DB={{postgres_db}} \
    -p {{postgres_port}}:5432 \
    -d postgres:latest
# @just migrate-up

# Start Redis container
start-redis:
    docker run --name gymtracker-redis \
    -p {{redis_port}}:6379 \
    -d redis:latest

# Start all containers
start-all:
    @just start-postgres
    @just start-redis

# Stop PostgreSQL container
stop-postgres:
    docker stop gymtracker-postgres
    docker rm gymtracker-postgres

# Stop Redis container
stop-redis:
    docker stop gymtracker-redis
    docker rm gymtracker-redis

# Stop all containers
stop-all:
    @just stop-postgres
    @just stop-redis

# Show container status
status:
    docker ps -a | grep gymtracker
