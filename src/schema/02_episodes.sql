CREATE TABLE IF NOT EXISTS "episodes" (
	id INTEGER PRIMARY KEY NOT NULL,
	title TEXT NOT NULL,
	`order` INTEGER  NOT NULL,
	topic_id INTEGER NOT NULL,
	FOREIGN KEY
	  (topic_id) REFERENCES topics (id)
);
