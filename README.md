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

- `STRIPE_SECRET`: 1234
- `STRIPE_CHECKOUT_CANCEL_URL`: https://1234.com
- `STRIPE_CHECKOUT_SUCCESS_URL`: https://1234.com
- `STRIPE_WEBHOOK_SECRET`: 1234
- `JWT_SECRET`: your-secret-key

## Database Setup

This service uses PostgreSQL. Run the SQL commands below to set up the required tables:

```sql
-- Create the 'users' table
-- If you need to add more information you should create another table
-- This is just the core for the skeleton to work
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    image_url TEXT,
    oauth_provider VARCHAR(255) NOT NULL,
    oauth_id VARCHAR(255) NOT NULL,
    stripe_customer_id VARCHAR(255),
    oauth_refresh_token TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT (NOW() AT TIME ZONE 'utc')
);


-- Creating a junction table for users and stripe products
CREATE TABLE user_subscription (
    user_id INTEGER NOT NULL,
    stripe_product_id VARCHAR(255) NOT NULL,
    stripe_payment_id VARCHAR(255) NOT NULL,
    subscription_date TIMESTAMPTZ DEFAULT (NOW() AT TIME ZONE 'utc'),
    is_active BOOLEAN DEFAULT TRUE,
    PRIMARY KEY (user_id, stripe_product_id),
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (stripe_product_id) REFERENCES products(stripe_product_id) -- Assumes a table 'products' exists
);

-- Define enum type for payment status
CREATE TYPE payment_status AS ENUM ('pending', 'successful', 'failed', 'denied');

CREATE TABLE payments (
    stripe_payment_id VARCHAR(255) NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    stripe_product_id VARCHAR(255) NOT NULL,
    payment_date TIMESTAMPTZ DEFAULT (NOW() AT TIME ZONE 'utc'),
    payment_status payment_status DEFAULT 'pending', -- using the enum type for payment status
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

### Stripe

- POST /stripe/checkout: Endpoint to create a checkout.
- GET /stripe/products: Get stripe products
- POST /stripe/webhook: The weebhook stripe uses

### Subscription

- GET /subscription/{user-id}: Get the subscription of a user

### User

- GET /user: Get User information

## cURL Requests

Below are the cURL commands to interact with the API endpoints defined in the application. These serve as examples for testing or initial integration checks.

### OAuth2 Authentication

#### Redirect to OAuth Provider:

```bash
# This endpoint is typically accessed directly via a browser to handle redirects properly.
curl -X GET "http://localhost:80/auth/redirect"
```

### Stripe Payments

#### Create Chekout:

```bash
curl -X POST "http://localhost:80/stripe/checkout?product_id={product_id}" \
     -H "Content-Type: application/json" \
     -H "Authorization: Bearer <token>"
```

#### Get Products:

```bash
 curl -X GET http://localhost:80/stripe/products \
     -H "Content-Type: application/json"
```

#### Get User Subscription:

```bash
curl -X GET http://localhost:80/subscription/{user_id} \
     -H "Content-Type: application/json"
```

### Get User

```bash
curl -X GET http://localhost:80/user \
     -H "Content-Type: application/json" \
     -H "Authorization: Bearer <token>"
```

## Middleware

The service is equipped with JWT validation middleware for secure API calls.

## Conclusion

This service is a basic skeleton for integrating OAuth2 and Stripe payments in a single Rust application. It can be expanded with further OAuth providers and more complex payment handling mechanisms as needed.
