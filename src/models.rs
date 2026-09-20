use diesel::prelude::*;
use crate::schema;


#[derive(Queryable, Debug, Identifiable)]
#[diesel(table_name = schema::Artists)]
pub struct Artist {
    pub id: Vec<u8>,
    pub name: String,
}

#[derive(Queryable, Debug, Identifiable, Associations)]
#[diesel(belongs_to(Artist))]
#[diesel(belongs_to(Song))]
#[diesel(table_name = schema::ArtistsSongs)]
#[diesel(primary_key(artist_id, song_id))]
pub struct ArtistsSong {
    pub artist_id: Vec<u8>,
    pub song_id: Vec<u8>,
}

#[derive(Queryable, Debug, Identifiable)]
#[diesel(belongs_to(Vote))]
#[diesel(table_name = schema::Comments)]
pub struct Comment {
    pub id: Vec<u8>,
    pub content: String,
    pub vote_id: Vec<u8>,
}

#[derive(Queryable, Debug, Identifiable)]
#[diesel(belongs_to(Round))]
#[diesel(belongs_to(Vote))]
#[diesel(table_name = schema::FinalVotes)]
#[diesel(primary_key(round_id, vote_id))]
pub struct FinalVote {
    pub round_id: Vec<u8>,
    pub vote_id: Vec<u8>,
}

#[derive(Queryable, Debug)]
#[diesel(table_name = schema::Leagues)]
pub struct League {
    pub id: Vec<u8>,
    pub description: String,
}

#[derive(Queryable, Debug)]
#[diesel(belongs_to(League))]
#[diesel(table_name = schema::Rounds)]
pub struct Round {
    pub id: Vec<u8>,
    pub name: String,
    pub description: String,
    pub league_id: Vec<u8>,
}

#[derive(Queryable, Debug)]
#[diesel(table_name = schema::Songs)]
pub struct Song {
    pub id: Vec<u8>,
    pub title: String,
    pub album_title: String,
    pub release_date: i64,
}

#[derive(Queryable, Debug)]
#[diesel(belongs_to(Song))]
#[diesel(belongs_to(Round))]
#[diesel(table_name = schema::Submissions)]
pub struct Submission {
    pub id: Vec<u8>,
    pub song_id: Vec<u8>,
    pub round_id: Vec<u8>,
}

#[derive(Queryable, Debug)]
#[diesel(table_name = schema::Users)]
pub struct User {
    pub id: Vec<u8>,
    pub first_name: String,
    pub last_name: String,
    pub phone_num: String,
}

#[derive(Queryable, Debug)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Round))]
#[diesel(belongs_to(Submission))]
#[diesel(table_name = schema::Votes)]
pub struct Vote {
    pub id: Vec<u8>,
    pub count: i32,
    pub user_id: Vec<u8>,
    pub round_id: Vec<u8>,
    pub submission_id: Vec<u8>,
}

