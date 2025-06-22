#!/bin/bash
# ESI Integration Test Runner
# 
# Runs real ESI API tests to validate TraderGrader's caching and rate limiting

set -e

echo "🚀 TraderGrader ESI Integration Tests"
echo "======================================"
echo ""

# Check if we should skip ESI tests
if [ "${SKIP_ESI_TESTS}" = "1" ]; then
    echo "⏭️  Skipping ESI tests (SKIP_ESI_TESTS=1)"
    echo ""
    echo "To run ESI tests:"
    echo "  unset SKIP_ESI_TESTS"
    echo "  cargo test --test real_esi_integration_test"
    exit 0
fi

echo "📡 Testing against real EVE Online ESI API..."
echo "   This will make actual API calls and may take a moment."
echo ""

# Run ESI integration tests
echo "🧪 Running ESI integration tests..."
cargo test --test real_esi_integration_test -- --nocapture

echo ""
echo "✅ ESI integration tests completed!"
echo ""
echo "💡 Test results show:"
echo "   • Rate limiting behavior with real API"
echo "   • Cache performance improvements"  
echo "   • Error handling for edge cases"
echo "   • Data validation with live market data"
echo ""
echo "To run these tests again:"
echo "  ./scripts/test-esi.sh"
echo ""
echo "To skip ESI tests in CI/automation:"
echo "  SKIP_ESI_TESTS=1 cargo test"