-- this file is used for postgresql database initialization
-- create user table
CREATE TABLE IF NOT EXISTS users (
    id bigserial PRIMARY KEY,
    fullname VARCHAR(64) NOT NULL,
    -- hashed argon2 password length 97
    password_hash VARCHAR(97) NOT NULL,
    email VARCHAR(64) NOT NULL,
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP
);
-- create unique index for users for email
CREATE UNIQUE INDEX IF NOT EXISTS email_idx ON users(email);

-- create chat type: sigle, group, private_channel, public_channel
CREATE TYPE chat_type AS ENUM ('single', 'group', 'private_channel', 'public_channel');

-- create chat table
CREATE TABLE IF NOT EXISTS chats (
    id bigserial PRIMARY KEY,
    name VARCHAR(128) NOT NULL UNIQUE,
    type chat_type NOT NULL,
    -- user id list
    members bigint[] NOT NULL,
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP
);

-- crate message table
CREATE TABLE IF NOT EXISTS messages (
    id bigserial PRIMARY KEY,
    chat_id bigint NOT NULL REFERENCES chats(id),
    sender_id bigint NOT NULL REFERENCES users(id),
    content text NOT NULL,
    images text[],
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP
);


-- create index for messages for chat_id and createed_at order by created_at desc
CREATE INDEX IF NOT EXISTS chat_id_created_at_idx ON messages(chat_id, created_at DESC);

-- create index for messages for sender_id
CREATE INDEX IF NOT EXISTS sender_id_idx ON messages(sender_id, created_at DESC);
