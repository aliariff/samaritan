#[derive(Serialize, Deserialize)]

pub struct User {
    pub id: i32,
    pub name: String,
    pub month: String,
    pub size: i32,
    pub electricity_usage: i32,
    pub water_usage: i32,
    pub gas_usage: i32,
    pub zip: String,
    pub country: String,
}
