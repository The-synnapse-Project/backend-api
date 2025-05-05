use clap::Parser;
use db::{establish_connection, models::Person};

#[derive(Parser)]
struct CliArgs {
    /// What to do
    #[command(subcommand)]
    action: Subcommands,
}

#[derive(Parser)]
enum Subcommands {
    /// Run the server
    Serve {
        /// The path to the SQLite database file
        #[arg(short, long)]
        db_url: String,
    },

    /// Show the database
    Show {
        /// The path to the SQLite database file
        #[arg(short, long)]
        db_url: String,
    },

    /// Seed the database
    Seed {
        /// The path to the SQLite database file
        #[arg(short, long)]
        db_url: String,
    },
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();
    match args.action {
        Subcommands::Serve { db_url } => api::run_server(&db_url).await?,
        Subcommands::Show { db_url } => {
            use db::schema::person::dsl::*;
            use diesel::prelude::*;
            let conn = &mut establish_connection(&db_url);
            let persons = person.select(Person::as_select()).load(conn)?;

            for p in persons {
                println!("{}: {} <{}>", p.id, p.name, p.email);
            }
        }
        Subcommands::Seed { db_url } => {
            use db::schema::person::dsl::*;
            use diesel::prelude::*;
            let conn = &mut establish_connection(&db_url);
            let new_person = Person {
                id: "1".to_string(),
                name: "John".to_string(),
                surname: "Doe".to_string(),
                email: "a@a.com".to_string(),
                password_hash: "asd".to_string(),
            };
            diesel::insert_into(person)
                .values(&new_person)
                .execute(conn)?;
            println!(
                "Inserted person: {}: {} <{}>",
                new_person.id, new_person.name, new_person.email
            );
        }
    }
    Ok(())
}
