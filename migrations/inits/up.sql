-- LeagueID = uuid7 for timestamp
CREATE TABLE Leagues (
    id BINARY(16) PRIMARY KEY NOT NULL,
    description TEXT NOT NULL
);

-- UserID = uuid7 for timestamp
CREATE TABLE Users (
    id BINARY(16) PRIMARY KEY NOT NULL,
    first_name varchar(50) NOT NULL,
    last_name varchar(50) NOT NULL,
    phone_num varchar(20) NOT NULL
);

-- Spotify uses a base62 encoding scheme for its IDs which occupy 16 bytes of space for a serialised UUID.
CREATE TABLE Artists (
    id BINARY(16) PRIMARY KEY NOT NULL,
    name varchar(256) NOT NULL
);

CREATE TABLE Songs (
    id BINARY(16) PRIMARY KEY NOT NULL,
    title varchar(256) NOT NULL,
    album_title varchar(256) NOT NULL,
    release_date INT8 NOT NULL
);

-- Join the many-to-many relationship with songs and artists up with inner joins
CREATE TABLE ArtistsSongs (
    artist_id BINARY(16) NOT NULL NOT NULL,
    song_id BINARY(16) NOT NULL,
    FOREIGN KEY (artist_id) REFERENCES Artists(id) ON DELETE CASCADE,
    FOREIGN KEY (song_id) REFERENCES Songs(id) ON DELETE CASCADE,
    PRIMARY KEY (artist_id, song_id)
);
CREATE INDEX idx_artists_songs ON ArtistsSongs(song_id);

-- id = uuid7 for timestamp
CREATE TABLE Submissions (
    id BINARY(16) PRIMARY KEY NOT NULL,
    song_id BINARY(16) NOT NULL,
    round_id BINARY(16) NOT NULL,
    CONSTRAINT s_id FOREIGN KEY (song_id) REFERENCES Songs(id)
    CONSTRAINT c_id FOREIGN KEY (round_id) REFERENCES Rounds(id)
);

-- id = uuid7 for timestamp
CREATE TABLE Rounds (
    id BINARY(16) PRIMARY KEY NOT NULL,
    name varchar(256) NOT NULL,
    description TEXT NOT NULL,
    league_id BINARY(16) NOT NULL,
    CONSTRAINT c_lid FOREIGN KEY (league_id) REFERENCES Leagues(id)
);

-- VoteID = uuid7 for timestamp
CREATE TABLE Votes (
    id BINARY(16) PRIMARY KEY NOT NULL,
    count INT4 NOT NULL,
    user_id BINARY(16) NOT NULL,
    round_id BINARY(16) NOT NULL,
    submission_id BINARY(16) NOT NULL,
    CONSTRAINT c_uid FOREIGN KEY (user_id) REFERENCES Users(id),
    CONSTRAINT c_rid FOREIGN KEY (round_id) REFERENCES Rounds(id),
    CONSTRAINT c_sid FOREIGN KEY (submission_id) REFERENCES Submissions(id)
);

-- M-M join table for the final results of a round. Old vote entries are kept detached in the Votes table.
CREATE TABLE FinalVotes (
    round_id BINARY(16) NOT NULL,
    vote_id BINARY(16) NOT NULL,
    FOREIGN KEY (round_id) REFERENCES Rounds(id) ON DELETE CASCADE,
    FOREIGN KEY (vote_id) REFERENCES Votes(id) ON DELETE CASCADE,
    PRIMARY KEY (round_id, vote_id)
);
CREATE INDEX idx_final_votes ON FinalVotes(vote_id);

-- CommentID = uuid7 for timestamp
CREATE TABLE Comments (
    id BINARY(16) PRIMARY KEY NOT NULL,
    content TEXT NOT NULL,
    vote_id BINARY(16) NOT NULL,
    CONSTRAINT c_vid FOREIGN KEY (vote_id) REFERENCES Votes(id)
);