use domain::rental_domain::rental_dto::RentalRes;
use domain::rental_domain::RentalRespository;

use crate::app_error::AppResult;
use crate::{AppUseCase, Outcome};

pub async fn get_rental_requests(
    repo: &impl RentalRespository,
    lessor_id: i64,
) -> AppResult<Outcome<Vec<RentalRes>>> {
    let rentals = repo.get_rental_lists(lessor_id).await?;
    Ok(Outcome::new(
        rentals,
        AppUseCase::GetData("Get rental result".to_string()),
    ))
}
