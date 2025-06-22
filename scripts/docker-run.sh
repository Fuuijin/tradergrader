#!/bin/bash
# Run TraderGrader in Docker container

set -e

echo "🐳 Running TraderGrader MCP Server in Docker..."

# Default values
IMAGE="tradergrader:latest"
CONTAINER_NAME="tradergrader-mcp"
INTERACTIVE=false
HEALTH_CHECK=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --image)
            IMAGE="$2"
            shift 2
            ;;
        --name)
            CONTAINER_NAME="$2"
            shift 2
            ;;
        --interactive|-i)
            INTERACTIVE=true
            shift
            ;;
        --health)
            HEALTH_CHECK=true
            shift
            ;;
        --help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --image IMAGE    Docker image to run (default: tradergrader:latest)"
            echo "  --name NAME      Container name (default: tradergrader-mcp)"
            echo "  --interactive    Run in interactive mode with TTY"
            echo "  --health         Run health check only"
            echo "  --help           Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0                                    # Run production server"
            echo "  $0 --interactive                     # Run with interactive shell"
            echo "  $0 --health                          # Run health check"
            echo "  $0 --image tradergrader:dev --name dev-server"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Check if container is already running
if docker ps -q -f name="$CONTAINER_NAME" | grep -q .; then
    echo "⚠️  Container '$CONTAINER_NAME' is already running"
    echo "Stop it first with: docker stop $CONTAINER_NAME"
    exit 1
fi

# Remove existing stopped container
if docker ps -aq -f name="$CONTAINER_NAME" | grep -q .; then
    echo "🗑️  Removing existing container..."
    docker rm "$CONTAINER_NAME" >/dev/null
fi

# Run health check
if [ "$HEALTH_CHECK" = true ]; then
    echo "Running health check..."
    docker run --rm --name "${CONTAINER_NAME}-health" "$IMAGE" --health
    exit $?
fi

# Build docker run command
DOCKER_ARGS="--name $CONTAINER_NAME"

if [ "$INTERACTIVE" = true ]; then
    DOCKER_ARGS="$DOCKER_ARGS -it"
    echo "Running in interactive mode..."
    echo "Use Ctrl+C to stop the server"
    echo ""
else
    DOCKER_ARGS="$DOCKER_ARGS -d"
    echo "Running in daemon mode..."
fi

# Run the container
docker run $DOCKER_ARGS "$IMAGE"

if [ "$INTERACTIVE" = false ]; then
    echo "✅ Container started: $CONTAINER_NAME"
    echo ""
    echo "To view logs: docker logs -f $CONTAINER_NAME"
    echo "To stop:      docker stop $CONTAINER_NAME"
    echo "To restart:   docker restart $CONTAINER_NAME"
fi