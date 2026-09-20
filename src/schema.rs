// @generated automatically by Diesel CLI.

diesel::table! {
    Artists (ArtistID) {
        ArtistID -> Nullable<Binary>,
        ArtistName -> Text,
    }
}

diesel::table! {
    ArtistsSongs (ArtistID, SongID) {
        ArtistID -> Binary,
        SongID -> Binary,
    }
}

diesel::table! {
    Comments (CommentID) {
        CommentID -> Nullable<Binary>,
        Content -> Text,
        VoteID -> Binary,
    }
}

diesel::table! {
    Leagues (LeagueID) {
        LeagueID -> Nullable<Binary>,
        LeagueDesc -> Text,
    }
}

diesel::table! {
    Rounds (RoundID) {
        RoundID -> Nullable<Binary>,
        RoundName -> Text,
        RoundDesc -> Text,
        LeagueID -> Binary,
    }
}

diesel::table! {
    Songs (SongID) {
        SongID -> Nullable<Binary>,
        Title -> Text,
        AlbumTitle -> Text,
        ReleaseDate -> BigInt,
    }
}

diesel::table! {
    Submissions (SubmissionID) {
        SubmissionID -> Nullable<Binary>,
        SongID -> Binary,
        RoundID -> Binary,
    }
}

diesel::table! {
    Users (UserID) {
        UserID -> Nullable<Binary>,
        FirstName -> Text,
        LastName -> Text,
        PhoneNum -> Text,
    }
}

diesel::table! {
    Votes (VoteID) {
        VoteID -> Nullable<Binary>,
        Count -> Integer,
        Finalised -> Bool,
        UserID -> Binary,
        RoundID -> Binary,
        SubmissionID -> Binary,
    }
}

diesel::joinable!(ArtistsSongs -> Artists (ArtistID));
diesel::joinable!(ArtistsSongs -> Songs (SongID));
diesel::joinable!(Comments -> Votes (VoteID));
diesel::joinable!(Rounds -> Leagues (LeagueID));
diesel::joinable!(Submissions -> Rounds (RoundID));
diesel::joinable!(Votes -> Rounds (RoundID));
diesel::joinable!(Votes -> Submissions (SubmissionID));
diesel::joinable!(Votes -> Users (UserID));

diesel::allow_tables_to_appear_in_same_query!(
    Artists,
    ArtistsSongs,
    Comments,
    Leagues,
    Rounds,
    Songs,
    Submissions,
    Users,
    Votes,
);
