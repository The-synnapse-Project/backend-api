use clap::Parser;
use db::establish_connection;

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
        #[arg(index = 2)]
        database_url: String,
    },

    /// Show the database
    Show {
        /// The path to the SQLite database file
        #[arg(index = 2)]
        database_url: String,
    },

    /// Seed the database
    Seed {
        /// The path to the SQLite database file
        #[arg(index = 2)]
        database_url: String,
    },
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();
    match args.action {
        Subcommands::Serve { database_url } => api::run_server(&database_url).await?,
        Subcommands::Show { database_url } => {
            let conn = &mut establish_connection(&database_url);
            let persons = db::interactions::Person::get(conn)?;

            for p in persons {
                println!("{}: {} <{}>", p.id, p.name, p.email);
            }
        }
        Subcommands::Seed { database_url } => {
            db::seed(&database_url)?;
        }
    }
    Ok(())
}
