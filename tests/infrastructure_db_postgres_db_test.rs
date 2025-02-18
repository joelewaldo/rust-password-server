use rust_password_server::bounded_context::infrastructure::db::postgres_db::*;
use rust_password_server::bounded_context::domain::{password::Password, password_db::PasswordDb, password_db::SortBy};
use sqlx::{query, Executor, PgPool};
use uuid::Uuid;
use chrono::Utc;
use tokio::sync::OnceCell;
use rust_password_server::bounded_context::infrastructure::config::app_config;
use chrono::{DateTime, Duration};

static DB_INSTANCE: OnceCell<tokio::sync::Mutex<Database>> = OnceCell::const_new();

async fn get_test_database() -> &'static tokio::sync::Mutex<Database> {
    DB_INSTANCE.get_or_init(|| async {
        let config = app_config::load_config();
        let base_db_url = &config.test_db_url;
        tokio::sync::Mutex::new(Database::new(base_db_url, 1).await.expect("Failed to create test DB"))
    }).await
}

async fn setup_db(database: &Database) {
    let mut conn = database.get_connection().await.expect("Failed to get DB connection");

    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS passwords (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            service TEXT NOT NULL,
            nonce TEXT NOT NULL,
            cipher TEXT NOT NULL,
            created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
            updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
        )
        "#,
    )
    .await
    .expect("Failed to setup the database");

    conn.execute("TRUNCATE TABLE passwords CASCADE")
        .await
        .expect("Failed to clean test database");
}

fn assert_datetime_approx_eq(left: DateTime<Utc>, right: DateTime<Utc>, tolerance: Duration) {
    let diff = (left - right).abs();
    assert!(diff <= tolerance, "Timestamps differ by more than the allowed tolerance");
}

#[tokio::test]
async fn test_save_and_get_password() {
    let mut database = get_test_database().await.lock().await;
    setup_db(&database).await;

    let test_password = Password {
        id: Uuid::new_v4(),
        service: "test_service".to_string(),
        nonce: "test_nonce".to_string(),
        cipher: "test_cipher".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    database.save(test_password.clone()).await.expect("Failed to save password");

    let retrieved_password = database
        .get_by_id(test_password.id)
        .await
        .expect("Failed to retrieve password");

    assert_eq!(test_password.id, retrieved_password.id);
    assert_eq!(test_password.service, retrieved_password.service);
    assert_eq!(test_password.nonce, retrieved_password.nonce);
    assert_eq!(test_password.cipher, retrieved_password.cipher);

    assert!(retrieved_password.created_at <= Utc::now());
    assert!(retrieved_password.updated_at <= Utc::now());
}

#[tokio::test]
async fn test_get_by_id() {
    let mut database = get_test_database().await.lock().await;
    setup_db(&database).await;

    let test_password = Password {
        id: Uuid::new_v4(),
        service: "test_service".to_string(),
        nonce: "test_nonce".to_string(),
        cipher: "test_cipher".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    database
        .save(test_password.clone())
        .await
        .expect("Failed to save password");

    let retrieved_password = database
        .get_by_id(test_password.id)
        .await
        .expect("Failed to retrieve password");

    assert_eq!(test_password.id, retrieved_password.id);
    assert_eq!(test_password.service, retrieved_password.service);
    assert_eq!(test_password.nonce, retrieved_password.nonce);
    assert_eq!(test_password.cipher, retrieved_password.cipher);
    
    let tolerance = Duration::milliseconds(1);
    assert_datetime_approx_eq(test_password.created_at, retrieved_password.created_at, tolerance);
    assert_datetime_approx_eq(test_password.updated_at, retrieved_password.updated_at, tolerance);

    let non_existent_id = Uuid::new_v4();
    let result = database.get_by_id(non_existent_id).await;

    assert!(result.is_err(), "Expected an error for non-existent password");
}

#[tokio::test]
async fn test_delete_password() {
    let mut database = get_test_database().await.lock().await;
    setup_db(&database).await;

    let test_password = Password {
        id: Uuid::new_v4(),
        service: "test_service".to_string(),
        nonce: "test_nonce".to_string(),
        cipher: "test_cipher".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    database.save(test_password.clone()).await.expect("Failed to save password");

    database
        .delete(test_password.id)
        .await
        .expect("Failed to delete password");

    let result = database.get_by_id(test_password.id).await;
    assert!(result.is_err(), "Password should be deleted");
}

#[tokio::test]
async fn test_search_by_service() {
    let mut database = get_test_database().await.lock().await;
    setup_db(&database).await;

    let passwords = vec![
        Password {
            id: Uuid::new_v4(),
            service: "Gmail Account".to_string(),
            nonce: "n1".to_string(),
            cipher: "c1".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        Password {
            id: Uuid::new_v4(),
            service: "GitHub Login".to_string(),
            nonce: "n2".to_string(),
            cipher: "c2".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
        Password {
            id: Uuid::new_v4(),
            service: "Work Email".to_string(),
            nonce: "n3".to_string(),
            cipher: "c3".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
    ];

    for pw in &passwords {
        database.save(pw.clone()).await.expect("Failed to save password");
    }

    // Test pagination
    let results = database
        .search_by_service("mail", 1, 10)
        .await
        .expect("Search failed");

    assert_eq!(results.len(), 2);
    let services: Vec<String> = results.into_iter().map(|pw| pw.service).collect();
    assert!(services.contains(&"Gmail Account".to_string()));
    assert!(services.contains(&"Work Email".to_string()));

    // Test exact match
    let exact_results = database
        .search_by_service("GitHub Login", 1, 10)
        .await
        .expect("Search failed");
    assert_eq!(exact_results.len(), 1);
}

#[tokio::test]
async fn test_list_sorted_created_at_asc() {
    let mut database = get_test_database().await.lock().await;
    setup_db(&database).await;

    let now = Utc::now();
    let passwords = vec![
        Password {
            id: Uuid::new_v4(),
            service: "Oldest".to_string(),
            nonce: "n1".to_string(),
            cipher: "c1".to_string(),
            created_at: now - Duration::hours(2),
            updated_at: now,
        },
        Password {
            id: Uuid::new_v4(),
            service: "Middle".to_string(),
            nonce: "n2".to_string(),
            cipher: "c2".to_string(),
            created_at: now - Duration::hours(1),
            updated_at: now,
        },
        Password {
            id: Uuid::new_v4(),
            service: "Newest".to_string(),
            nonce: "n3".to_string(),
            cipher: "c3".to_string(),
            created_at: now,
            updated_at: now,
        },
    ];

    for pw in &passwords {
        database.save(pw.clone()).await.expect("Failed to save password");
    }

    let results = database.list_sorted(&SortBy::CreatedAtAsc, 1, 10)
        .await
        .expect("Sorting failed");

    assert_eq!(results.len(), 3);
    assert_eq!(results[0].service, "Oldest");
    assert_eq!(results[1].service, "Middle");
    assert_eq!(results[2].service, "Newest");
}

#[tokio::test]
async fn test_list_sorted_updated_at_desc() {
    let mut database = get_test_database().await.lock().await;
    setup_db(&database).await;

    let now = Utc::now();
    let passwords = vec![
        Password {
            id: Uuid::new_v4(),
            service: "Updated Recently".to_string(),
            nonce: "n1".to_string(),
            cipher: "c1".to_string(),
            created_at: now - Duration::hours(2),
            updated_at: now,
        },
        Password {
            id: Uuid::new_v4(),
            service: "Updated Long Ago".to_string(),
            nonce: "n2".to_string(),
            cipher: "c2".to_string(),
            created_at: now - Duration::hours(3),
            updated_at: now - Duration::hours(1),
        },
    ];

    for pw in &passwords {
        database.save(pw.clone()).await.expect("Failed to save password");
    }

    let results = database.list_sorted(&SortBy::UpdatedAtDesc, 1, 10)
        .await
        .expect("Sorting failed");

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].service, "Updated Recently");
    assert_eq!(results[1].service, "Updated Long Ago");
}

#[tokio::test]
async fn test_empty_search_results() {
    let mut database = get_test_database().await.lock().await;
    setup_db(&database).await;

    let results = database
        .search_by_service("nonexistent", 1, 10)
        .await
        .expect("Search failed");
    
    assert!(results.is_empty());
}

#[tokio::test]
async fn test_all_sorting_variants() {
    let mut database = get_test_database().await.lock().await;
    setup_db(&database).await;

    let now = Utc::now();
    let passwords = vec![
        Password {
            id: Uuid::new_v4(),
            service: "A".to_string(),
            nonce: "n1".to_string(),
            cipher: "c1".to_string(),
            created_at: now - Duration::hours(2),
            updated_at: now - Duration::hours(1),
        },
        Password {
            id: Uuid::new_v4(),
            service: "B".to_string(),
            nonce: "n2".to_string(),
            cipher: "c2".to_string(),
            created_at: now - Duration::hours(1),
            updated_at: now,
        },
    ];

    for pw in &passwords {
        database.save(pw.clone()).await.expect("Failed to save password");
    }

    let test_cases = vec![
        (SortBy::CreatedAtAsc, vec!["A", "B"]),
        (SortBy::CreatedAtDesc, vec!["B", "A"]),
        (SortBy::UpdatedAtAsc, vec!["A", "B"]),
        (SortBy::UpdatedAtDesc, vec!["B", "A"]),
    ];

    for (sort_by, expected_order) in test_cases {
        let results = database.list_sorted(&sort_by, 1, 10)
            .await
            .expect("Sorting failed");
        
        let services: Vec<&str> = results.iter().map(|pw| pw.service.as_str()).collect();
        assert_eq!(services, expected_order, "Failed for {:?}", sort_by);
    }
}

#[tokio::test]
async fn test_search_by_service_pagination() {
    let mut database = get_test_database().await.lock().await;
    setup_db(&database).await;

    for i in 0..10 {
        let password = Password {
            id: Uuid::new_v4(),
            service: format!("Service {}", i),
            nonce: format!("n{}", i),
            cipher: format!("c{}", i),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        database.save(password).await.expect("Failed to save password");
    }

    let page1 = database
        .search_by_service("Service", 1, 5)
        .await
        .expect("Search failed");
    assert_eq!(page1.len(), 5);

    let page2 = database
        .search_by_service("Service", 2, 5)
        .await
        .expect("Search failed");
    assert_eq!(page2.len(), 5);

    let page1_services: Vec<String> = page1.into_iter().map(|pw| pw.service).collect();
    let page2_services: Vec<String> = page2.into_iter().map(|pw| pw.service).collect();
    for service in page1_services {
        assert!(!page2_services.contains(&service));
    }

    let page3 = database
        .search_by_service("Service", 3, 5)
        .await
        .expect("Search failed");
    assert!(page3.is_empty());
}