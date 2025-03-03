#!/bin/bash

echo "Database is ready!"

echo "Creating database..."

echo "ENV VARIABLES $DATABASE_URL"

sqlx database create

echo "Running migrations..."
sqlx migrate run --source /social-net/libs/sqlx/migrations

echo "Database initialization complete."