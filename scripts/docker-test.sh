#!/bin/bash
# Test TraderGrader Docker functionality

set -e

echo "🧪 Testing TraderGrader Docker setup..."

# Default values
IMAGE="tradergrader:latest"
CLEANUP=true

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --image)
            IMAGE="$2"
            shift 2
            ;;
        --no-cleanup)
            CLEANUP=false
            shift
            ;;
        --help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --image IMAGE    Docker image to test (default: tradergrader:latest)"
            echo "  --no-cleanup     Don't remove test containers after testing"
            echo "  --help           Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

echo "Testing image: $IMAGE"
echo ""

# Test 1: Health check
echo "🔍 Test 1: Health check..."
if docker run --rm --name tradergrader-test-health "$IMAGE" --health; then
    echo "✅ Health check passed"
else
    echo "❌ Health check failed"
    exit 1
fi
echo ""

# Test 2: MCP protocol basic test
echo "🔍 Test 2: MCP protocol test..."
TEST_CONTAINER="tradergrader-test-mcp"

# Create a simple MCP test
cat > /tmp/mcp_test.json << 'EOF'
{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2025-03-26", "capabilities": {}, "clientInfo": {"name": "test-client", "version": "1.0.0"}}}
{"jsonrpc": "2.0", "method": "notifications/initialized"}
{"jsonrpc": "2.0", "id": 2, "method": "tools/list"}
EOF

# Run the container with test input
if docker run --rm --name "$TEST_CONTAINER" -i "$IMAGE" < /tmp/mcp_test.json | grep -q "TraderGrader"; then
    echo "✅ MCP protocol test passed"
else
    echo "❌ MCP protocol test failed"
    rm -f /tmp/mcp_test.json
    exit 1
fi

# Cleanup test file
rm -f /tmp/mcp_test.json
echo ""

# Test 3: Container startup and shutdown
echo "🔍 Test 3: Container lifecycle test..."
LIFECYCLE_CONTAINER="tradergrader-test-lifecycle"

# Start container in background
docker run -d --name "$LIFECYCLE_CONTAINER" "$IMAGE" >/dev/null

# Wait for container to be ready with active polling
echo "⏳ Waiting for container to be ready..."
for i in {1..30}; do
    if docker logs "$LIFECYCLE_CONTAINER" 2>&1 | grep -q "TraderGrader MCP Server starting"; then
        echo "✅ Container is ready"
        break
    fi
    sleep 1
    if [ "$i" -eq 30 ]; then
        echo "❌ Container failed to become ready within timeout"
        docker rm -f "$LIFECYCLE_CONTAINER" >/dev/null 2>&1 || true
        exit 1
    fi
done

# Check if container started (it may exit quickly for MCP servers without input)
if docker ps -aq -f name="$LIFECYCLE_CONTAINER" | grep -q .; then
    echo "✅ Container started successfully"
    
    # Check logs for expected MCP server startup message
    if docker logs "$LIFECYCLE_CONTAINER" 2>&1 | grep -q "TraderGrader MCP Server starting"; then
        echo "✅ MCP server started correctly"
    else
        echo "⚠️  Unexpected log output"
    fi
    
    # Cleanup
    if [ "$CLEANUP" = true ]; then
        docker rm "$LIFECYCLE_CONTAINER" >/dev/null 2>&1 || true
        echo "✅ Container cleaned up"
    fi
else
    echo "❌ Container failed to start"
    exit 1
fi
echo ""

# Test 4: Image security check (basic)
echo "🔍 Test 4: Basic security check..."
USER_CHECK=$(docker run --rm "$IMAGE" whoami)
if [ "$USER_CHECK" = "appuser" ]; then
    echo "✅ Container runs as non-root user: $USER_CHECK"
else
    echo "⚠️  Container user: $USER_CHECK (should be 'appuser')"
fi
echo ""

echo "🎉 All Docker tests passed!"
echo ""
echo "Docker setup is ready for use!"
echo ""
echo "Quick start commands:"
echo "  ./scripts/docker-build.sh                 # Build image"
echo "  ./scripts/docker-run.sh --interactive     # Run interactively"
echo "  docker-compose up                         # Use docker-compose"