#[derive(Queryable)]
pub struct Mood {
    pub id: i32,
    pub value: i32,
    pub name: String,
}