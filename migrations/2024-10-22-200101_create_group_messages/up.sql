CREATE TABLE group_messages(
  id UUID PRIMARY KEY,
  message TEXT NOT NULL,
  sender UUID NOT NULL,
  to_group UUID NOT NULL,
  sequence SERIAL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,  
  CONSTRAINT fk_group_sender FOREIGN KEY (sender) REFERENCES users(id) ON DELETE CASCADE,
  CONSTRAINT fk_group_chat FOREIGN KEY (to_group) REFERENCES groups(id) ON DELETE CASCADE
)
