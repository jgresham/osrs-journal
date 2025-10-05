#!/bin/bash

echo "🧪 Testing Dink API Server..."

# Test health endpoint
echo "📡 Testing health endpoint..."
echo "Response:"
curl -s http://localhost:3000/health | jq '.' || echo "Health check failed"

echo ""

# Test notification endpoint
echo "📨 Testing notification endpoint..."
echo "Response:"
curl -s -X POST http://localhost:3000/ \
  -F 'payload_json={"type": "DEATH", "playerName": "TestPlayer", "content": "TestPlayer has died...", "dinkAccountHash": "test_hash"}' \
  | jq '.' || echo "Notification test failed"

echo ""
echo "✅ Test complete!"
