# Wixen
<<<<<<< HEAD
A fully accesible desktop client to interact with Mastodon, Bluesky/ATPro networks, and RSS feeds. Other functionality will be added.
=======


Wixen is a cross-platform Rust application using the Winit UI kit and AccessKit for accessibility. It falls back to native platform UIs (Win32 for Windows, Cocoa for macOS) if accessible UI elements are not supported by Winit.

## Features
- Uses Winit for window management
- Integrates AccessKit for accessibility (accesskit_winit = "0.29.0")
- Platform-specific UI fallback: Win32 (Windows), Cocoa (macOS)

## Getting Started
1. Ensure you have Rust and Cargo installed.
2. Build and run the project:
   ```powershell
   cargo run
   ```

## Next Steps
- Implement accessible UI elements using Winit and AccessKit
- Add platform-specific UI fallback logic

---
This README will be updated as the project progresses.
>>>>>>> 5e8524c (Initial commit)
