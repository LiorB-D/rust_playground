use postgres::{Client, NoTls, Error};
use postgres::types::Type;
use std::io::{self, Write};
use chrono::{DateTime, Utc};

fn prompt(label: &str) -> String {
    print!("{}: ", label);
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    input.to_string()
}

struct PgCredentials {
    host: String,
    user: String,
    password: String,
    dbname: String
}

impl PgCredentials {
    fn get_connection_str(&self) -> String {
        format!(
            "host={} user={} password={} dbname={}", 
            self.host, self.user, self.password, self.dbname
        )
    }

    fn from_prompt() -> Self {
        let host = prompt("Host");
        let user = prompt("User");
        let password = prompt("Password");
        let dbname = prompt("Database Name");

        PgCredentials {
            host: host,
            user: user,
            password: password,
            dbname: dbname
        }
    }
}

fn main() -> Result<(), Error> {

    let creds = PgCredentials::from_prompt();

    let mut client = Client::connect(
        &creds.get_connection_str(),
        NoTls,
    )?;

    println!("Connected to database!");

    loop {
        let query = prompt("Query");

        for row in client.query(&query, &[])? {
            for (i, column) in row.columns().iter().enumerate() {
                let value = match *column.type_() {
                    Type::INT4 => row.get::<_, i32>(i).to_string(),
                    Type::INT8 => row.get::<_, i64>(i).to_string(),
                    Type::BOOL => row.get::<_, bool>(i).to_string(),
                    Type::FLOAT4 => row.get::<_, f32>(i).to_string(),
                    Type::FLOAT8 => row.get::<_, f64>(i).to_string(),
                    Type::TIMESTAMPTZ => row.get::<_, DateTime<Utc>>(i).to_string(),
                    _ => row.try_get::<_, String>(i)
                    .unwrap_or_else(|_| format!("<unsupported: {}>", column.type_()))
                };
                print!("{}: {} | ", column.name(), value);
            }
            println!("--------------------------------");
        }
    }

}