-- Create users table
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    wallet_address TEXT,
    created_at TIMESTAMP DEFAULT now(),
    updated_at TIMESTAMP DEFAULT now()
);

-- Add foreign key constraints to strategies and knowledge tables
ALTER TABLE strategies ADD CONSTRAINT fk_strategies_user_id FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;
ALTER TABLE knowledge ADD CONSTRAINT fk_knowledge_user_id FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;
