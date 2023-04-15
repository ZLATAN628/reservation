-- Add down migration script here
DROP TABLE rsvp.reservation_changes;
DROP TYPE rsvp.reservation_update_type;
DROP TRIGGER reservations_trigger ON rsvp.reservation;
DROP FUNCTION rsvp.reservations_trigger();
