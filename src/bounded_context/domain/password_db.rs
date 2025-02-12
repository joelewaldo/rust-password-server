use super::password::Password;
use uuid::Uuid;

pub trait PasswordDb {
    fn save(&mut self, password: Password);
    fn get_by_id(&mut self, id: Uuid) -> Result<Password, Box<dyn std::error::Error>>;
}