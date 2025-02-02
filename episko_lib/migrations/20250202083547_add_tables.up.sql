-- Add up migration script here
CREATE TABLE IF NOT EXISTS category (
  id BLOB PRIMARY KEY NOT NULL,
  name TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS language (
  id BLOB PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  version TEXT NOT NULL,
  -- Theoretically not needed as the id hash ensures uniqueness
  CONSTRAINT unq UNIQUE (name, version)
);

CREATE TABLE IF NOT EXISTS build_system (
  id BLOB PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  version TEXT NOT NULL,
  -- Theoretically not needed as the id hash ensures uniqueness
  CONSTRAINT unq UNIQUE (name, version)
);

CREATE TABLE IF NOT EXISTS ide (
  id BLOB PRIMARY KEY NOT NULL,
  name TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS metadata (
  id BLOB PRIMARY KEY NOT NULL,
  directory TEXT NOT NULL,
  title TEXT NOT NULL,
  description TEXT,
  preferred_ide INTEGER,
  repository_url TEXT,
  created BLOB NOT NULL,
  updated BLOB NOT NULL,
  FOREIGN KEY (preferred_ide) REFERENCES ide (id)
);

-- n * m Relations
CREATE TABLE IF NOT EXISTS rel_metadata_category (
  metadata_id BLOB NOT NULL,
  category_id BLOB NOT NULL,
  PRIMARY KEY (metadata_id, category_id),
  FOREIGN KEY (metadata_id) REFERENCES metadata (id) ON DELETE CASCADE,
  FOREIGN KEY (category_id) REFERENCES category (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS rel_metadata_language (
  metadata_id BLOB NOT NULL,
  language_id BLOB NOT NULL,
  PRIMARY KEY (metadata_id, language_id),
  FOREIGN KEY (metadata_id) REFERENCES metadata (id) ON DELETE CASCADE,
  FOREIGN KEY (language_id) REFERENCES language (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS rel_metadata_build_system (
  metadata_id BLOB NOT NULL,
  build_system_id BLOB NOT NULL,
  PRIMARY KEY (metadata_id, build_system_id),
  FOREIGN KEY (metadata_id) REFERENCES metadata (id) ON DELETE CASCADE,
  FOREIGN KEY (build_system_id) REFERENCES build_system (id) ON DELETE CASCADE
);
