CREATE TYPE approval AS ENUM ('approved','unhandled','unapproved');

CREATE TABLE user_group_join_requests(
  user_id UUID,
  group_id UUID,
  approved APPROVAL DEFAULT 'unhandled', 
  PRIMARY KEY(user_id,group_id),
  CONSTRAINT fk_group_join_user_id FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
  CONSTRAINT fk_group_id FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE
  )

