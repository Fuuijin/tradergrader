#!/usr/bin/env python3
"""
Simple test script for TraderGrader MCP server
This sends JSON-RPC messages to test the MCP tools
"""
import json
import subprocess
import sys

def send_mcp_message(message):
    """Send a JSON-RPC message to the MCP server"""
    try:
        # Start the server process
        proc = subprocess.Popen(
            ['cargo', 'run'],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            cwd='.'
        )
        
        # Send the message
        message_json = json.dumps(message)
        stdout, stderr = proc.communicate(input=message_json + '\n', timeout=10)
        
        if stderr:
            print(f"Server stderr: {stderr}", file=sys.stderr)
        
        # Parse response
        if stdout.strip():
            return json.loads(stdout.strip().split('\n')[-1])
        return None
        
    except subprocess.TimeoutExpired:
        proc.kill()
        return {"error": "Timeout"}
    except Exception as e:
        return {"error": str(e)}

def test_initialize():
    """Test MCP initialization"""
    message = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize"
    }
    response = send_mcp_message(message)
    print("=== Initialize Test ===")
    print(json.dumps(response, indent=2))
    return response

def test_tools_list():
    """Test tools/list endpoint"""
    message = {
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list"
    }
    response = send_mcp_message(message)
    print("\n=== Tools List Test ===")
    print(json.dumps(response, indent=2))
    return response

def test_health_check():
    """Test health_check tool"""
    message = {
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": {
            "name": "health_check",
            "arguments": {}
        }
    }
    response = send_mcp_message(message)
    print("\n=== Health Check Test ===")
    print(json.dumps(response, indent=2))
    return response

def test_market_summary():
    """Test market summary for Tritanium in The Forge"""
    message = {
        "jsonrpc": "2.0",
        "id": 4,
        "method": "tools/call",
        "params": {
            "name": "get_market_summary",
            "arguments": {
                "region_id": 10000002,  # The Forge
                "type_id": 34           # Tritanium
            }
        }
    }
    response = send_mcp_message(message)
    print("\n=== Market Summary Test (Tritanium in The Forge) ===")
    print(json.dumps(response, indent=2))
    return response

if __name__ == "__main__":
    print("Testing TraderGrader MCP Server...")
    
    # Run tests
    test_initialize()
    test_tools_list()
    test_health_check()
    
    # This test makes a real API call
    print("\n" + "="*50)
    print("WARNING: The following test makes a real API call to EVE ESI")
    print("="*50)
    test_market_summary()