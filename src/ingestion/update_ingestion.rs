use crate::db;
use crate::ingestion::RouteOfAdministrationClassification;
use chrono::NaiveDateTime;
use clap::Args;
use sea_orm::ActiveModelTrait;
use sea_orm::EntityTrait;
use sea_orm::Set;

#[derive(Args)]
pub struct UpdateIngestion {
    #[arg(short, long)]
    id: i32,
    #[arg(short, long)]
    substance_name: Option<String>,
    #[arg(short, long)]
    route_of_administration: Option<RouteOfAdministrationClassification>,
    #[arg(short, long)]
    dosage: Option<f32>,
    #[arg(short, long)]
    notes: Option<String>,
    #[arg(short, long)]
    ingested_at: Option<NaiveDateTime>,
}

impl UpdateIngestion {
    pub async fn handle(&self, db: &sea_orm::DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
        let ingestion = db::ingestion::Entity::find_by_id(self.id)
            .one(db)
            .await?
            .ok_or("Ingestion not found")?;

        let mut active_model: db::ingestion::ActiveModel = ingestion.into();

        if let Some(substance_name) = &self.substance_name {
            active_model.substance_name = Set(substance_name.to_owned());
        }
        if let Some(route_of_administration) = &self.route_of_administration {
            active_model.route_of_administration = Set(route_of_administration.serialize());
        }
        if let Some(dosage) = self.dosage {
            active_model.dosage = Set(dosage);
        }
        if let Some(notes) = &self.notes {
            active_model.notes = Set(Some(notes.to_owned()));
        }
        if let Some(ingested_at) = self.ingested_at {
            active_model.ingested_at = Set(ingested_at.and_utc());
        }

        active_model.update(db).await?;

        println!("Ingestion updated successfully");
        Ok(())
    }
}
