CREATE TABLE users(
  id UUID PRIMARY KEY,
  nickname TEXT NOT NULL,
  name TEXT NOT NULL,
  surname TEXT NOT NULL,
  email TEXT NOT NULL,
  password TEXT not NULL,
  image TEXT,
   UNIQUE(email),
    UNIQUE(nickname)
)
