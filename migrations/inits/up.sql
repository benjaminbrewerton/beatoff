-- LeagueID = uuid7 for timestamp
CREATE TABLE Leagues (
    LeagueID BINARY(16) PRIMARY KEY,
    LeagueDesc TEXT NOT NULL
);

-- UserID = uuid7 for timestamp
CREATE TABLE Users (
    UserID BINARY(16) PRIMARY KEY,
    FirstName varchar(50) NOT NULL,
    LastName varchar(50) NOT NULL,
    PhoneNum varchar(20) NOT NULL
);

-- Spotify uses a base62 encoding scheme for its IDs which occupy 16 bytes of space for a serialised UUID.
CREATE TABLE Artists (
    ArtistID BINARY(16) PRIMARY KEY,
    ArtistName varchar(256) NOT NULL
);

CREATE TABLE Songs (
    SongID BINARY(16) PRIMARY KEY,
    Title varchar(256) NOT NULL,
    AlbumTitle varchar(256) NOT NULL,
    ReleaseDate INT8 NOT NULL
);

-- Join the many-to-many relationship with songs and artists up with inner joins
CREATE TABLE ArtistsSongs (
    ArtistID BINARY(16) NOT NULL,
    SongID BINARY(16) NOT NULL,
    FOREIGN KEY (ArtistID) REFERENCES Artists(ArtistID) ON DELETE CASCADE,
    FOREIGN KEY (SongID) REFERENCES Songs(SongID) ON DELETE CASCADE,
    PRIMARY KEY (ArtistID, SongID)
);

CREATE INDEX idx_artists_songs ON ArtistsSongs(SongID);

-- SubmissionID = uuid7 for timestamp
CREATE TABLE Submissions (
    SubmissionID BINARY(16) PRIMARY KEY,
    SongID BINARY(16) NOT NULL,
    RoundID BINARY(16) NOT NULL,
    CONSTRAINT c_RoundID FOREIGN KEY (RoundID) REFERENCES Rounds(RoundID)
);

-- RoundID = uuid7 for timestamp
CREATE TABLE Rounds (
    RoundID BINARY(16) PRIMARY KEY,
    RoundName varchar(256) NOT NULL,
    RoundDesc TEXT NOT NULL,
    LeagueID BINARY(16) NOT NULL,
    CONSTRAINT c_LeagueID FOREIGN KEY (LeagueID) REFERENCES Leagues(LeagueID)
);

-- VoteID = uuid7 for timestamp
CREATE TABLE Votes (
    VoteID BINARY(16) PRIMARY KEY,
    Count INT4 NOT NULL,
    Finalised BOOLEAN DEFAULT FALSE NOT NULL,
    UserID BINARY(16) NOT NULL,
    RoundID BINARY(16) NOT NULL,
    SubmissionID BINARY(16) NOT NULL,
    CONSTRAINT c_UserID FOREIGN KEY (UserID) REFERENCES Users(UserID),
    CONSTRAINT c_RoundID FOREIGN KEY (RoundID) REFERENCES Rounds(RoundID),
    CONSTRAINT c_SubmissionID FOREIGN KEY (SubmissionID) REFERENCES Submissions(SubmissionID)
);

-- CommentID = uuid7 for timestamp
CREATE TABLE Comments (
    CommentID BINARY(16) PRIMARY KEY,
    Content TEXT NOT NULL,
    VoteID BINARY(16) NOT NULL,
    CONSTRAINT c_VoteID FOREIGN KEY (VoteID) REFERENCES Votes(VoteID)
);