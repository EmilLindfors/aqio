# Claude Instructions for aqio-database

## Refactoring Philosophy

When working on this codebase, follow these principles:

### ✅ **DO: Embrace Quality Over Legacy**
- **Remove old/duplicate files** - Don't keep legacy code for "backward compatibility"
- **Consolidate naming** - Use clear, consistent names (Event, not EnhancedEvent)
- **Respect existing sophisticated schema** - The database design is excellent; adapt code to it
- **Focus on hexagonal architecture** - Domain layer separate from infrastructure
- **Create rich domain models** - Match the sophisticated database features

### ❌ **DON'T: Create Confusion**
- **Don't keep multiple versions** of the same functionality
- **Don't simplify the database schema** - It's well-designed with advanced features
- **Don't create backward compatibility layers** - They just add confusion
- **Don't ignore the existing schema** - Respect categories, location types, registration workflows

### 🏗️ **Architecture Approach**
- **Hexagonal Architecture** - Clean separation between domain, application, infrastructure
- **Repository Patterns** - Use trait interfaces for testability
- **Rich Domain Models** - Models should capture business complexity, not just database fields
- **Proper Error Handling** - Domain errors separate from infrastructure errors

### 🧹 **Refactoring Process**
1. **Understand the existing schema first** - Don't assume it needs simplification
2. **Remove duplicated/legacy code immediately** - Don't preserve "just in case"
3. **Rename confusing names** - Enhanced/Basic prefixes add confusion
4. **Complete one layer at a time** - Finish domain layer before infrastructure
5. **Fix compilation as you go** - Don't accumulate technical debt

### 💾 **Database Schema Respect**
The existing database schema is sophisticated and well-designed:
- **Event categories** instead of simple enums
- **Location types** (physical/virtual/hybrid)
- **Rich invitation workflows** (pending → sent → delivered → opened → accepted/declined)
- **Registration management** with waitlists
- **External contacts** and user integration
- **JSON fields** for extensibility

**Adapt the code to match this design**, not the other way around.

### 🔍 **Search Tools**
- **Always use `rg` (ripgrep) instead of `grep`** - It's faster and has better defaults
- **Use `rg` instead of find commands** for searching file contents
- **Example usage**:
  - `rg "pattern"` - search for pattern in all files
  - `rg -n "pattern"` - show line numbers
  - `rg -B5 -A5 "pattern"` - show context around matches

### 🎯 **Quality Standards**
- **No compilation warnings** - Fix as you go
- **Clear, descriptive naming** - Self-documenting code
- **Proper error handling** - No generic `anyhow` errors in final code
- **Testable architecture** - Repository traits enable mocking
- **Type safety** - Use Rust's type system to prevent runtime errors

### 📝 **Commit Style**
When making changes:
1. **Remove before adding** - Clean up legacy code first
2. **One concept per commit** - Don't mix unrelated changes
3. **Complete features** - Don't leave half-finished functionality
4. **Test as you go** - Ensure compilation at minimum

## Current Status

✅ **COMPLETED**: Hexagonal architecture with rich domain models matching sophisticated schema
🔧 **IN PROGRESS**: Fixing minor compilation errors and completing repository methods
📋 **NEXT**: Basic integration tests and remaining repository implementations

The architecture is solid and production-ready. Only implementation details remain.