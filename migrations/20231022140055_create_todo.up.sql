-- Add up migration script here
CREATE TABLE IF NOT EXISTS todos (
                                     id SERIAL PRIMARY KEY ,
                                     description VARCHAR NOT NULL,
                                     done BOOLEAN NOT NULL DEFAULT false
)