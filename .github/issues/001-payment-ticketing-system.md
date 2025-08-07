# Payment & Ticketing System

**Labels:** `enhancement`, `feature`, `payment`, `high-priority`

## Overview
Add comprehensive payment and ticketing functionality to support paid events, ticket sales, and revenue management.

## User Story
As an event organizer, I want to create paid events with different ticket types so that I can monetize my events and manage capacity through pricing.

## Acceptance Criteria

### Core Payment Features
- [ ] Support for multiple ticket types (Early Bird, Regular, VIP, Student)
- [ ] Integration with payment processors (Stripe, PayPal, Vipps for Norway)
- [ ] Automatic invoice generation and receipt sending
- [ ] Refund and cancellation management
- [ ] Tax calculation and reporting

### Ticketing Features  
- [ ] Discount codes and promotional pricing
- [ ] Group/bulk ticket purchases
- [ ] Ticket transfer between users
- [ ] Digital ticket generation with QR codes
- [ ] Print-at-home ticket options

### Revenue Management
- [ ] Payment status tracking (pending, completed, failed, refunded)
- [ ] Revenue reporting and analytics
- [ ] Payout scheduling and tracking
- [ ] Fee calculation (platform fees, payment processor fees)

## Database Schema Requirements

### New Tables Needed
```sql
-- Ticket types for events
CREATE TABLE ticket_types (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL REFERENCES events(id),
    name TEXT NOT NULL, -- "Early Bird", "Regular", "VIP"
    description TEXT,
    price DECIMAL(10,2) NOT NULL,
    currency TEXT NOT NULL DEFAULT 'NOK',
    quantity_total INTEGER,
    quantity_sold INTEGER NOT NULL DEFAULT 0,
    sale_start_date DATETIME,
    sale_end_date DATETIME,
    is_active BOOLEAN NOT NULL DEFAULT TRUE
);

-- Individual ticket purchases
CREATE TABLE tickets (
    id TEXT PRIMARY KEY,
    ticket_type_id TEXT NOT NULL REFERENCES ticket_types(id),
    event_id TEXT NOT NULL REFERENCES events(id),
    registration_id TEXT NOT NULL REFERENCES event_registrations(id),
    order_id TEXT NOT NULL REFERENCES orders(id),
    ticket_code TEXT UNIQUE NOT NULL, -- QR code content
    status TEXT NOT NULL CHECK(status IN ('active', 'used', 'cancelled', 'refunded'))
);

-- Payment orders
CREATE TABLE orders (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL REFERENCES events(id),
    purchaser_email TEXT NOT NULL,
    purchaser_name TEXT NOT NULL,
    total_amount DECIMAL(10,2) NOT NULL,
    currency TEXT NOT NULL DEFAULT 'NOK',
    payment_method TEXT NOT NULL,
    payment_status TEXT NOT NULL CHECK(payment_status IN ('pending', 'completed', 'failed', 'refunded')),
    payment_processor_id TEXT, -- Stripe payment intent ID, etc.
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Discount codes
CREATE TABLE discount_codes (
    id TEXT PRIMARY KEY,
    event_id TEXT REFERENCES events(id), -- NULL for global codes
    code TEXT NOT NULL UNIQUE,
    discount_type TEXT NOT NULL CHECK(discount_type IN ('percentage', 'fixed')),
    discount_value DECIMAL(10,2) NOT NULL,
    max_uses INTEGER,
    used_count INTEGER NOT NULL DEFAULT 0,
    valid_from DATETIME,
    valid_until DATETIME,
    is_active BOOLEAN NOT NULL DEFAULT TRUE
);
```

## API Endpoints Needed
- `POST /api/events/{id}/ticket-types` - Create ticket types
- `GET /api/events/{id}/tickets/available` - Check ticket availability  
- `POST /api/tickets/purchase` - Purchase tickets (creates order + payment intent)
- `POST /api/tickets/{id}/transfer` - Transfer ticket to another person
- `POST /api/orders/{id}/refund` - Process refund
- `GET /api/orders/{id}/invoice` - Download invoice PDF

## Integration Requirements
- **Stripe**: For international card payments
- **Vipps**: For Norwegian mobile payments  
- **PayPal**: Alternative payment method
- **PDF Generation**: For invoices and tickets
- **QR Code Library**: For ticket validation

## Security Considerations
- PCI DSS compliance for payment data
- Encrypted storage of payment processor tokens
- Audit logging for all financial transactions
- Rate limiting on payment endpoints
- Webhook signature validation

## Testing Requirements
- [ ] Unit tests for payment calculations
- [ ] Integration tests with payment processor webhooks
- [ ] Load testing for high-traffic ticket sales
- [ ] Security testing for payment flows

## Documentation Needed
- Payment processor setup guide
- Ticket type configuration examples
- Refund policy templates
- Tax reporting documentation

## Dependencies
- `stripe` or `stripe-rust` crate
- `reqwest` for API calls
- `pdf-writer` for invoice generation
- `qrcode` crate for QR code generation

## Estimated Effort
**Large (3-4 weeks)** - Complex integration with multiple payment systems and significant database schema changes.

## Priority
**High** - Essential for monetizing events and expanding beyond free events.