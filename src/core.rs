use clap::Parser;
use miette::Error;
use sea_orm::DatabaseConnection;

// Workaround for applying trait into dervives from clap::command

pub(super) trait Command {}
impl<T: Parser> Command for T {}

// It's not really necessary abstraction in codebase, yet I find it easier to add handlers into actual commands that are used by clap library. Taking LogIngestion as example it reduced need for log_ingestion function which could easily gone out name scheme and produce unecessary mess in codebase. We have clap commands which are E2E points of interaction without space for doing stupid namings.

/// CommandHandler trait defines a common interface for executing clap operations in a command pattern implementation. It's designed to work with Sea ORM database connections and provides a standardized way to handle various database commands for the commands. Database connection is required as it's only injectable dependency that we have currently, fruther dependencies will be required to be passed as dependency injection container.
pub trait CommandHandler: Command
{
    fn handle(&self, db: &DatabaseConnection) -> ();
}
