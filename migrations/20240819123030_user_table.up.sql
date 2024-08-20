-- Add up migration script here
CREATE TABLE IF NOT EXISTS "DEMO"."USER"
(
    user_id serial NOT NULL,
    name character varying(150) NOT NULL,
    email_address character varying(150) NOT NULL,
    enabled boolean NOT NULL DEFAULT FALSE,
    email_confirmed boolean NOT NULL DEFAULT FALSE,
    role VARCHAR(20) NOT NULL DEFAULT 'ROLE_USER' CHECK (role IN ('ROLE_SUPER_USER', 'ROLE_ADMIN', 'ROLE_USER')),
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT pk_user_id PRIMARY KEY (user_id),
    CONSTRAINT uq_user_email_address UNIQUE (email_address)
);