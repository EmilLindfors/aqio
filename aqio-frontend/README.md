# AQIO Frontend

This project uses a custom Dioxus component library with CSS-first styling instead of Tailwind CSS.

## Project Structure

```
project/
├─ assets/ # CSS files and other assets
├─ src/
│  ├─ main.rs # Application entry point
│  ├─ components/ # Application-specific components
│  ├─ lib/ # Custom component library
│  └─ api.rs # API client
├─ Cargo.toml # Project dependencies
```

## Component Architecture

The project uses a custom component library with:
- CSS-first styling with data attributes
- Type-safe props using Rust enums
- Proper Props structs with Default traits
- CSS injection per component

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

