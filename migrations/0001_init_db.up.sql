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

CREATE TABLE payments (
    stripe_payment_id VARCHAR(255) NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    payment_date TIMESTAMPTZ DEFAULT (NOW() AT TIME ZONE 'utc'),
    payment_status payment_status DEFAULT 'pending', -- using the enum type for payment status
    FOREIGN KEY (user_id) REFERENCES users(id)
);
