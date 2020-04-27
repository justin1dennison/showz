CREATE TABLE IF NOT EXISTS "resources" (
	id INTEGER PRIMARY KEY NOT NULL, 
	name TEXT NOT NULL, 
	episode_id INTEGER NOT NULL,
	FOREIGN KEY
	  (episode_id) REFERENCES episodes (id)
);
