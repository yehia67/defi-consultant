-- Create knowledge table
CREATE TABLE knowledge (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    source_id TEXT NOT NULL,
    content TEXT NOT NULL,
    tags TEXT[] NOT NULL,
    created_at TIMESTAMP DEFAULT now(),
    updated_at TIMESTAMP DEFAULT now(),
    UNIQUE(user_id, source_id)
);

-- Create index on user_id for faster lookups
CREATE INDEX idx_knowledge_user_id ON knowledge(user_id);
-- Create index on tags for filtering
CREATE INDEX idx_knowledge_tags ON knowledge USING GIN(tags);
