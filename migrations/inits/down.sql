-- This file should undo anything in `up.sql`
DROP TABLE Leagues;
DROP TABLE Users;
DROP TABLE Artists;
DROP TABLE Songs;
DROP TABLE ArtistsSongs;
DROP TABLE Submissions;
DROP TABLE Rounds;
DROP TABLE Votes;
DROP TABLE Comments;
DROP INDEX IF EXISTS idx_artists_songs;
