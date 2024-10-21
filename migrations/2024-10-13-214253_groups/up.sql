CREATE TABLE groups(
  id UUID PRIMARY KEY,
  name TEXT NOT NULL,
  created_by_user UUID,
  CONSTRAINT fk_user FOREIGN KEY (created_by_user) REFERENCES users(id) ON DELETE CASCADE
)
