# Aqio - Comprehensive Event Management Platform

A modern, full-featured event management system built with Rust, supporting everything from intimate birthday parties to large industry conferences.

## 🎯 Overview

Aqio provides a complete event management solution with advanced features for invitations, registration, communication, analytics, and more. Built specifically for Norwegian aquaculture industry events but flexible enough for any event type.

## ✨ Features

### Core Event Management
- 📅 **Event Creation & Management** - Create events with detailed scheduling and location info
- 👥 **Contact & Invitation System** - Manage contacts and send personalized invitations
- ✅ **RSVP & Registration** - Track attendee responses with dietary restrictions and special needs
- 📧 **Email Communications** - Professional email templates with SMTP integration
- 📊 **Analytics & Reporting** - Event performance metrics and attendee insights

### Advanced Features
- 🏢 **Venue Management** - Track venues, spaces, and booking availability
- 🎫 **Ticketing & Payments** - Support for paid events with multiple ticket types
- 🔄 **Recurring Events** - Weekly/monthly meetings and event series
- 👤 **User Profiles** - Extended profiles with preferences and accessibility needs
- 💬 **Event Comments** - Discussion threads for event collaboration
- 📱 **Check-in System** - QR code and mobile check-in support
- 📋 **Post-Event Surveys** - Feedback collection and analysis
- 📁 **File Sharing** - Event documents and photo albums

### Enterprise Features  
- 🏛️ **Multi-Organization Support** - Manage multiple companies/departments
- 🔐 **Role-Based Access Control** - Granular permissions and team management
- 🔗 **API & Webhooks** - Third-party integrations (Slack, Google Calendar, CRM)
- 🌍 **Internationalization** - Multi-language support
- 📈 **Advanced Analytics** - Executive dashboards and ROI tracking

## 🏗️ Architecture

### Backend (Rust)
- **aqio-core** - Shared models and business logic
- **aqio-api** - REST API server with Axum
- **aqio-database** - Database layer with SQLx migrations
- **aqio-frontend** - Dioxus-based web interface

### Database (SQLite/PostgreSQL)
- 25+ tables covering all event management aspects
- Comprehensive migration system
- Full-text search capabilities
- Audit logging and data integrity

### Key Technologies
- **Rust** - Performance and memory safety
- **Axum** - Modern async web framework  
- **SQLx** - Compile-time checked SQL queries
- **Dioxus** - React-like frontend framework
- **Tokio** - Async runtime

## 🚀 Quick Start

### Prerequisites
- Rust 1.75+
- SQLx CLI: `cargo install sqlx-cli`

### Setup
```bash
# Clone repository
git clone https://github.com/your-org/aqio.git
cd aqio

# Setup database
sqlx database create
sqlx migrate run --source aqio-database/migrations

# Run development server
cargo run --bin aqio-api

# Run frontend (separate terminal)
cd aqio-frontend
cargo run
```

### Environment Configuration
Create `.env` file:
```env
DATABASE_URL=sqlite:aqio.db
SMTP_HOST=smtp.aqio.no
SMTP_USERNAME=noreply@aqio.no
KEYCLOAK_URL=https://auth.aqio.no
```

## 📊 Database Schema

Our comprehensive schema includes:

### Core Tables
- **users** - User accounts with Keycloak integration
- **companies** - Norwegian aquaculture organizations  
- **events** - Event details with categorization
- **event_invitations** - Invitation management
- **event_registrations** - RSVP and attendance tracking

### Communication
- **notifications** - Multi-channel notification system
- **email_queue** - Reliable email delivery
- **event_updates** - Event announcements
- **event_comments** - Discussion threads

### Advanced Features
- **venues** & **venue_bookings** - Location management
- **ticket_types** & **orders** - Payment processing
- **event_series** - Recurring event patterns
- **surveys** & **survey_responses** - Feedback collection

## 🎫 Use Cases

### Personal Events
Perfect for birthday parties, family gatherings, and social celebrations:
- Private event settings
- Guest/+1 management  
- Dietary restriction tracking
- Photo sharing

### Professional Events
Ideal for conferences, workshops, and corporate events:
- Multi-tier registration
- Payment processing
- Venue management
- Analytics and reporting

### Recurring Events
Great for weekly meetings and ongoing series:
- Automatic scheduling
- Series-wide registration
- Bulk management tools

## 🔧 Development

### Running Tests
```bash
cargo test --workspace
```

### Database Migrations
```bash
# Create new migration
sqlx migrate add descriptive_name

# Apply migrations
sqlx migrate run --source aqio-database/migrations

# Prepare for offline compilation
sqlx prepare --workspace
```

### Code Generation
```bash
# Update SQLx prepared statements
DATABASE_URL=sqlite:aqio.db cargo sqlx prepare --workspace
```

## 📝 Contributing

1. Check existing [issues](.github/issues/) for planned features
2. Follow GitHub flow: feature branch → PR → review → merge
3. Ensure tests pass and code is formatted (`cargo fmt`)
4. Update documentation for new features

### Feature Requests
See [feature request template](.github/ISSUE_TEMPLATE/feature_request.md) for adding new capabilities.

## 🎯 Roadmap

### Planned Features (see [issues](.github/issues/))
- [ ] **Payment System** - Stripe/Vipps integration for paid events
- [ ] **Recurring Events** - Advanced scheduling patterns  
- [ ] **Venue Management** - Location booking and availability
- [ ] **RBAC System** - Enterprise team management
- [ ] **API Integrations** - Webhooks and third-party connectivity

### Current Status
- ✅ **Core Event Management** - Complete
- ✅ **Invitation System** - Complete  
- ✅ **Email Communications** - Complete
- ✅ **Database Architecture** - Complete
- 🔄 **Frontend Interface** - In Progress
- 📅 **Payment Integration** - Planned
- 📅 **Advanced Analytics** - Planned

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Acknowledgments

- Norwegian aquaculture industry for requirements and feedback
- Rust community for excellent ecosystem and tools
- Open source contributors and maintainers

---

**Built with ❤️ for the Norwegian aquaculture community and event organizers worldwide.**