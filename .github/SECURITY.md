# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

The Aqio team takes security vulnerabilities seriously. We appreciate your efforts to responsibly disclose your findings.

### How to Report

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report security vulnerabilities by emailing: **security@aqio.no** (or create a private vulnerability report on GitHub)

Please include the following information:
- Type of issue (e.g. buffer overflow, SQL injection, cross-site scripting, etc.)
- Full paths of source file(s) related to the manifestation of the issue
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit the issue

### Response Timeline

- **Within 48 hours**: We'll acknowledge your report
- **Within 7 days**: We'll provide a detailed response including next steps
- **Within 30 days**: We'll work to resolve critical vulnerabilities

### Security Measures

Aqio implements several security measures:

#### Authentication & Authorization
- Integration with Keycloak for secure user management
- Role-based access control (planned)
- Secure session management
- Input validation and sanitization

#### Data Protection
- SQL injection prevention using parameterized queries (SQLx)
- XSS protection through proper output encoding
- CSRF protection for state-changing operations
- Secure password handling (delegated to Keycloak)

#### Infrastructure Security
- HTTPS-only communication in production
- Secure headers configuration
- Rate limiting to prevent abuse
- Database access controls

#### Code Security
- Regular dependency updates
- Rust's memory safety guarantees
- Comprehensive input validation
- Secure coding practices

### Security Best Practices for Contributors

When contributing to Aqio, please follow these security guidelines:

#### Input Validation
```rust
// DO: Use proper validation
pub struct CreateEventRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    #[validate(email)]
    pub organizer_email: String,
}

// DON'T: Trust user input
// let query = format!("SELECT * FROM events WHERE title = '{}'", user_input);
```

#### Database Access
```rust
// DO: Use parameterized queries
sqlx::query!("SELECT * FROM events WHERE organizer_id = ?", user_id)

// DON'T: String concatenation
// let query = format!("SELECT * FROM events WHERE organizer_id = {}", user_id);
```

#### Error Handling
```rust
// DO: Sanitize error messages
match result {
    Err(_) => return Err(ApiError::InternalError),
}

// DON'T: Expose internal details
// return Err(format!("Database error: {}", db_error));
```

#### Authentication
```rust
// DO: Verify permissions
if !user.can_edit_event(&event_id) {
    return Err(ApiError::Forbidden);
}

// DON'T: Trust client-side data
// if request.is_admin { // Never trust client claims
```

### Disclosure Policy

- We'll work with you to understand and resolve the issue quickly
- We'll keep you informed throughout the process
- Once the vulnerability is resolved, we'll publish a security advisory
- We'll credit you for the discovery (unless you prefer to remain anonymous)

### Bug Bounty Program

Currently, Aqio does not have a formal bug bounty program. However, we greatly appreciate security researchers who help us keep Aqio secure and may offer recognition or small tokens of appreciation for significant findings.

## Security Contact

For security-related questions or concerns, please contact:
- **Email**: security@aqio.no
- **GitHub**: Create a private security advisory

Thank you for helping keep Aqio and its users safe!