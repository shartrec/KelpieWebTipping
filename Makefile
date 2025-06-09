# Variables
BACKEND_DIR = backend
FRONTEND_DIR = frontend

# Default target
all: backend frontend

# Build the backend
backend:
	cargo build --manifest-path $(BACKEND_DIR)/Cargo.toml

# Build the frontend
frontend:
	cd $(FRONTEND_DIR) && trunk build

# Clean both backend and frontend
clean:
	cargo clean --manifest-path $(BACKEND_DIR)/Cargo.toml
	cd $(FRONTEND_DIR) && trunk clean