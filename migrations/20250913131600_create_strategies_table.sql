-- Create strategies table
CREATE TABLE strategies (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    strategy_id TEXT NOT NULL,
    name TEXT NOT NULL,
    category TEXT NOT NULL,
    description TEXT NOT NULL,
    risk_level TEXT NOT NULL,
    tags TEXT[] NOT NULL,
    steps TEXT[] NOT NULL,
    requirements TEXT[] NOT NULL,
    expected_returns JSONB NOT NULL,
    created_at TIMESTAMP DEFAULT now(),
    updated_at TIMESTAMP DEFAULT now(),
    author TEXT NOT NULL,
    version TEXT NOT NULL,
    UNIQUE(user_id, strategy_id)
);

-- Create index on user_id for faster lookups
CREATE INDEX idx_strategies_user_id ON strategies(user_id);
-- Create index on category for filtering
CREATE INDEX idx_strategies_category ON strategies(category);
-- Create index on tags for filtering
CREATE INDEX idx_strategies_tags ON strategies USING GIN(tags);
