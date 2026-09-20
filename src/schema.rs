// @generated automatically by Diesel CLI.
#![allow(non_snake_case)]

diesel::table! {
    Artists (id) {
        id -> Binary,
        name -> Text,
    }
}

diesel::table! {
    ArtistsSongs (artist_id, song_id) {
        artist_id -> Binary,
        song_id -> Binary,
    }
}

diesel::table! {
    Comments (id) {
        id -> Binary,
        content -> Text,
        vote_id -> Binary,
    }
}

diesel::table! {
    FinalVotes (round_id, vote_id) {
        round_id -> Binary,
        vote_id -> Binary,
    }
}

diesel::table! {
    Leagues (id) {
        id -> Binary,
        description -> Text,
    }
}

diesel::table! {
    Rounds (id) {
        id -> Binary,
        name -> Text,
        description -> Text,
        league_id -> Binary,
    }
}

diesel::table! {
    Songs (id) {
        id -> Binary,
        title -> Text,
        album_title -> Text,
        release_date -> BigInt,
    }
}

diesel::table! {
    Submissions (id) {
        id -> Binary,
        song_id -> Binary,
        round_id -> Binary,
    }
}

diesel::table! {
    Users (id) {
        id -> Binary,
        first_name -> Text,
        last_name -> Text,
        phone_num -> Text,
    }
}

diesel::table! {
    Votes (id) {
        id -> Binary,
        count -> Integer,
        user_id -> Binary,
        round_id -> Binary,
        submission_id -> Binary,
    }
}

diesel::joinable!(ArtistsSongs -> Artists (artist_id));
diesel::joinable!(ArtistsSongs -> Songs (song_id));
diesel::joinable!(Comments -> Votes (vote_id));
diesel::joinable!(FinalVotes -> Rounds (round_id));
diesel::joinable!(FinalVotes -> Votes (vote_id));
diesel::joinable!(Rounds -> Leagues (league_id));
diesel::joinable!(Submissions -> Rounds (round_id));
diesel::joinable!(Submissions -> Songs (song_id));
diesel::joinable!(Votes -> Rounds (round_id));
diesel::joinable!(Votes -> Submissions (submission_id));
diesel::joinable!(Votes -> Users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    Artists,
    ArtistsSongs,
    Comments,
    FinalVotes,
    Leagues,
    Rounds,
    Songs,
    Submissions,
    Users,
    Votes,
);
