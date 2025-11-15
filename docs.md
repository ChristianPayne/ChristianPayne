# Tech Stack
## Core Technologies
- **[Rust](https://www.rust-lang.org/)** - Systems programming language for performance and safety
- **[Leptos](https://leptos.dev/)** - Full-stack web framework for Rust with reactive client-side rendering
- **[Leptos Router](https://leptos.dev/router)** - Client-side routing for single-page applications

## UI & Styling
- **[Thaw](https://github.com/thaw-ui/thaw)** - Component library for Leptos applications
- **[Tailwind CSS](https://tailwindcss.com/)** - Utility-first CSS framework for styling

## Build Tools
- **[Trunk](https://trunkrs.dev/)** - Build tool for Rust WebAssembly applications

## Development Environment Setup
Ensure the following is installed:

**Trunk**
```bash
cargo install trunk
```

# Build Commands
## Development
```bash
# Start development server with hot reload
trunk serve
# The application will be available at `http://localhost:4000` (configured in `Trunk.toml`).

# Build Tailwind CSS in watch mode
tailwindcss -i ./styles/tailwind.css -o ./src/tailwind.css --watch
```

## Production
```bash
# Build for production
trunk build --release

# Build Tailwind CSS for production (minified)
tailwindcss -i ./styles/tailwind.css -o ./src/tailwind.css --minify
```
