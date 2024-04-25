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


