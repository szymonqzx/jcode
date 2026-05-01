#!/usr/bin/env pwsh
# Test script to capture and analyze debug socket events
# Usage: ./scripts/debug_socket_test.ps1 [capture|compare]

$debugSocket = if ($env:XDG_RUNTIME_DIR) { "$env:XDG_RUNTIME_DIR/jcode-debug.sock" } else { "/tmp/jcode-debug.sock" }
$captureFile = "/tmp/jcode_debug_capture.jsonl"

$command = if ($args) { $args[0] } else { "capture" }

switch ($command) {
    "capture" {
        Write-Output "Connecting to debug socket: $debugSocket"
        Write-Output "Saving events to: $captureFile"
        Write-Output "Press Ctrl+C to stop"
        Write-Output "---"
        # On Windows, use named pipes or adapt for Unix sockets if on WSL
        if ($IsWindows) {
            Write-Error "Unix sockets not supported on Windows. Use WSL or adapt for named pipes."
            exit 1
        }
        nc -U "$debugSocket" | Tee-Object -FilePath $captureFile | jq -c '.'
    }
    "snapshot" {
        Write-Output "Getting state snapshot from debug socket..."
        if ($IsWindows) {
            Write-Error "Unix sockets not supported on Windows. Use WSL or adapt for named pipes."
            exit 1
        }
        timeout 1 nc -U "$debugSocket" | head -1 | jq '.'
    }
    "watch" {
        Write-Output "Watching debug socket events (pretty print)..."
        if ($IsWindows) {
            Write-Error "Unix sockets not supported on Windows. Use WSL or adapt for named pipes."
            exit 1
        }
        nc -U "$debugSocket" | jq '.'
    }
    "analyze" {
        if (Test-Path $captureFile) {
            Write-Output "Analyzing captured events..."
            Write-Output ""
            Write-Output "Event types:"
            jq -r '.type' $captureFile | sort | uniq -c | sort -rn
            Write-Output ""
            $lineCount = (Get-Content $captureFile).Count
            Write-Output "Total events: $lineCount"
        } else {
            Write-Output "No capture file found. Run 'capture' first."
        }
    }
    default {
        Write-Output "Usage: $PSCommandPath [capture|snapshot|watch|analyze]"
        Write-Output "  capture  - Capture events to file and display"
        Write-Output "  snapshot - Get initial state snapshot"
        Write-Output "  watch    - Watch events in real-time (pretty)"
        Write-Output "  analyze  - Analyze captured events"
    }
}
