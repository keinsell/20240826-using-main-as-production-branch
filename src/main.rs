use async_std::task;
use miette::set_panic_hook;

mod db {
    use std::fs::{self, File};

    use platform_dirs::AppDirs;
    use sea_orm::{Database, DatabaseConnection};
    use sea_orm_migration::prelude::*;

    use sea_migration::Migrator;

    fn get_database_uri() -> String {
        let application_directories = AppDirs::new(Some("xyz.neuronek.cli"), true).unwrap();

        dbg!(&application_directories);

        let database_file_path = application_directories.data_dir.join("data.db");
        dbg!(&database_file_path);

        fs::create_dir_all(&application_directories.data_dir).unwrap();

        if !&database_file_path.exists() {
            File::create(&database_file_path).unwrap();
        }

        let db_path = database_file_path.into_os_string().into_string();
        let db_uri = "sqlite://".to_string() + &db_path.unwrap();

        dbg!(&db_uri);

        db_uri
    }

    lazy_static::lazy_static! {
    #[derive(Clone, Debug)]
     pub static ref DATABASE_CONNECTION: DatabaseConnection = {
             async_std::task::block_on(async {
                 let db_url = get_database_uri();
                 dbg!(&db_url);
                 println!("Connecting to database at {:#?}", &db_url);
                 Database::connect(db_url).await.unwrap()
             })
         };
     }

    pub(super) async fn migrate_database(database_connection: &DatabaseConnection) {
        let pending_migrations =
            Migrator::get_pending_migrations(&database_connection.into_schema_manager_connection())
                .await
                .unwrap_or_else(|err| {
                    println!("Failed to read pending migrations");
                    panic!("{}", err)
                });

        if !pending_migrations.is_empty() {
            println!("There are {} migrations pending.", pending_migrations.len());
            println!("Applying migrations...");
            Migrator::up(database_connection.into_schema_manager_connection(), None)
                .await
                .unwrap();
        } else {
            println!("Everything is up to date!")
        }
    }
}

mod cli {
    use crate::db;
    use clap::{Parser, Subcommand};
    use std::{ops::Deref, path::PathBuf};

    pub(super) mod substance {
        use clap::{Parser, Subcommand};
        use sea_orm::{
            ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait,
            Set, TryIntoModel,
        };
        use tabled::{Table, Tabled};
        use tabled::settings::Style;

        #[derive(Parser, Debug)]
        #[command(version, about, long_about = None)]
        pub struct CreateSubstance {
            #[arg(short, long)]
            pub name: String,
        }

        #[derive(Parser, Debug)]
        #[command(version, about, long_about = None)]
        pub struct UpdateSubstance {
            #[arg(short, long)]
            pub id: i32,
            #[arg(short, long)]
            pub name: Option<String>,
        }

        #[derive(Parser, Debug)]
        #[command(version, about, long_about = None)]
        pub struct DeleteSubstance {
            #[arg(short, long)]
            pub id: String,
        }

        #[derive(Parser, Debug)]
        #[command(version, about, long_about = None)]
        pub struct ListSubstance {
            #[arg(short = 'l', long, default_value_t = 10)]
            pub limit: u64,
            #[arg(short = 'p', long, default_value_t = 0)]
            pub page: u64,
        }

        #[derive(Subcommand)]
        pub enum SubstanceCommands {
            Create(CreateSubstance),
            Update(UpdateSubstance),
            Delete(DeleteSubstance),
            List(ListSubstance),
        }

        #[derive(Parser)]
        #[command(args_conflicts_with_subcommands = true)]
        pub(crate) struct SubstanceCommand {
            #[command(subcommand)]
            pub command: SubstanceCommands,
        }

        #[derive(Tabled)]
        pub(crate) struct Substance {
            id: i32,
            name: String,
        }

        pub async fn create_substance(
            create_substance_command: CreateSubstance,
            db_conn: &DatabaseConnection,
        ) -> Result<sea_entity::substance::Model, DbErr> {
            let substance_active_model = sea_entity::substance::ActiveModel {
                name: ActiveValue::set(create_substance_command.name),
                ..Default::default()
            };
            let substance_model = substance_active_model.insert(db_conn).await.unwrap();
            substance_model.try_into_model()
        }

        pub async fn update_substance(
            update_substance: UpdateSubstance,
            db_conn: &DatabaseConnection,
        ) -> Result<sea_entity::substance::Model, DbErr> {
            let active_model = sea_entity::substance::ActiveModel {
                id: Set(update_substance.id),
                name: update_substance
                    .name
                    .map(ActiveValue::set)
                    .unwrap_or(ActiveValue::not_set()),
                // ..Default::default()
            };

            active_model.update(db_conn).await.map_err(|err| {
                println!("{}", err);
                err
            })
        }

        pub async fn list_substances(
            list_substance_query: ListSubstance,
            database_connection: &DatabaseConnection,
        ) {
            let entities = sea_entity::substance::Entity::find()
                .paginate(database_connection, list_substance_query.limit)
                .fetch_page(list_substance_query.page)
                .await
                .expect("Substances should be fetched");

            let substances: Vec<Substance> = entities.into_iter().map(|entity| Substance {
                id: entity.id,
                name: entity.name,
            }).collect();

            let mut substance_table = Table::new(substances);
            substance_table.with(Style::rounded());

            println!("{}", substance_table.to_string());
        }

        pub async fn execute_substance_command(
            command: SubstanceCommands,
            database_connection: &DatabaseConnection,
        ) {
            match command {
                SubstanceCommands::Create(payload) => {
                    create_substance(payload, database_connection)
                        .await
                        .expect("Should create substance");
                }
                SubstanceCommands::Update(command) => {
                    update_substance(command, database_connection)
                        .await
                        .expect("Substance should be updated");
                }
                SubstanceCommands::Delete(_) => todo!(),
                SubstanceCommands::List(query) => {
                    let substances = list_substances(query, database_connection).await;
                }
            }
        }
    }
    pub(super) mod ingestion {
        use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};
        use clap::{Parser, Subcommand};
        use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr, TryIntoModel};

        fn parse_humanized_date(s: &str) -> Result<DateTime<Local>, String> {
            fn convert_to_local(naive_dt: NaiveDateTime) -> DateTime<Local> {
                Local.from_local_datetime(&naive_dt).unwrap()
            }

            fuzzydate::parse(s)
                .map(convert_to_local)
                .map_err(|parse_error| format!("Failed to parse: {}", parse_error))
        }

        #[derive(Parser, Debug)]
        #[command(version, about, long_about = None)]
        pub struct CreateIngestion {
            #[arg(short = 's', long)]
            pub substance_id: i32,
            #[arg(short = 'u', long, default_value_t=String::from("mg"))]
            pub dosage_unit: String,
            #[arg(short = 'v', long)]
            pub dosage_amount: f64,
            /// Date of ingestion, by default
            /// current date is used if not provided.
            ///
            /// Date can be provided as timestamp and in human-readable format such as
            /// "today 10:00", "yesterday 13:00", "monday 15:34" which will be later
            /// parsed into proper timestamp.
            #[arg(
                short='t',
                long,
                value_parser=parse_humanized_date,
                default_value_t=Local::now(),
                default_value="now"
            )]
            pub ingestion_date: DateTime<Local>,
        }

        #[derive(Subcommand)]
        pub(crate) enum IngestionCommands {
            Create(CreateIngestion),
        }

        #[derive(Parser)]
        #[command(args_conflicts_with_subcommands = true)]
        pub(crate) struct IngestionCommand {
            #[command(subcommand)]
            pub command: IngestionCommands,
        }

        pub async fn create_ingestion(
            create_ingestion_command: CreateIngestion,
            db_conn: &DatabaseConnection,
        ) -> Result<sea_entity::ingestion::Model, DbErr> {
            let active_model = sea_entity::ingestion::ActiveModel {
                id: Default::default(),
                substance_id: ActiveValue::Set(create_ingestion_command.substance_id),
                dosage_unit: ActiveValue::Set(create_ingestion_command.dosage_unit),
                dosage_value: ActiveValue::Set(create_ingestion_command.dosage_amount),
                ingested_at: ActiveValue::Set(create_ingestion_command.ingestion_date.into()),
                created_at: ActiveValue::Set(Utc::now().into()),
                updated_at: ActiveValue::Set(Utc::now().into()),
            };

            let model = active_model.insert(db_conn).await.unwrap();
            model.try_into_model()
        }

        pub async fn execute_ingestion_command(
            ingestion_command: IngestionCommand,
            db_conn: &DatabaseConnection,
        ) {
            match ingestion_command.command {
                IngestionCommands::Create(payload) => {
                    create_ingestion(payload, db_conn)
                        .await
                        .expect("Should create ingestion");
                }
            }
        }
    }

    #[derive(Subcommand)]
    pub(super) enum ProgramCommand {
        Substance(substance::SubstanceCommand),
        Ingestion(ingestion::IngestionCommand),
    }

    #[derive(Parser)]
    #[command(
        version = "0.0.1-dev",
        about = "Dosage journal that knows!",
        long_about = "🧬 Intelligent dosage tracker application with purpose to monitor supplements, nootropics and psychoactive substances along with their long-term influence on one's mind and body."
    )]
    pub(super) struct Program {
        /// Optional name to operate on
        pub name: Option<String>,

        /// Sets a custom config file
        #[arg(short, long, value_name = "FILE")]
        pub config: Option<PathBuf>,

        /// Turn debugging information on
        #[arg(short, long, action = clap::ArgAction::Count)]
        pub debug: u8,

        #[command(subcommand)]
        pub command: ProgramCommand,
    }

    pub(super) async fn run_program() {
        let cli = Program::parse();

        match cli.command {
            ProgramCommand::Substance(substance_command) => {
                substance::execute_substance_command(
                    substance_command.command,
                    db::DATABASE_CONNECTION.deref(),
                )
                    .await;
            }
            ProgramCommand::Ingestion(ingestion_command) => {
                ingestion::execute_ingestion_command(
                    ingestion_command,
                    db::DATABASE_CONNECTION.deref(),
                )
                    .await;
            }
        }
    }
}

fn main() {
    // set_hook();
    set_panic_hook();

    task::block_on(async {
        db::migrate_database(&db::DATABASE_CONNECTION).await;
        cli::run_program().await;
    });
}

#[cfg(test)]
mod tests {
    use crate::cli::ingestion::{create_ingestion, CreateIngestion};
    use crate::cli::substance::{
        create_substance, list_substances, update_substance, CreateSubstance, ListSubstance,
    };
    use chrono::{DateTime, Local, Utc};
    use sea_orm::{
        ConnectionTrait, Database, DatabaseBackend, DatabaseConnection, DbBackend, EntityTrait,
        MockDatabase, MockExecResult, Schema,
    };

    use super::*;

    /// Utility to use a database that behaves like a real one
    /// instead mock-up in which we know inputs and outputs.
    async fn use_memory_sqlite() -> DatabaseConnection {
        Database::connect("sqlite::memory:").await.unwrap()
    }

    async fn setup_schema(db: &DatabaseConnection) {
        const DB_BACKEND: DbBackend = DbBackend::Sqlite;

        let backend = db.get_database_backend();

        async fn execute_create_table(
            db: &DatabaseConnection,
            backend: &DbBackend,
            entity: impl EntityTrait,
        ) {
            db.execute(backend.build(&Schema::new(DB_BACKEND).create_table_from_entity(entity)))
                .await
                .expect("");
        }

        execute_create_table(db, &backend, sea_entity::substance::Entity).await;
        execute_create_table(db, &backend, sea_entity::ingestion::Entity).await;
    }

    #[async_std::test]
    async fn test_create_substance() {
        let caffeine_fixture = sea_entity::substance::Model {
            id: 1,
            name: "caffeine".to_owned(),
        };

        let db = use_memory_sqlite().await;
        setup_schema(&db).await;

        let command = CreateSubstance {
            name: "caffeine".to_string(),
        };

        let result = create_substance(command, &db).await;
        assert!(result.is_ok());

        let substance = result.unwrap();
        assert_eq!(substance, caffeine_fixture);
    }

    #[async_std::test]
    async fn test_create_substance_with_mock() {
        let caffeine_fixture = sea_entity::substance::Model {
            id: 78,
            name: "caffeine".to_owned(),
        };

        // Create a mock in-memory SQLite database
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results([[caffeine_fixture.clone()]])
            .append_exec_results([MockExecResult {
                last_insert_id: 78,
                rows_affected: 1,
            }])
            .into_connection();

        // Create the command to create a substance
        let command = CreateSubstance {
            name: "Caffeine".to_string(),
        };

        // Call the create_substance function with the command and the reference to the database
        let result = create_substance(command, &db).await;
        assert!(result.is_ok());

        let substance = result.unwrap();
        assert_eq!(substance, caffeine_fixture);
    }

    #[async_std::test]
    async fn test_update_substance_should_fail() {
        let db = use_memory_sqlite().await;
        setup_schema(&db).await;

        let command = cli::substance::UpdateSubstance {
            id: 1,
            name: Option::from("Coffee".to_string()),
        };

        let result = update_substance(command, &db).await;
        assert!(result.is_err());
    }

    #[async_std::test]
    async fn test_list_substances() {
        let caffeine_fixture = sea_entity::substance::Model {
            id: 78,
            name: "caffeine".to_owned(),
        };

        // Create a mock in-memory SQLite database
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results([[caffeine_fixture.clone()]])
            .append_exec_results([MockExecResult {
                last_insert_id: 78,
                rows_affected: 1,
            }])
            .into_connection();

        let substances = list_substances(ListSubstance { limit: 10, page: 0 }, &db)
            .await;
    }

    #[async_std::test]
    async fn test_update_substance() {
        let db = use_memory_sqlite().await;
        setup_schema(&db).await;

        create_substance(
            CreateSubstance {
                name: "caffeine".to_owned(),
            },
            &db,
        )
            .await
            .expect("Substance should be created");

        let command = cli::substance::UpdateSubstance {
            id: 1,
            name: Option::from("Coffee".to_string()),
        };

        let result = update_substance(command, &db).await;
        assert!(result.is_ok());

        let substance = result.unwrap();
        assert_eq!(
            substance,
            sea_entity::substance::Model {
                id: 1,
                name: "Coffee".to_owned(),
            }
        );
    }

    #[async_std::test]
    async fn test_create_ingestion() {
        let caffeine_ingestion = sea_entity::ingestion::Model {
            id: 1,
            substance_id: 1,
            dosage_unit: "mg".to_string(),
            dosage_value: 20.0,
            ingested_at: Utc::now().into(),
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
        };

        let db = use_memory_sqlite().await;
        setup_schema(&db).await;

        create_substance(
            CreateSubstance {
                name: "caffeine".to_owned(),
            },
            &db,
        )
            .await
            .expect("Substance should be created");

        let command = CreateIngestion {
            substance_id: caffeine_ingestion.substance_id,
            dosage_unit: caffeine_ingestion.dosage_unit.clone(),
            dosage_amount: caffeine_ingestion.dosage_value,
            ingestion_date: DateTime::<Local>::default(),
        };

        let result = create_ingestion(command, &db).await;
        assert!(result.is_ok());

        let model = result.unwrap();
        assert_eq!(model.substance_id, 1);
        assert_eq!(model.dosage_unit, "mg");
        assert_eq!(model.dosage_value, 20.0);
    }
}
