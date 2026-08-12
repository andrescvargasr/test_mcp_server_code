use serde_json::json;
use std::env;
use std::process;

fn print_help() {
    println!("Usage: control_led [action] [mdns] [port] [options]");
    println!("\nActions:");
    println!("  on, off, toggle, red, green, blue, list");
    println!("\nOptions:");
    println!("  --action <action>         LED action (on, off, toggle, red, green, blue, list)");
    println!("  --mdns <hostname>         mDNS hostname prefix (default: mcp-led)");
    println!("  --port <port>             Target server HTTP port (default: 8080)");
    println!("  --list-tools, -l, --list  Fetch and display available tools from server");
    println!("  --r <val>                 Red channel (0-255)");
    println!("  --g <val>                 Green channel (0-255)");
    println!("  --b <val>                 Blue channel (0-255)");
    println!("  --color <str>             Color string, e.g. 'rgb(255,0,0)'");
    println!("  --rainbow                 Enable rainbow effect");
    println!("  --index <val>             LED position (0-...) or 256 for all");
    println!("  -h, --help                Print help information");
}

fn fetch_tools_list(
    client: &reqwest::blocking::Client,
    url: &str,
    session_id: &str,
    protocol_version: &str,
    req_id: u64,
) {
    let payload = json!({
        "jsonrpc": "2.0",
        "id": req_id,
        "method": "tools/list",
        "params": {}
    });

    let res = match client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Mcp-Session-Id", session_id)
        .header("Mcp-Protocol-Version", protocol_version)
        .json(&payload)
        .send()
    {
        Ok(r) => r,
        Err(e) => {
            println!("Error fetching tools list: {}", e);
            process::exit(1);
        }
    };

    let body = match res.text() {
        Ok(b) => b,
        Err(e) => {
            println!("Error reading tools list response: {}", e);
            process::exit(1);
        }
    };

    println!("Available Tools (tools/list):");
    if let Ok(res_json) = serde_json::from_str::<serde_json::Value>(&body) {
        if let Some(tools) = res_json.pointer("/result/tools").and_then(|t| t.as_array()) {
            if tools.is_empty() {
                println!("  No tools registered on server.");
            }
            for tool in tools {
                let name = tool.get("name").and_then(|n| n.as_str()).unwrap_or("Unknown");
                let desc = tool.get("description").and_then(|d| d.as_str()).unwrap_or("");
                println!("  - {}: {}", name, desc);
                if let Some(schema) = tool.get("inputSchema") {
                    println!("    Schema: {}", schema);
                }
            }
            return;
        } else if let Ok(pretty) = serde_json::to_string_pretty(&res_json) {
            println!("{}", pretty);
            return;
        }
    }
    println!("{}", body);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut i = 1;

    let mut action_opt: Option<String> = None;
    let mut r_opt: Option<i64> = None;
    let mut g_opt: Option<i64> = None;
    let mut b_opt: Option<i64> = None;
    let mut color_opt: Option<String> = None;
    let mut rainbow_flag = false;
    let mut index_opt: Option<i64> = None;
    let mut mdns_opt: Option<String> = None;
    let mut port_opt: Option<String> = None;
    let mut list_tools_flag = false;
    let mut pos_args: Vec<String> = Vec::new();

    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "--action" => {
                if i + 1 < args.len() {
                    action_opt = Some(args[i + 1].to_lowercase());
                    i += 2;
                } else {
                    println!("Error: --action requires a value");
                    process::exit(1);
                }
            }
            "--mdns" => {
                if i + 1 < args.len() {
                    mdns_opt = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    println!("Error: --mdns requires a value");
                    process::exit(1);
                }
            }
            "--port" => {
                if i + 1 < args.len() {
                    port_opt = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    println!("Error: --port requires a value");
                    process::exit(1);
                }
            }
            "--list-tools" | "--list" | "-l" => {
                list_tools_flag = true;
                i += 1;
            }
            "--r" => {
                if i + 1 < args.len() {
                    let val: i64 = args[i + 1].parse().unwrap_or_else(|_| {
                        println!("Error: --r requires an integer");
                        process::exit(1);
                    });
                    r_opt = Some(val);
                    i += 2;
                } else {
                    println!("Error: --r requires a value");
                    process::exit(1);
                }
            }
            "--g" => {
                if i + 1 < args.len() {
                    let val: i64 = args[i + 1].parse().unwrap_or_else(|_| {
                        println!("Error: --g requires an integer");
                        process::exit(1);
                    });
                    g_opt = Some(val);
                    i += 2;
                } else {
                    println!("Error: --g requires a value");
                    process::exit(1);
                }
            }
            "--b" => {
                if i + 1 < args.len() {
                    let val: i64 = args[i + 1].parse().unwrap_or_else(|_| {
                        println!("Error: --b requires an integer");
                        process::exit(1);
                    });
                    b_opt = Some(val);
                    i += 2;
                } else {
                    println!("Error: --b requires a value");
                    process::exit(1);
                }
            }
            "--color" => {
                if i + 1 < args.len() {
                    color_opt = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    println!("Error: --color requires a value");
                    process::exit(1);
                }
            }
            "--rainbow" => {
                rainbow_flag = true;
                i += 1;
            }
            "--index" => {
                if i + 1 < args.len() {
                    let val: i64 = args[i + 1].parse().unwrap_or_else(|_| {
                        println!("Error: --index requires an integer");
                        process::exit(1);
                    });
                    index_opt = Some(val);
                    i += 2;
                } else {
                    println!("Error: --index requires a value");
                    process::exit(1);
                }
            }
            "-h" | "--help" => {
                print_help();
                process::exit(0);
            }
            flag if flag.starts_with('-') => {
                println!("Error: Unknown argument '{}'", flag);
                print_help();
                process::exit(1);
            }
            pos => {
                pos_args.push(pos.to_string());
                i += 1;
            }
        }
    }

    let valid_actions = ["on", "off", "toggle", "red", "green", "blue", "list"];

    let mut pos_mdns: Option<String> = None;
    let mut pos_port: Option<String> = None;

    if !pos_args.is_empty() {
        let first_arg = pos_args[0].to_lowercase();
        if valid_actions.contains(&first_arg.as_str()) {
            if action_opt.is_none() {
                action_opt = Some(first_arg);
            }
            if pos_args.len() > 1 {
                pos_mdns = Some(pos_args[1].clone());
            }
            if pos_args.len() > 2 {
                pos_port = Some(pos_args[2].clone());
            }
        } else if list_tools_flag
            || r_opt.is_some()
            || g_opt.is_some()
            || b_opt.is_some()
            || color_opt.is_some()
            || rainbow_flag
            || index_opt.is_some()
        {
            pos_mdns = Some(pos_args[0].clone());
            if pos_args.len() > 1 {
                pos_port = Some(pos_args[1].clone());
            }
        } else {
            println!(
                "Error: Invalid action '{}'. Choose from: {} (or use --list-tools)",
                pos_args[0],
                valid_actions.join(", ")
            );
            process::exit(1);
        }
    }

    let mdns = mdns_opt
        .or(pos_mdns)
        .unwrap_or_else(|| "mcp-led".to_string());
    let port = port_opt
        .or(pos_port)
        .unwrap_or_else(|| "8080".to_string());

    let mut tool_args = serde_json::Map::new();
    if let Some(ref act) = action_opt {
        if act != "list" {
            tool_args.insert("action".to_string(), json!(act));
        }
    }
    if let Some(r) = r_opt {
        tool_args.insert("r".to_string(), json!(r));
    }
    if let Some(g) = g_opt {
        tool_args.insert("g".to_string(), json!(g));
    }
    if let Some(b) = b_opt {
        tool_args.insert("b".to_string(), json!(b));
    }
    if let Some(ref color) = color_opt {
        tool_args.insert("color".to_string(), json!(color));
    }
    if rainbow_flag {
        tool_args.insert("rainbow".to_string(), json!(true));
    }
    if let Some(idx) = index_opt {
        tool_args.insert("index".to_string(), json!(idx));
    }

    let is_list_action = action_opt.as_deref() == Some("list");
    if !list_tools_flag && !is_list_action && tool_args.is_empty() {
        print_help();
        process::exit(1);
    }

    let url = format!("http://{}.local:{}/mcp", mdns, port);
    let mut protocol_version = "2025-11-25".to_string();

    let client = reqwest::blocking::Client::new();

    // 1. Initialize session
    let init_payload = json!({
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
    });

    let res = match client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("Mcp-Protocol-Version", &protocol_version)
        .json(&init_payload)
        .send()
    {
        Ok(r) => r,
        Err(e) => {
            println!("Error during initialize: {}", e);
            process::exit(1);
        }
    };

    let session_id = match res.headers().get("Mcp-Session-Id") {
        Some(h) => match h.to_str() {
            Ok(s) => s.to_string(),
            Err(e) => {
                println!("Error reading Mcp-Session-Id header: {}", e);
                process::exit(1);
            }
        },
        None => {
            println!("Error: No Mcp-Session-Id header returned!");
            process::exit(1);
        }
    };

    let body_text = res.text().unwrap_or_default();
    if let Ok(init_json) = serde_json::from_str::<serde_json::Value>(&body_text) {
        if let Some(pv) = init_json.pointer("/result/protocolVersion").and_then(|v| v.as_str()) {
            protocol_version = pv.to_string();
        }
    }

    // 2. Send initialized notification
    let notif_payload = json!({
        "jsonrpc": "2.0",
        "method": "notifications/initialized"
    });

    if let Err(e) = client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("Mcp-Session-Id", &session_id)
        .header("Mcp-Protocol-Version", &protocol_version)
        .json(&notif_payload)
        .send()
    {
        println!("Warning: Error sending notification: {}", e);
    }

    let mut req_id = 1;

    // 3. Fetch tools list if action is 'list' or --list-tools/--list flag is set
    if is_list_action || list_tools_flag {
        fetch_tools_list(&client, &url, &session_id, &protocol_version, req_id);
        req_id += 1;
    }

    // 4. Call tool led_control with the built arguments
    if !tool_args.is_empty() {
        let tool_payload = json!({
            "jsonrpc": "2.0",
            "id": req_id,
            "method": "tools/call",
            "params": {
                "name": "led_control",
                "arguments": serde_json::Value::Object(tool_args)
            }
        });

        let res_tool = match client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("Mcp-Session-Id", &session_id)
            .header("Mcp-Protocol-Version", &protocol_version)
            .json(&tool_payload)
            .send()
        {
            Ok(r) => r,
            Err(e) => {
                println!("Error calling tool: {}", e);
                process::exit(1);
            }
        };

        let body = match res_tool.text() {
            Ok(b) => b,
            Err(e) => {
                println!("Error reading tool response: {}", e);
                process::exit(1);
            }
        };

        if let Ok(res_json) = serde_json::from_str::<serde_json::Value>(&body) {
            if let Some(content) = res_json.pointer("/result/content").and_then(|c| c.as_array()) {
                for item in content {
                    if item.get("type").and_then(|t| t.as_str()) == Some("text") {
                        if let Some(text) = item.get("text").and_then(|t| t.as_str()) {
                            println!("{}", text);
                        }
                    }
                }
                return;
            }
            println!("Tool call executed. Response:\n{}", body);
        } else {
            println!("Tool call executed. Response:\n{}", body);
        }
    }
}
