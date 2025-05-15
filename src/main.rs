use std::env;

use clap::Parser;
use db::establish_connection;
use dotenv::dotenv;
use fstdout_logger::{LoggerConfig, init_logger_with_config};
use log::{LevelFilter, debug, error, info, warn};

#[derive(Parser)]
#[command(
    name = "synnapse-db-api-cli",
    version,
    about = "A CLI for the Synnapse project for connecting to database with an API"
)]
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
        #[arg()]
        database_url: Option<String>,
    },

    /// Show the database
    Show {
        /// The path to the SQLite database file
        #[arg()]
        database_url: Option<String>,
    },

    /// Seed the database
    Seed {
        /// The path to the SQLite database file
        #[arg()]
        database_url: Option<String>,
    },
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log_level = match env::var("SYN_LOG_LEVEL") {
        Ok(level) => match level.parse::<u8>() {
            Ok(level) => match level {
                0 => LevelFilter::Off,
                1 => LevelFilter::Error,
                2 => LevelFilter::Warn,
                3 => LevelFilter::Info,
                4 => LevelFilter::Debug,
                _ => LevelFilter::Trace,
            },
            Err(_) => LevelFilter::Warn,
        },
        Err(_) => LevelFilter::Warn,
    };
    if let Err(e) = init_logger_with_config(
        Some("latest.log"),
        LoggerConfig::builder()
            .show_file_info(cfg!(debug_assertions))
            .level(log_level)
            .build(),
    ) {
        eprintln!("Failed to initialize logger: {}", e);
        return Err(Box::new(e) as Box<dyn std::error::Error>);
    }

    let args = CliArgs::parse();

    let status = dotenv().ok();
    if let Some(status) = status {
        info!("Loaded .env file: {}", status.to_str().unwrap());
    } else {
        warn!("No .env file found");
    }

    info!("Starting synnapse-db-api-cli");
    debug!("Logger initialized");

    match args.action {
        Subcommands::Serve { database_url } => {
            let database_url = match database_url {
                Some(url) => url,
                None => {
                    if let Ok(url) = std::env::var("DATABASE_URL") {
                        info!("Using DATABASE_URL from environment: {}", url);
                        if let Err(e) = api::run_server(&url).await {
                            error!("Server error: {}", e);
                            return Err(e.into());
                        }
                        info!("Server shut down gracefully");
                        return Ok(());
                    } else {
                        warn!("No DATABASE_URL found in environment");
                    }
                    let command = std::env::args()
                        .next()
                        .unwrap_or_else(|| "synnapse-db-api-cli".to_string());
                    println!(
                        "DATABASE_URL not set.\nUsage: {} serve <DATABASE_URL>",
                        command
                    );
                    return Err("No database URL provided".into());
                }
            };
            info!("Starting server with database at: {}", database_url);
            if let Err(e) = api::run_server(&database_url).await {
                error!("Server error: {}", e);
                return Err(e.into());
            }
            info!("Server shut down gracefully");
        }
        Subcommands::Show { database_url } => {
            let database_url = match database_url {
                Some(url) => url,
                None => {
                    let command = std::env::args()
                        .next()
                        .unwrap_or_else(|| "synnapse-db-api-cli".to_string());
                    println!(
                        "DATABASE_URL not set.\nUsage: {} serve <DATABASE_URL>",
                        command
                    );
                    return Err("No database URL provided".into());
                }
            };
            info!("Showing database contents from: {}", database_url);
            let conn = &mut establish_connection(&database_url);
            match db::interactions::person::PersonInteractor::get(conn) {
                Ok(persons) => {
                    if persons.is_empty() {
                        warn!("No persons found in database");
                    } else {
                        info!("Found {} persons in database", persons.len());
                        for p in persons {
                            info!("{}: {} <{}>", p.id, p.name, p.email);
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to retrieve persons: {}", e);
                    return Err(e.into());
                }
            }
        }
        Subcommands::Seed { database_url } => {
            let database_url = match database_url {
                Some(url) => url,
                None => {
                    let command = std::env::args()
                        .next()
                        .unwrap_or_else(|| "synnapse-db-api-cli".to_string());
                    println!(
                        "DATABASE_URL not set.\nUsage: {} serve <DATABASE_URL>",
                        command
                    );
                    return Err("No database URL provided".into());
                }
            };
            info!("Seeding database at: {}", database_url);
            if let Err(e) = db::seed(&database_url) {
                error!("Failed to seed database: {}", e);
                return Err(e);
            }
            info!("Database seeded successfully");
        }
    }
    info!("Program completed successfully");
    Ok(())
}
