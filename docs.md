# ChristianPayne Portfolio Documentation

## 🚀 Technology Stack

This project is a modern web application built with the following technologies:

### Core Technologies
- **[Rust](https://www.rust-lang.org/)** - Systems programming language for performance and safety
- **[Leptos](https://leptos.dev/)** - Full-stack web framework for Rust with reactive client-side rendering
- **[Leptos Router](https://leptos.dev/router)** - Client-side routing for single-page applications

### UI & Styling
- **[Thaw](https://github.com/thaw-ui/thaw)** - Component library for Leptos applications
- **[Tailwind CSS](https://tailwindcss.com/)** - Utility-first CSS framework for styling

### Build Tools
- **[Trunk](https://trunkrs.dev/)** - Build tool for Rust WebAssembly applications
- **[Cargo](https://doc.rust-lang.org/cargo/)** - Rust package manager and build system

## 🛠️ Development Environment Setup

### Prerequisites

Before you begin, ensure you have the following installed:

1. **Trunk** (WebAssembly build tool)
   ```bash
   cargo install trunk
   ```

The application will be available at `http://localhost:4000` (configured in `Trunk.toml`).

## 🔧 Build Commands

### Development
```bash
# Start development server with hot reload
trunk serve

# Build Tailwind CSS in watch mode
tailwindcss -i ./styles/tailwind.css -o ./src/tailwind.css --watch
```

### Production
```bash
# Build for production
trunk build --release

# Build Tailwind CSS for production (minified)
tailwindcss -i ./styles/tailwind.css -o ./src/tailwind.css --minify
```

## 🚀 Deployment

The project is configured for easy deployment:
- **Static Files** - Generated in `/dist` directory
- **Netlify Ready** - `.netlify` folder excluded in `.gitignore`
- **WebAssembly** - Optimized for web deployment
