# Tauri + React + TanStack Router + shadcn/ui

A modern desktop application built with Tauri, React, TanStack Router (client-side only, no SSR), and shadcn/ui.

## Tech Stack

- **[Tauri 2.0](https://v2.tauri.app/)** - Build lightweight desktop applications with web technologies
- **[React 19](https://react.dev/)** - Modern React with latest features
- **[TanStack Router](https://tanstack.com/router)** - Type-safe, client-side routing (no SSR)
- **[shadcn/ui](https://ui.shadcn.com/)** - Beautiful, accessible component library
- **[Tailwind CSS v4](https://tailwindcss.com/)** - Utility-first CSS framework
- **[TypeScript](https://www.typescriptlang.org/)** - Type-safe JavaScript
- **[Vite](https://vitejs.dev/)** - Fast build tool and dev server

## Features

- ✅ File-based routing with TanStack Router
- ✅ Type-safe navigation and route parameters
- ✅ shadcn/ui components with Tailwind CSS v4
- ✅ Path aliases configured (`@/*` points to `src/*`)
- ✅ Dark mode support with shadcn/ui theming
- ✅ TanStack Router DevTools for development

## Getting Started

### Prerequisites

Make sure you have installed the prerequisites for your OS: [Tauri Prerequisites](https://tauri.app/start/prerequisites/)

### Installation

```bash
pnpm install
```

### Development

```bash
pnpm tauri dev
```

### Build

```bash
pnpm tauri build
```

## Project Structure

```
tauri-app/
├── src/
│   ├── routes/          # File-based routes
│   │   ├── __root.tsx   # Root layout with navigation
│   │   ├── index.tsx    # Home page (/)
│   │   └── about.tsx    # About page (/about)
│   ├── components/
│   │   └── ui/          # shadcn/ui components
│   ├── lib/
│   │   └── utils.ts     # Utility functions
│   ├── main.tsx         # Application entry point
│   └── index.css        # Global styles with Tailwind
├── src-tauri/           # Tauri/Rust backend
└── tsr.config.json      # TanStack Router configuration
```

## Adding Components

Add new shadcn/ui components:

```bash
pnpm dlx shadcn@latest add [component-name]
```

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
