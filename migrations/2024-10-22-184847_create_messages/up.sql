CREATE TABLE messages(
  id UUID PRIMARY KEY,
  message TEXT NOT NULL,
  sender UUID NOT NULL,
  receiver UUID NOT NULL,
  sequence SERIAL,
  read BOOLEAN DEFAULT false,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,  
  CONSTRAINT fk_sender FOREIGN KEY (sender) REFERENCES users(id) ON DELETE CASCADE,
  CONSTRAINT fk_receiver FOREIGN KEY (receiver) REFERENCES users(id) ON DELETE CASCADE
)
