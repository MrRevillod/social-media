-- This file should undo anything in `up.sql`

DROP TABLE sessions;
DROP TABLE users;
DROP FUNCTION diesel_manage_updated_at(regclass);
DROP FUNCTION diesel_set_updated_at();
