# Common Crate

Shared models and database logic for the Merco API.

## Structure

```
common/
├── src/
│   ├── models/
│   │   ├── auth/              # Authentication-related models
│   │   │   ├── user.rs        # User model + SQL queries
│   │   │   ├── session.rs     # Session model + SQL queries
│   │   │   ├── role.rs        # Role & Permission models + SQL queries
│   │   │   ├── otp.rs         # OTP code model + SQL queries
│   │   │   ├── magic_link.rs  # Magic link model + SQL queries
│   │   │   └── password_reset.rs # Password reset model + SQL queries
│   │   ├── project/           # Project-related models
│   │   │   └── project.rs     # Project model + SQL queries
│   │   └── webhook/           # Webhook-related models
│   │       └── webhook.rs     # Webhook model + SQL queries
│   └── database/              # Database utilities
│       ├── connection.rs      # Connection pool helpers
│       └── migrations/        # Migration runner
└── migrations/                # SQL migration files
    └── 001_initial_schema.sql
```

## Usage

### Running Migrations

```rust
use common::database::{create_pool, run_migrations};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = create_pool().await?;
    run_migrations(&pool).await?;
    Ok(())
}
```

### Using Models with SQL Queries

Each model has associated SQL query methods:

```rust
use common::models::auth::User;
use sqlx::PgPool;

// Create a user
let user = User {
    id: uuid::Uuid::new_v4(),
    project_id: project_id,
    email: "user@example.com".to_string(),
    // ... other fields
};

let created_user = User::create(&pool, &user).await?;

// Find user by email
let user = User::find_by_email(&pool, project_id, "user@example.com").await?;

// Update user
let updated_user = User::update(&pool, &user).await?;

// List users with pagination
let users = User::list(&pool, project_id, 10, 0).await?;
```

### Example: Using Auth Models

```rust
use common::models::auth::{User, Session, Role, user_role};

// Create user
let user = User::create(&pool, &user_data).await?;

// Create session
let session = Session::create(&pool, &session_data).await?;

// Assign role to user
user_role::assign(&pool, user.id, role.id).await?;

// Check if user has role
let has_admin = user_role::has_role(&pool, user.id, "admin").await?;
```

### Example: Using Project Models

```rust
use common::models::project::Project;

// Find project by API key
let project = Project::find_by_api_key(&pool, "api_key_here").await?;

// Update project settings
project.settings = serde_json::json!({"new_setting": "value"});
let updated = Project::update(&pool, &project).await?;
```

## Migrations

Migration files should be named with a numeric prefix:
- `001_initial_schema.sql`
- `002_add_indexes.sql`
- etc.

They will be run in alphabetical order.
