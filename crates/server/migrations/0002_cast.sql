-- Phase 2b.2: Agendas per Act of the team's characters, set by the game
-- master (A-13). A JSON object from character id to up to three texts;
-- kept with the team, so a character's own data never carries them.
ALTER TABLE teams ADD COLUMN agendas TEXT NOT NULL DEFAULT '{}';
