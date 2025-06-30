use domain::rental_domain::RentalRespository;
use domain_core::rental::RentalBuilder;
use domain_core::utils::Clock;
use garde::Validate;

use crate::app_error::{AppError, AppResult};
use crate::commands::rental_commands::{CreateRentalCommand, CreateRentalRes};
use crate::{AppUseCase, Outcome};

pub async fn create_rental_request(
    repo:&impl RentalRespository,
    time:&impl Clock,
    data:CreateRentalCommand,
) -> AppResult<Outcome<CreateRentalRes>> {

    let builder = RentalBuilder::default();

    let builder = builder
        .venue_id(data.venue_id)
        .organizer_id(data.organizer_id)
        .start_time(data.start_time)
        .end_time(data.end_time)
        .activity_type(data.activity_type)
        .request_comments(data.request_comments)
        .updatetime(time.now())
        .createtime(time.now());

    let rental = builder.build()
        .map_err(|e|{
            AppError::CreateEntityFailed { 
                entity_type:"vental".to_string(),
                message: e.to_string(), 
                source: e 
            }
        })?;

    rental.validate().map_err(|e|{
        AppError::EntityInvalid { 
            entity_type: "user".to_string(),
            cause: e.to_string()
        }
    })?;
   

    let rental = repo.create_rental_request(rental).await?;

    let id = rental.id().ok_or(AppError::IdInexisted("user".to_string()))?;

    let res = CreateRentalRes {
        id,
        venue_id:rental.venue_id().clone(),
        organizer_id:rental.organizer_id().clone(),
        start_time:rental.start_time().format("%Y-%m-%d %H:%M:%S").to_string(),
        end_time:rental.end_time().format("%Y-%m-%d %H:%M:%S").to_string(),
    };

    Ok(Outcome::new(res,AppUseCase::CreateRentalRequest))
}
