# Tauri + React + TanStack Router + shadcn/ui

A modern cross-platform application built with Tauri 2.0, supporting **Desktop (Windows, macOS, Linux)** and **Mobile (iOS, Android)** from a single codebase.

## Tech Stack

- **[Tauri 2.0](https://v2.tauri.app/)** - Build lightweight desktop applications with web technologies
- **[React 19](https://react.dev/)** - Modern React with latest features
- **[TanStack Router](https://tanstack.com/router)** - Type-safe, client-side routing (no SSR)
- **[shadcn/ui](https://ui.shadcn.com/)** - Beautiful, accessible component library
- **[Tailwind CSS v4](https://tailwindcss.com/)** - Utility-first CSS framework
- **[TypeScript](https://www.typescriptlang.org/)** - Type-safe JavaScript
- **[Vite](https://vitejs.dev/)** - Fast build tool and dev server

## Supported Platforms

- 🖥️ **Desktop**: Windows, macOS, Linux
- 📱 **Mobile**: iOS, Android

## Features

- ✅ Cross-platform: Build for 5 platforms from a single codebase
- ✅ File-based routing with TanStack Router (client-side, no SSR)
- ✅ Type-safe navigation and route parameters
- ✅ shadcn/ui components with Tailwind CSS v4
- ✅ Path aliases configured (`@/*` points to `src/*`)
- ✅ Dark mode support with shadcn/ui theming
- ✅ TanStack Router DevTools for development
- ✅ Platform-specific configurations for optimal builds

## Getting Started

### Prerequisites

#### Core Requirements (All Platforms)

1. **Rust**: Install via [rustup](https://rustup.rs/)
2. **Node.js**: v18+ ([nodejs.org](https://nodejs.org/))
3. **Package Manager**: Choose one:
   - **pnpm**: `npm install -g pnpm` or `corepack enable`
   - **bun**: [Install from bun.sh](https://bun.sh/)

   > **Note**: This project is configured for `bun` by default. To use `pnpm`, update `beforeDevCommand` and `beforeBuildCommand` in `src-tauri/tauri.conf.json`

#### Desktop Prerequisites

##### Linux
```bash
# Debian/Ubuntu
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev

# Arch
sudo pacman -Syu
sudo pacman -S webkit2gtk-4.1 base-devel curl wget file openssl appmenu-gtk-module libappindicator-gtk3 librsvg xdotool
```

##### macOS
- Install [Xcode](https://apps.apple.com/app/xcode/id497799835) from Mac App Store
- Or for desktop-only: `xcode-select --install`

##### Windows
- Install [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
  - Select "Desktop development with C++"
- [WebView2](https://developer.microsoft.com/microsoft-edge/webview2/) (pre-installed on Windows 10 v1803+)

#### Mobile Prerequisites

##### Android
1. Install [Android Studio](https://developer.android.com/studio)
2. Set environment variables:
   ```bash
   export JAVA_HOME=/path/to/jdk
   export ANDROID_HOME=/path/to/Android/Sdk
   export NDK_HOME=$ANDROID_HOME/ndk/version
   ```
3. Install via SDK Manager:
   - Android SDK Platform
   - Android SDK Platform-Tools
   - NDK (Side by side)
   - Android SDK Build-Tools
   - Android SDK Command-line Tools
4. Add Rust targets:
   ```bash
   rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
   ```

##### iOS (macOS only)
1. Install [Xcode](https://apps.apple.com/app/xcode/id497799835) (full version, not Command Line Tools)
2. Install Homebrew: `/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"`
3. Install CocoaPods: `brew install cocoapods`
4. Add Rust targets:
   ```bash
   rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim
   ```

### Installation

```bash
# Using bun (default)
bun install

# Or using pnpm
pnpm install
```

### Development

#### Desktop
```bash
# Using bun (default)
bun run tauri:dev

# Or using pnpm
pnpm tauri:dev
```

#### Android
```bash
# First time only
bun run tauri:android:init  # or: pnpm tauri:android:init

# Development
bun run tauri:android:dev   # or: pnpm tauri:android:dev
```

#### iOS (macOS only)
```bash
# First time only
bun run tauri:ios:init      # or: pnpm tauri:ios:init

# Development
bun run tauri:ios:dev       # or: pnpm tauri:ios:dev
```

### Build

#### Desktop
```bash
bun run tauri:build         # or: pnpm tauri:build
```

#### Android
```bash
bun run tauri:android:build # or: pnpm tauri:android:build
```

#### iOS
```bash
bun run tauri:ios:build     # or: pnpm tauri:ios:build
```

## Project Structure

```
tauri-app/
├── src/
│   ├── routes/                    # File-based routes
│   │   ├── __root.tsx             # Root layout with navigation
│   │   ├── index.tsx              # Home page (/)
│   │   └── about.tsx              # About page (/about)
│   ├── components/
│   │   └── ui/                    # shadcn/ui components
│   ├── lib/
│   │   └── utils.ts               # Utility functions
│   ├── main.tsx                   # Application entry point
│   └── index.css                  # Global styles with Tailwind
├── src-tauri/                     # Tauri/Rust backend
│   ├── tauri.conf.json            # Main Tauri configuration
│   ├── tauri.android.conf.json    # Android-specific config
│   ├── tauri.ios.conf.json        # iOS-specific config
│   ├── tauri.windows.conf.json    # Windows-specific config
│   ├── tauri.macos.conf.json      # macOS-specific config
│   └── tauri.linux.conf.json      # Linux-specific config
└── tsr.config.json                # TanStack Router configuration
```

## Platform-Specific Configurations

The project includes platform-specific configuration files that Tauri automatically merges with the main configuration:

- **Android** (`tauri.android.conf.json`): Min SDK 24, auto-incrementing version codes
- **iOS** (`tauri.ios.conf.json`): Minimum iOS 14.0
- **Windows** (`tauri.windows.conf.json`): WebView2 installation, WiX installer config
- **macOS** (`tauri.macos.conf.json`): Minimum macOS 10.13, hardened runtime
- **Linux** (`tauri.linux.conf.json`): DEB and AppImage configurations

## Adding Components

Add new shadcn/ui components:

```bash
# Using bun
bunx shadcn@latest add [component-name]

# Or using pnpm
pnpm dlx shadcn@latest add [component-name]
```

## Available Scripts

| Command | Description |
|---------|-------------|
| `bun dev` / `pnpm dev` | Start Vite dev server |
| `bun run build` / `pnpm build` | Build web assets |
| `bun run tauri:dev` / `pnpm tauri:dev` | Start desktop app in dev mode |
| `bun run tauri:build` / `pnpm tauri:build` | Build desktop app for production |
| `bun run tauri:android:init` / `pnpm tauri:android:init` | Initialize Android project |
| `bun run tauri:android:dev` / `pnpm tauri:android:dev` | Run Android app in dev mode |
| `bun run tauri:android:build` / `pnpm tauri:android:build` | Build Android APK |
| `bun run tauri:ios:init` / `pnpm tauri:ios:init` | Initialize iOS project (macOS only) |
| `bun run tauri:ios:dev` / `pnpm tauri:ios:dev` | Run iOS app in dev mode (macOS only) |
| `bun run tauri:ios:build` / `pnpm tauri:ios:build` | Build iOS app (macOS only) |

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Resources

- [Tauri 2.0 Documentation](https://v2.tauri.app/)
- [TanStack Router Documentation](https://tanstack.com/router)
- [shadcn/ui Documentation](https://ui.shadcn.com/)
- [Tauri Prerequisites Guide](https://v2.tauri.app/start/prerequisites/)
