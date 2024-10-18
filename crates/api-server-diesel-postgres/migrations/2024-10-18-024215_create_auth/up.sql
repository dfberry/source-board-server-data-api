-- Your SQL goes here
CREATE TABLE IF NOT EXISTS github_user_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    github_user_id UUID NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now() NOT NULL
);

CREATE TABLE IF NOT EXISTS github_user_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    github_user_id UUID NOT NULL,
    encrypted_access_token TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now() NOT NULL
);

CREATE TABLE IF NOT EXISTS github_users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    github_id TEXT NOT NULL UNIQUE,
    username TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ DEFAULT now() NOT NULL
);

CREATE TABLE app_user_watch_repos (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    github_user_id UUID NOT NULL,
    org_repo_name TEXT NOT NULL UNIQUE,
    type VARCHAR(30) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT now() NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS unique_user_repo ON app_user_watch_repos (github_user_id, org_repo_name);

ALTER TABLE github_user_sessions ADD CONSTRAINT github_user_sessions_user_id_fk FOREIGN KEY (github_user_id) REFERENCES github_users(id) ON DELETE NO ACTION ON UPDATE NO ACTION;
ALTER TABLE github_user_tokens ADD CONSTRAINT github_user_tokens_user_id_fk FOREIGN KEY (github_user_id) REFERENCES github_users(id) ON DELETE NO ACTION ON UPDATE NO ACTION;
ALTER TABLE app_user_watch_repos ADD CONSTRAINT app_user_watch_repos_user_id_fk FOREIGN KEY (github_user_id) REFERENCES github_users(id) ON DELETE NO ACTION ON UPDATE NO ACTION;