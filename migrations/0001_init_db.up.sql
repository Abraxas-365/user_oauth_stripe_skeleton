-- Create the 'users' table
-- If you need to add more information you should create another table
-- This is just the core for the skeleton to work
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    name: VARCHAR(255) NOT NULL,
    image_url TEXT,
    oauth_provider VARCHAR(255) NOT NULL,
    oauth_id VARCHAR(255) NOT NULL,
    stripe_cutomer_id VARCHAR(255),
    oauth_refresh_token TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT (NOW() AT TIME ZONE 'utc')
);

-- Define enum type for payment status
CREATE TYPE payment_status AS ENUM ('pending', 'successful', 'failed', 'denied');

CREATE TABLE payments (
    stripe_payment_id VARCHAR(255) NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    payment_date TIMESTAMPTZ DEFAULT (NOW() AT TIME ZONE 'utc'),
    payment_status payment_status DEFAULT 'pending', -- using the enum type for payment status
    FOREIGN KEY (user_id) REFERENCES users(id)
);


-- If you need to add more information you should create another table
-- This is just the core for the skeleton to work
CREATE TABLE tiers (
    id SERIAL PRIMARY KEY,
    price BIGINT NOT NULL,  
);




CREATE TABLE subscriptions (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    tier_id INTEGER NOT NULL,
    start_date TIMESTAMPTZ DEFAULT (NOW() AT TIME ZONE 'utc'),
    end_date TIMESTAMPTZ,
    active BOOLEAN DEFAULT TRUE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (tier_id) REFERENCES tiers(id) ON DELETE CASCADE
);



