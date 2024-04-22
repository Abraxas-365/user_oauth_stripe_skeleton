# Rust Skeleton Service for OAuth2 Authentication and Stripe Payments

## Overview

This Rust-based service is designed to handle user authentication using OAuth2 and manage payment processes through Stripe. It provides essential endpoints for user sign-up and login using OAuth2, as well as endpoints for creating and updating payment statuses via Stripe.

## Features

- OAuth2 Authentication: Integration with Google's OAuth2 for user authentication.
- User Management: Database management for storing user information securely.
- Stripe Payment Integration: APIs to create payments and update their status.

## Pre-requisites

Ensure you have Rust and Cargo installed on your machine. You can follow the installation guide here: Rust Programming Language.
Additionally, you'll need:

- PostgreSQL server
- Access to a Stripe account for API keys
- Access to Google Cloud for OAuth2 credentials

Configuration
Before starting the service, configure the environmental variables by creating a .env file in the root directory with the following variables:

- `GOOGLE_CLIENT_ID`: Your Google app client ID
- `GOOGLE_CLIENT_SECRET`: Your Google app client secret
- `GOOGLE_REDIRECT_URI`: The redirect URI set in your Google app
- `DATABASE_URL`: Your PostgreSQL database URL

## Database Setup

This service uses PostgreSQL. Run the SQL commands below to set up the required tables:

```sql
-- Create the 'users' table
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    image_url TEXT,
    oauth_provider VARCHAR(255) NOT NULL,
    oauth_id VARCHAR(255) NOT NULL,
    oauth_refresh_token TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT (NOW() AT TIME ZONE 'utc')
);

-- Define enum type for payment status
CREATE TYPE payment_status AS ENUM ('pending', 'successful', 'failed', 'denied');

-- Create the 'payments' table
CREATE TABLE payments (
    stripe_payment_id VARCHAR(255) NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    payment_date TIMESTAMPTZ DEFAULT (NOW() AT TIME ZONE 'utc'),
    payment_status payment_status DEFAULT 'pending',
    FOREIGN KEY (user_id) REFERENCES users(id)
);
```

## Running the Service

To run the service, perform the following commands in the terminal:

```bash
cargo build --release
cargo run
```

or whith Docker

```bash
docker compose build
docker compose up
```

## API Endpoints

### Authentication

- GET /auth/redirect: Redirects to Google OAuth.
- GET /auth/callback: Callback endpoint for Google OAuth.

### Payments

- POST /payments/create: Endpoint to create a payment.
- POST /payments/status: Update payment status.

## cURL Requests

Below are the cURL commands to interact with the API endpoints defined in the application. These serve as examples for testing or initial integration checks.

### OAuth2 Authentication

#### Redirect to OAuth Provider:

```bash
# This endpoint is typically accessed directly via a browser to handle redirects properly.
curl -X GET "http://localhost:8080/auth/redirect"
```

#### OAuth Callback (Example cURL, actual usage via browser after redirect):

```bash
# Example cURL for callback mechanism, note: the 'code' parameter will be provided by the OAuth provider.
curl -X GET "http://localhost:8080/auth/callback?code=AUTHORIZATION_CODE_HERE"
```

### Stripe Payments

#### Create Payment:

```bash
curl -X POST "http://localhost:8080/payments/create" \
     -H "Content-Type: application/json" \
     -H "Authorization: Bearer YOUR_JWT_TOKEN" \
     -d '{"stripe_payment_id": "your_stripe_payment_id"}'
```

#### Update Payment Status:

```bash
curl -X POST "http://localhost:8080/payments/status" \
     -H "Content-Type: application/json" \
     -H "Authorization: Bearer YOUR_JWT_TOKEN" \
     -d '{"stripe_payment_id": "your_stripe_payment_id", "new_status": "successful"}'
```

## Middleware

The service is equipped with JWT validation middleware for secure API calls.

## Conclusion

This service is a basic skeleton for integrating OAuth2 and Stripe payments in a single Rust application. It can be expanded with further OAuth providers and more complex payment handling mechanisms as needed.
