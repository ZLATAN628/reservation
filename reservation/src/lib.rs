use async_trait::async_trait;

mod error;
mod manager;

pub type ReservationId = String;
pub type ResourceId = String;
pub type UserId = String;

#[derive(Debug)]
pub struct ReservationManager {
    pool: sqlx::PgPool,
}

#[async_trait]
pub trait Rsvp {
    /// make a resvervation
    async fn reserve(
        &self,
        mut rsvp: abi::Reservation,
    ) -> Result<abi::Reservation, error::ReservationError>;

    // /// change reservation status (if current status is pending, change it to confirmed)
    // async fn change_status(
    //     &self,
    //     id: ReservationId,
    // ) -> Result<abi::Reservation, error::ReservationError>;

    // /// update note
    // async fn update_note(
    //     &self,
    //     id: ReservationId,
    //     note: String,
    // ) -> Result<abi::Reservation, error::ReservationError>;

    // /// delete reservation
    // async fn delete_reservation(&self, id: ReservationId) -> Result<(), error::ReservationError>;

    // /// get reservation by id
    // async fn get_reservation(
    //     &self,
    //     id: ReservationId,
    // ) -> Result<abi::Reservation, error::ReservationError>;

    // /// query reservation
    // async fn query(
    //     &self,
    //     query: abi::ReservationQuery,
    // ) -> Result<Vec<abi::Reservation>, error::ReservationError>;
}
