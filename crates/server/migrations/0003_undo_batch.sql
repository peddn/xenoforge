-- Abnahme Phase 2: the log entries a snapshot's command wrote, as the
-- number of their first entry (`batch`); undo names and marks them. Empty
-- when the command wrote none, and for snapshots from before.
ALTER TABLE snapshots ADD COLUMN batch INTEGER;
