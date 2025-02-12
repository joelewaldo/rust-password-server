pub mod create_password_builder {
    use crate::bounded_context::application::create_password::CreatePassword;
    use crate::bounded_context::infrastructure::config::app_config;
    use crate::bounded_context::infrastructure::db::postgres_db::Database;

    pub async fn build() -> CreatePassword {
        let app_config = app_config::load_config();
        let password_db = Database::new(&app_config.db_url, 1).await.expect(
            "Failed to connect to password db"
        );

        CreatePassword::new(Box::new(password_db))
    }
}