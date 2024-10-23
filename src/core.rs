use miette::Error;
use sea_orm::DatabaseConnection;

pub trait CommandHandler
{
    fn handle(&self, db: &DatabaseConnection) -> ();
}
