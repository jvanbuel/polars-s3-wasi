# ---- Builder Stage ----
FROM ghcr.io/astral-sh/uv:latest AS uv

FROM python:3.14-slim AS builder

WORKDIR /app

# Enable bytecode compilation for dependencies
ENV UV_COMPILE_BYTECODE=1
ENV UV_LINK_MODE=copy

# Copy uv binary from the official image
COPY --from=uv /uv /uvx /bin/

# Copy dependency files
COPY pyproject.toml uv.lock ./

# Install dependencies into /app/.venv
# --frozen ensures we stick exactly to uv.lock
# --no-install-project installs only dependencies, not the app itself
RUN uv sync --frozen --no-install-project --no-dev

# Copy application code
COPY main.py .

# Explicitly compile the application code
RUN python -m compileall .

# ---- Final Stage ----
FROM python:3.14-slim AS final

WORKDIR /app

# Copy the virtual environment (pre-compiled)
COPY --from=builder /app/.venv /app/.venv

# Copy the application and its compiled bytecode
COPY --from=builder /app/main.py .
COPY --from=builder /app/__pycache__ ./__pycache__

# Add virtual environment to PATH
ENV PATH="/app/.venv/bin:$PATH"

# Run the application
CMD ["python", "main.py"]