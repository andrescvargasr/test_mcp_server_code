import argparse
import urllib.request
import json
import sys

def fetch_tools_list(url, session_id, protocol_version, req_id=1):
    tool_list_payload = {
        "jsonrpc": "2.0",
        "id": req_id,
        "method": "tools/list",
        "params": {}
    }
    req = urllib.request.Request(
        url,
        data=json.dumps(tool_list_payload).encode('utf-8'),
        headers={
            "Content-Type": "application/json",
            "Mcp-Session-Id": session_id,
            "Mcp-Protocol-Version": protocol_version
        },
        method="POST"
    )
    try:
        with urllib.request.urlopen(req) as response:
            body = response.read().decode('utf-8')
            res_json = json.loads(body)
            print("Available Tools (tools/list):")
            if "result" in res_json and "tools" in res_json["result"]:
                tools = res_json["result"]["tools"]
                if not tools:
                    print("  No tools registered on server.")
                for tool in tools:
                    name = tool.get("name", "Unknown")
                    desc = tool.get("description", "")
                    print(f"  - {name}: {desc}")
                    schema = tool.get("inputSchema")
                    if schema:
                        print(f"    Schema: {json.dumps(schema)}")
            else:
                print(json.dumps(res_json, indent=2))
    except Exception as e:
        print(f"Error fetching tools list: {e}")
        sys.exit(1)

def main():
    valid_actions = ["on", "off", "toggle", "red", "green", "blue", "list"]

    parser = argparse.ArgumentParser(
        description="Control Zephyr LED strip using MCP.",
        usage="python control_led.py [action] [mdns] [port] [--list-tools] [options]"
    )

    # Optional flags for LED controls (from control_led_2)
    parser.add_argument("--action", type=str, help="LED action (on, off, toggle, red, green, blue, list)")
    parser.add_argument("--r", type=int, help="Red channel (0-255)")
    parser.add_argument("--g", type=int, help="Green channel (0-255)")
    parser.add_argument("--b", type=int, help="Blue channel (0-255)")
    parser.add_argument("--color", type=str, help="Color string, e.g., 'rgb(255,0,0)'")
    parser.add_argument("--rainbow", action="store_true", help="Enable rainbow effect")
    parser.add_argument("--index", type=int, help="LED position (0-...) or 256 for all")

    # Connection flags
    parser.add_argument("--mdns", type=str, help="mDNS hostname prefix (default: mcp-led)")
    parser.add_argument("--port", type=str, help="Target server HTTP port (default: 8080)")

    # List tools flag (from control_led)
    parser.add_argument("--list-tools", "-l", "--list", dest="list_tools", action="store_true", help="Fetch and display available tools from server")

    # Positional arguments (from control_led)
    parser.add_argument("pos_arg1", nargs="?", default=None, help="Action or mDNS hostname")
    parser.add_argument("pos_arg2", nargs="?", default=None, help="mDNS hostname or Port")
    parser.add_argument("pos_arg3", nargs="?", default=None, help="Port")

    args = parser.parse_args()

    action = args.action
    mdns = args.mdns
    port = args.port
    list_tools_flag = args.list_tools

    pos_mdns = None
    pos_port = None

    if args.pos_arg1:
        first_arg = args.pos_arg1.lower()
        if first_arg in valid_actions:
            if not action:
                action = first_arg
            pos_mdns = args.pos_arg2
            pos_port = args.pos_arg3
        elif list_tools_flag or args.r is not None or args.g is not None or args.b is not None or args.color or args.rainbow or args.index is not None:
            pos_mdns = args.pos_arg1
            pos_port = args.pos_arg2
        else:
            print(f"Error: Invalid action '{args.pos_arg1}'. Choose from: {', '.join(valid_actions)} (or use --list-tools)")
            sys.exit(1)
    elif args.pos_arg2:
        pos_mdns = args.pos_arg2
        pos_port = args.pos_arg3

    if not mdns:
        mdns = pos_mdns if pos_mdns else "mcp-led"
    if not port:
        port = pos_port if pos_port else "8080"

    # Build the arguments dictionary for the tool call
    tool_args = {}
    if action and action != "list":
        tool_args["action"] = action
    if args.r is not None:
        tool_args["r"] = args.r
    if args.g is not None:
        tool_args["g"] = args.g
    if args.b is not None:
        tool_args["b"] = args.b
    if args.color:
        tool_args["color"] = args.color
    if args.rainbow:
        tool_args["rainbow"] = True
    if args.index is not None:
        tool_args["index"] = args.index

    if not list_tools_flag and action != "list" and not tool_args:
        parser.print_help()
        sys.exit(1)

    url = f"http://{mdns}.local:{port}/mcp"
    protocol_version = "2025-11-25"

    # 1. Initialize session
    init_payload = {
        "jsonrpc": "2.0",
        "id": 0,
        "method": "initialize",
        "params": {
            "protocolVersion": protocol_version,
            "capabilities": {},
            "clientInfo": {
                "name": "antigravity-client",
                "version": "1.0.0"
            }
        }
    }

    req = urllib.request.Request(
        url,
        data=json.dumps(init_payload).encode('utf-8'),
        headers={
            "Content-Type": "application/json",
            "Mcp-Protocol-Version": protocol_version
        },
        method="POST"
    )

    try:
        with urllib.request.urlopen(req) as response:
            body = json.loads(response.read().decode('utf-8'))
            if "result" in body and "protocolVersion" in body["result"]:
                protocol_version = body["result"]["protocolVersion"]
            headers = response.info()
            session_id = headers.get("Mcp-Session-Id")
    except Exception as e:
        print(f"Error during initialize: {e}")
        sys.exit(1)

    if not session_id:
        print("Error: No Mcp-Session-Id header returned!")
        sys.exit(1)

    print("Protocol Version:", protocol_version)

    # 2. Send initialized notification
    notif_payload = {
        "jsonrpc": "2.0",
        "method": "notifications/initialized"
    }
    req_notif = urllib.request.Request(
        url,
        data=json.dumps(notif_payload).encode('utf-8'),
        headers={
            "Content-Type": "application/json",
            "Mcp-Session-Id": session_id,
            "Mcp-Protocol-Version": protocol_version
        },
        method="POST"
    )
    try:
        with urllib.request.urlopen(req_notif) as response:
            pass
    except Exception as e:
        print(f"Warning: Error sending notification: {e}")

    req_id = 1

    # 3. Fetch tools list if action is 'list' or --list-tools flag is set
    if action == "list" or list_tools_flag:
        fetch_tools_list(url, session_id, protocol_version, req_id=req_id)
        req_id += 1

    # 4. Call tool led_control with the built arguments
    if tool_args:
        tool_payload = {
            "jsonrpc": "2.0",
            "id": req_id,
            "method": "tools/call",
            "params": {
                "name": "led_control",
                "arguments": tool_args
            }
        }
        req_tool = urllib.request.Request(
            url,
            data=json.dumps(tool_payload).encode('utf-8'),
            headers={
                "Content-Type": "application/json",
                "Mcp-Session-Id": session_id,
                "Mcp-Protocol-Version": protocol_version
            },
            method="POST"
        )

        try:
            with urllib.request.urlopen(req_tool) as response:
                body = response.read().decode('utf-8')
                res_json = json.loads(body)
                if "result" in res_json and "content" in res_json["result"]:
                    for content_item in res_json["result"]["content"]:
                        if content_item.get("type") == "text":
                            print(content_item.get("text"))
                else:
                    print("Tool call executed. Response:")
                    print(body)
        except Exception as e:
            print(f"Error calling tool: {e}")
            sys.exit(1)

if __name__ == "__main__":
    main()
