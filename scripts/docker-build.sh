#!/bin/bash
# Build TraderGrader Docker image

set -e

echo "🐳 Building TraderGrader Docker image..."

# Parse arguments
BUILD_TYPE="production"
TAG="tradergrader:latest"

while [[ $# -gt 0 ]]; do
    case $1 in
        --dev)
            BUILD_TYPE="development"
            TAG="tradergrader:dev"
            shift
            ;;
        --tag)
            TAG="$2"
            shift 2
            ;;
        --help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --dev      Build development image (stops at builder stage)"
            echo "  --tag TAG  Set custom image tag (default: tradergrader:latest)"
            echo "  --help     Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Build the image
if [ "$BUILD_TYPE" = "development" ]; then
    echo "Building development image (with Rust toolchain)..."
    docker build --target builder -t "$TAG" .
else
    echo "Building production image (optimized)..."
    docker build -t "$TAG" .
fi

echo "✅ Docker image built successfully: $TAG"

# Show image size
echo ""
echo "Image information:"
docker images "$TAG" --format "table {{.Repository}}\t{{.Tag}}\t{{.Size}}\t{{.CreatedAt}}"