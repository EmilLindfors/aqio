# Venue & Location Management System

**Labels:** `enhancement`, `feature`, `venue`, `medium-priority`

## Overview
Add comprehensive venue and location management to track available spaces, amenities, booking conflicts, and vendor connections.

## User Story
As an event organizer, I want to manage venue information, check availability, and track booking conflicts so that I can efficiently plan events and avoid double-bookings.

## Acceptance Criteria

### Venue Management
- [ ] Venue database with detailed information (capacity, amenities, contact info)
- [ ] Multiple rooms/spaces per venue
- [ ] Equipment and facility tracking
- [ ] Venue photos and virtual tours
- [ ] Accessibility information

### Booking & Availability
- [ ] Real-time availability checking
- [ ] Booking conflict detection
- [ ] Hold/reserve functionality for planning
- [ ] Recurring booking support
- [ ] Cancellation and rescheduling

### Vendor Integration
- [ ] Catering partner connections
- [ ] Equipment rental providers
- [ ] Setup/cleanup service providers
- [ ] Preferred vendor lists per venue

### Location Features
- [ ] Geographic search and mapping
- [ ] Public transport information
- [ ] Parking availability
- [ ] Nearby amenities (hotels, restaurants)

## Database Schema Requirements

### New Tables
```sql
-- Venue/location master data
CREATE TABLE venues (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    
    -- Location information
    address TEXT NOT NULL,
    city TEXT NOT NULL,
    postal_code TEXT,
    country TEXT NOT NULL DEFAULT 'Norway',
    latitude DECIMAL(10, 8),
    longitude DECIMAL(11, 8),
    
    -- Contact information
    contact_name TEXT,
    contact_email TEXT,
    contact_phone TEXT,
    website_url TEXT,
    
    -- Facility information
    total_capacity INTEGER,
    parking_spaces INTEGER,
    accessibility_features TEXT, -- JSON array
    wifi_available BOOLEAN NOT NULL DEFAULT FALSE,
    catering_available BOOLEAN NOT NULL DEFAULT FALSE,
    
    -- Status and metadata
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    requires_approval BOOLEAN NOT NULL DEFAULT FALSE,
    booking_lead_time_hours INTEGER DEFAULT 24,
    cancellation_policy TEXT,
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Rooms/spaces within venues
CREATE TABLE venue_spaces (
    id TEXT PRIMARY KEY,
    venue_id TEXT NOT NULL REFERENCES venues(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    
    -- Space specifications
    capacity INTEGER NOT NULL,
    area_sqm INTEGER,
    space_type TEXT NOT NULL CHECK(space_type IN (
        'conference_room', 'auditorium', 'classroom', 'banquet_hall', 
        'outdoor_space', 'exhibition_hall', 'breakout_room'
    )),
    
    -- Equipment and features
    equipment_available TEXT, -- JSON array: ["projector", "sound_system", "whiteboard"]
    seating_arrangements TEXT, -- JSON: {"theater": 100, "classroom": 60, "banquet": 80}
    
    -- Pricing (optional for paid venues)
    hourly_rate DECIMAL(10,2),
    daily_rate DECIMAL(10,2),
    currency TEXT DEFAULT 'NOK',
    
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Venue bookings and availability
CREATE TABLE venue_bookings (
    id TEXT PRIMARY KEY,
    venue_id TEXT NOT NULL REFERENCES venues(id),
    space_id TEXT REFERENCES venue_spaces(id),
    event_id TEXT REFERENCES events(id), -- NULL for holds/blocks
    
    -- Booking details
    booked_by_user_id TEXT NOT NULL REFERENCES users(id),
    booking_type TEXT NOT NULL CHECK(booking_type IN ('confirmed', 'hold', 'block')) DEFAULT 'confirmed',
    
    -- Time slot
    start_datetime DATETIME NOT NULL,
    end_datetime DATETIME NOT NULL,
    setup_time_minutes INTEGER DEFAULT 0,
    cleanup_time_minutes INTEGER DEFAULT 0,
    
    -- Booking information
    purpose TEXT,
    expected_attendees INTEGER,
    special_requirements TEXT,
    
    -- Status
    status TEXT NOT NULL CHECK(status IN ('active', 'cancelled', 'completed')) DEFAULT 'active',
    cancellation_reason TEXT,
    
    -- Confirmation and approval
    requires_approval BOOLEAN NOT NULL DEFAULT FALSE,
    approval_status TEXT CHECK(approval_status IN ('pending', 'approved', 'rejected')),
    approved_by_user_id TEXT REFERENCES users(id),
    approved_at DATETIME,
    
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Vendor/service provider database
CREATE TABLE venue_vendors (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    vendor_type TEXT NOT NULL CHECK(vendor_type IN (
        'catering', 'av_equipment', 'security', 'cleaning', 'decoration', 'transportation'
    )),
    
    -- Contact information
    contact_name TEXT,
    contact_email TEXT,
    contact_phone TEXT,
    website_url TEXT,
    
    -- Service information
    description TEXT,
    services_offered TEXT, -- JSON array
    coverage_areas TEXT, -- JSON array of cities/regions
    
    -- Ratings and verification
    average_rating DECIMAL(3,2),
    total_reviews INTEGER DEFAULT 0,
    is_verified BOOLEAN NOT NULL DEFAULT FALSE,
    
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Venue-vendor relationships (preferred vendors per venue)
CREATE TABLE venue_vendor_relationships (
    venue_id TEXT NOT NULL REFERENCES venues(id) ON DELETE CASCADE,
    vendor_id TEXT NOT NULL REFERENCES venue_vendors(id) ON DELETE CASCADE,
    relationship_type TEXT NOT NULL CHECK(relationship_type IN ('preferred', 'exclusive', 'available')),
    notes TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    PRIMARY KEY (venue_id, vendor_id)
);

-- Venue photos and media
CREATE TABLE venue_media (
    id TEXT PRIMARY KEY,
    venue_id TEXT NOT NULL REFERENCES venues(id) ON DELETE CASCADE,
    space_id TEXT REFERENCES venue_spaces(id) ON DELETE CASCADE,
    
    media_type TEXT NOT NULL CHECK(media_type IN ('photo', 'video', 'virtual_tour', 'floor_plan')),
    file_url TEXT NOT NULL,
    title TEXT,
    description TEXT,
    sort_order INTEGER DEFAULT 0,
    
    is_primary BOOLEAN NOT NULL DEFAULT FALSE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

### Modified Tables
```sql
-- Enhanced location information for events
ALTER TABLE events ADD COLUMN venue_id TEXT REFERENCES venues(id);
ALTER TABLE events ADD COLUMN space_id TEXT REFERENCES venue_spaces(id);
ALTER TABLE events ADD COLUMN booking_id TEXT REFERENCES venue_bookings(id);
```

## API Endpoints Needed
- `GET /api/venues/search` - Search venues by location, capacity, amenities
- `GET /api/venues/{id}/availability` - Check availability for date range
- `POST /api/venues/{id}/book` - Create booking or hold
- `GET /api/venues/{id}/vendors` - Get preferred vendors for venue
- `POST /api/venues/{id}/reviews` - Add venue review/rating
- `GET /api/bookings/conflicts` - Check for booking conflicts

## Features in Detail

### Venue Search & Filtering
```json
{
  "location": {
    "city": "Bergen",
    "radius_km": 25,
    "coordinates": {"lat": 60.3913, "lng": 5.3221}
  },
  "capacity": {"min": 50, "max": 200},
  "date_range": {
    "start": "2024-09-18T16:00:00Z",
    "end": "2024-09-18T23:00:00Z"
  },
  "amenities": ["parking", "wifi", "catering", "accessibility"],
  "space_type": "conference_room"
}
```

### Availability Checking
- Real-time availability with setup/cleanup buffer times
- Recurring booking conflict detection
- Hold/reserve slots for planning
- Waitlist for popular venues

### Equipment & Amenities Tracking
```json
{
  "audio_visual": ["projector", "sound_system", "microphones", "screens"],
  "seating": {
    "theater": 150,
    "classroom": 80,
    "banquet": 100,
    "cocktail": 200
  },
  "catering": {
    "kitchen_available": true,
    "approved_vendors": ["vendor-1", "vendor-2"],
    "restrictions": ["no_alcohol", "kosher_available"]
  }
}
```

## Integration Points
- **Google Maps API**: For location services and directions
- **Norwegian Address API**: For address validation
- **Public Transport APIs**: For transit information
- **Calendar Systems**: For booking synchronization

## UI/UX Features
- Interactive venue map with filtering
- Photo galleries and virtual tours
- Availability calendar widget
- Booking confirmation flow
- Vendor comparison interface

## Testing Requirements
- [ ] Availability calculation accuracy
- [ ] Booking conflict detection
- [ ] Geographic search performance
- [ ] Concurrent booking handling
- [ ] Data integrity for venue-event relationships

## Documentation Needed
- Venue onboarding guide
- Booking workflow documentation
- Integration setup (maps, transit)
- Vendor management guide

## Dependencies
- Mapping service integration
- Image storage and processing
- Geographic calculation utilities
- Address validation services

## Estimated Effort
**Medium-Large (3-4 weeks)** - Complex geographic features and booking logic.

## Priority
**Medium** - Valuable for professional event organizers and venue management.

## Norwegian Market Considerations
- Integration with Norwegian address systems (Kartverket)
- Support for common Norwegian venue types
- Local vendor ecosystem connections
- Norwegian accessibility standards compliance