use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, ColumnTrait, QueryFilter};
use crate::domain::user::user_entity::{Entity as User, Model as UserModel, ActiveModel as UserActiveModel};
use crate::domain::user::user_entity;
use crate::domain::role::role_entity::{Entity as Role, Model as RoleModel};
use crate::domain::group::group_entity::{Entity as Group, Model as GroupModel};

/// Service layer for user management operations.
/// 
/// This service provides business logic for user-related operations including
/// CRUD operations, user validation, and relationship management with roles and groups.
/// It acts as an intermediary between the controller layer and the data access layer.
/// 
/// # Features
/// 
/// * User CRUD operations with comprehensive validation
/// * Email and display ID uniqueness validation
/// * Role and group existence validation
/// * Search functionality by name, email, and display ID
/// 
/// # Examples
/// 
/// ```rust
/// use sea_orm::DatabaseConnection;
/// 
/// let db = get_database_connection().await;
/// 
/// // Get all users
/// let users = UserService::get_all_users(&db).await?;
/// 
/// // Create a new user
/// let new_user = UserService::create_user(
///     &db,
///     "John Doe".to_string(),
///     "password123".to_string(),
///     "john@example.com".to_string(),
///     false,
///     "john_doe".to_string(),
///     0,
///     true,
///     1,
///     1,
///     chrono::Utc::now().naive_utc().date(),
/// ).await?;
/// ```
pub struct UserService;

impl UserService {
    /// Retrieves all users from the database.
    /// 
    /// This method fetches all user records from the database without any filtering.
    /// 
    /// # Arguments
    /// 
    /// * `db` - A reference to the database connection
    /// 
    /// # Returns
    /// 
    /// * `Ok(Vec<UserModel>)` - A vector containing all user models
    /// * `Err(sea_orm::DbErr)` - Database error if the operation fails
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let users = UserService::get_all_users(&db).await?;
    /// println!("Found {} users", users.len());
    /// ```
    pub async fn get_all_users(db: &DatabaseConnection) -> Result<Vec<UserModel>, sea_orm::DbErr> {
        User::find().all(db).await
    }

    /// Retrieves a user by their unique identifier.
    /// 
    /// This method fetches a single user record from the database using the provided ID.
    /// 
    /// # Arguments
    /// 
    /// * `db` - A reference to the database connection
    /// * `id` - The unique identifier of the user to retrieve
    /// 
    /// # Returns
    /// 
    /// * `Ok(Some(UserModel))` - The user model if found
    /// * `Ok(None)` - If no user with the given ID exists
    /// * `Err(sea_orm::DbErr)` - Database error if the operation fails
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// match UserService::get_user_by_id(&db, 1).await? {
    ///     Some(user) => println!("Found user: {}", user.name),
    ///     None => println!("User not found"),
    /// }
    /// ```
    pub async fn get_user_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<UserModel>, sea_orm::DbErr> {
        User::find_by_id(id).one(db).await
    }

    /// Retrieves a user by their email address.
    /// 
    /// This method fetches a single user record from the database using the provided email.
    /// Since email addresses are unique, this method returns at most one user.
    /// 
    /// # Arguments
    /// 
    /// * `db` - A reference to the database connection
    /// * `email` - The email address of the user to retrieve
    /// 
    /// # Returns
    /// 
    /// * `Ok(Some(UserModel))` - The user model if found
    /// * `Ok(None)` - If no user with the given email exists
    /// * `Err(sea_orm::DbErr)` - Database error if the operation fails
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// match UserService::get_user_by_email(&db, "john@example.com").await? {
    ///     Some(user) => println!("Found user: {}", user.name),
    ///     None => println!("User not found"),
    /// }
    /// ```
    pub async fn get_user_by_email(db: &DatabaseConnection, email: &str) -> Result<Option<UserModel>, sea_orm::DbErr> {
        User::find().filter(user_entity::Column::Email.eq(email)).one(db).await
    }

    /// Searches for users by name using partial matching.
    /// 
    /// This method performs a case-sensitive search for users whose names contain
    /// the provided search term. It returns all matching users.
    /// 
    /// # Arguments
    /// 
    /// * `db` - A reference to the database connection
    /// * `name` - The search term to match against user names
    /// 
    /// # Returns
    /// 
    /// * `Ok(Vec<UserModel>)` - A vector containing all matching user models
    /// * `Err(sea_orm::DbErr)` - Database error if the operation fails
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let john_users = UserService::get_user_by_name(&db, "John").await?;
    /// for user in john_users {
    ///     println!("Found user: {}", user.name);
    /// }
    /// ```
    pub async fn get_user_by_name(db: &DatabaseConnection, name: &str) -> Result<Vec<UserModel>, sea_orm::DbErr> {
        User::find().filter(user_entity::Column::Name.contains(name)).all(db).await
    }

    /// Retrieves a user by their display ID.
    /// 
    /// This method fetches a single user record from the database using the provided display ID.
    /// Since display IDs are unique, this method returns at most one user.
    /// 
    /// # Arguments
    /// 
    /// * `db` - A reference to the database connection
    /// * `user_display_id` - The display ID of the user to retrieve
    /// 
    /// # Returns
    /// 
    /// * `Ok(Some(UserModel))` - The user model if found
    /// * `Ok(None)` - If no user with the given display ID exists
    /// * `Err(sea_orm::DbErr)` - Database error if the operation fails
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// match UserService::get_user_by_display_id(&db, "john_doe").await? {
    ///     Some(user) => println!("Found user: {}", user.name),
    ///     None => println!("User not found"),
    /// }
    /// ```
    pub async fn get_user_by_display_id(db: &DatabaseConnection, user_display_id: &str) -> Result<Option<UserModel>, sea_orm::DbErr> {
        User::find().filter(user_entity::Column::UserDisplayId.eq(user_display_id)).one(db).await
    }

    /// Creates a new user in the database with comprehensive validation.
    /// 
    /// This method creates a new user with all the provided information. It includes
    /// extensive validation to ensure data integrity and prevent duplicates.
    /// 
    /// # Arguments
    /// 
    /// * `db` - A reference to the database connection
    /// * `name` - The user's full name
    /// * `password` - The user's password (should be hashed before calling this method)
    /// * `email` - The user's email address (must be unique)
    /// * `email_confirmed` - Whether the email has been confirmed
    /// * `user_display_id` - A unique display identifier for the user
    /// * `balance` - The user's current balance
    /// * `is_active` - Whether the user account is active
    /// * `role_id` - The ID of the user's role (must reference an existing role)
    /// * `group_id` - The ID of the user's group (must reference an existing group)
    /// * `created_date` - The date when the user was created
    /// 
    /// # Returns
    /// 
    /// * `Ok(UserModel)` - The newly created user model
    /// * `Err(sea_orm::DbErr)` - Database error if the operation fails, including
    ///   custom errors for validation failures
    /// 
    /// # Validation Rules
    /// 
    /// This function performs the following validations:
    /// * Email must be unique across all users
    /// * Display ID must be unique across all users
    /// * Name must be unique across all users
    /// * Role ID must reference an existing role
    /// * Group ID must reference an existing group
    /// 
    /// # Errors
    /// 
    /// This function will return an error if:
    /// * Email already exists
    /// * Display ID already exists
    /// * Name already exists
    /// * Role does not exist
    /// * Group does not exist
    /// * Database operation fails
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// match UserService::create_user(
    ///     &db,
    ///     "John Doe".to_string(),
    ///     "hashed_password".to_string(),
    ///     "john@example.com".to_string(),
    ///     false,
    ///     "john_doe".to_string(),
    ///     0,
    ///     true,
    ///     1, // role_id
    ///     1, // group_id
    ///     chrono::Utc::now().naive_utc().date(),
    /// ).await {
    ///     Ok(user) => println!("Created user: {}", user.name),
    ///     Err(e) => println!("Failed to create user: {}", e),
    /// }
    /// ```
    pub async fn create_user(
        db: &DatabaseConnection, 
        name: String, 
        password: String, 
        email: String, 
        email_confirmed: bool,
        user_display_id: String,
        balance: i32,
        is_active: bool,
        role_id: i32,
        group_id: i32,
        created_date: chrono::NaiveDate,
    ) -> Result<UserModel, sea_orm::DbErr> {
        // Check if email already exists
        let found_user = User::find().filter(user_entity::Column::Email.eq(email.clone())).one(db).await?;
        if found_user.is_some() {
            return Err(sea_orm::DbErr::Custom("user with this email already exists".to_string()));
        }

        // Check if user_display_id already exists
        let found_display_id = User::find().filter(user_entity::Column::UserDisplayId.eq(user_display_id.clone())).one(db).await?;
        if found_display_id.is_some() {
            return Err(sea_orm::DbErr::Custom("user with this display ID already exists".to_string()));
        }

        // Check if name already exists
        let found_name = User::find().filter(user_entity::Column::Name.eq(name.clone())).one(db).await?;
        if found_name.is_some() {
            return Err(sea_orm::DbErr::Custom("user with this name already exists".to_string()));
        }

        // Check if role exists
        let found_role = Role::find_by_id(role_id).one(db).await?;
        if found_role.is_none() {
            return Err(sea_orm::DbErr::Custom("role does not exist".to_string()));
        }

        // Check if group exists
        let found_group = Group::find_by_id(group_id).one(db).await?;
        if found_group.is_none() {
            return Err(sea_orm::DbErr::Custom("group does not exist".to_string()));
        }

        let active_model = UserActiveModel {
            name: Set(name),
            password: Set(password),
            email: Set(email),
            email_confirmed: Set(email_confirmed),
            user_display_id: Set(user_display_id),
            balance: Set(balance),
            is_active: Set(is_active),
            role_id: Set(role_id),
            group_id: Set(group_id),
            created_date: Set(created_date),
            ..Default::default()
        };
        active_model.insert(db).await
    }

    /// Updates an existing user in the database with partial updates and validation.
    /// 
    /// This method updates specific fields of an existing user. Only the fields that
    /// are provided (not None) will be updated. It includes validation to ensure
    /// data integrity and prevent duplicates.
    /// 
    /// # Arguments
    /// 
    /// * `db` - A reference to the database connection
    /// * `id` - The unique identifier of the user to update
    /// * `name` - Optional new name for the user
    /// * `password` - Optional new password for the user
    /// * `email` - Optional new email address (must be unique if changed)
    /// * `email_confirmed` - Optional email confirmation status
    /// * `user_display_id` - Optional new display ID (must be unique if changed)
    /// * `balance` - Optional new balance
    /// * `is_active` - Optional active status
    /// * `role_id` - Optional new role ID (must reference an existing role if changed)
    /// * `group_id` - Optional new group ID (must reference an existing group if changed)
    /// 
    /// # Returns
    /// 
    /// * `Ok(Some(UserModel))` - The updated user model if found and updated
    /// * `Ok(None)` - If no user with the given ID exists
    /// * `Err(sea_orm::DbErr)` - Database error if the operation fails, including
    ///   custom errors for validation failures
    /// 
    /// # Validation Rules
    /// 
    /// This function performs the following validations for changed fields:
    /// * Email must be unique across all users (only if email is being changed)
    /// * Display ID must be unique across all users (only if display ID is being changed)
    /// * Name must be unique across all users (only if name is being changed)
    /// * Role ID must reference an existing role (only if role ID is being changed)
    /// * Group ID must reference an existing group (only if group ID is being changed)
    /// 
    /// # Errors
    /// 
    /// This function will return an error if:
    /// * Email already exists (only if email is being changed)
    /// * Display ID already exists (only if display ID is being changed)
    /// * Name already exists (only if name is being changed)
    /// * Role does not exist (only if role ID is being changed)
    /// * Group does not exist (only if group ID is being changed)
    /// * Database operation fails
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// match UserService::update_user(
    ///     &db,
    ///     1,
    ///     Some("Jane Doe".to_string()),
    ///     None, // password unchanged
    ///     Some("jane@example.com".to_string()),
    ///     Some(true),
    ///     None, // display_id unchanged
    ///     Some(100),
    ///     None, // is_active unchanged
    ///     None, // role_id unchanged
    ///     None, // group_id unchanged
    /// ).await {
    ///     Ok(Some(user)) => println!("Updated user: {}", user.name),
    ///     Ok(None) => println!("User not found"),
    ///     Err(e) => println!("Failed to update user: {}", e),
    /// }
    /// ```
    pub async fn update_user(
        db: &DatabaseConnection, 
        id: i32, 
        name: Option<String>,
        password: Option<String>,
        email: Option<String>,
        email_confirmed: Option<bool>,
        user_display_id: Option<String>,
        balance: Option<i32>,
        is_active: Option<bool>,
        role_id: Option<i32>,
        group_id: Option<i32>,
    ) -> Result<Option<UserModel>, sea_orm::DbErr> {
        if let Some(user) = User::find_by_id(id).one(db).await? {
            // Extract values before moving user
            let current_name = user.name.clone();
            let current_email = user.email.clone();
            let current_user_display_id = user.user_display_id.clone();
            let current_role_id = user.role_id;
            let current_group_id = user.group_id;
            
            let mut active_model: UserActiveModel = user.into();
            
            if let Some(name) = name {
                // Check if name already exists for another user (only if name is being changed)
                if name != current_name {
                    let found_name = User::find()
                        .filter(user_entity::Column::Name.eq(name.clone()))
                        .filter(user_entity::Column::Id.ne(id))
                        .one(db).await?;
                    if found_name.is_some() {
                        return Err(sea_orm::DbErr::Custom("user with this name already exists".to_string()));
                    }
                }
                active_model.name = Set(name);
            }
            if let Some(password) = password {
                active_model.password = Set(password);
            }
            if let Some(email) = email {
                // Check if email already exists for another user (only if email is being changed)
                if email != current_email {
                    let found_user = User::find()
                        .filter(user_entity::Column::Email.eq(email.clone()))
                        .filter(user_entity::Column::Id.ne(id))
                        .one(db).await?;
                    if found_user.is_some() {
                        return Err(sea_orm::DbErr::Custom("user with this email already exists".to_string()));
                    }
                }
                active_model.email = Set(email);
            }
            if let Some(email_confirmed) = email_confirmed {
                active_model.email_confirmed = Set(email_confirmed);
            }
            if let Some(user_display_id) = user_display_id {
                // Check if user_display_id already exists for another user (only if display_id is being changed)
                if user_display_id != current_user_display_id {
                    let found_display_id = User::find()
                        .filter(user_entity::Column::UserDisplayId.eq(user_display_id.clone()))
                        .filter(user_entity::Column::Id.ne(id))
                        .one(db).await?;
                    if found_display_id.is_some() {
                        return Err(sea_orm::DbErr::Custom("user with this display ID already exists".to_string()));
                    }
                }
                active_model.user_display_id = Set(user_display_id);
            }
            if let Some(balance) = balance {
                active_model.balance = Set(balance);
            }
            if let Some(is_active) = is_active {
                active_model.is_active = Set(is_active);
            }
            if let Some(role_id) = role_id {
                // Check if role exists (only if role_id is being changed)
                if role_id != current_role_id {
                    let found_role = Role::find_by_id(role_id).one(db).await?;
                    if found_role.is_none() {
                        return Err(sea_orm::DbErr::Custom("role does not exist".to_string()));
                    }
                }
                active_model.role_id = Set(role_id);
            }
            if let Some(group_id) = group_id {
                // Check if group exists (only if group_id is being changed)
                if group_id != current_group_id {
                    let found_group = Group::find_by_id(group_id).one(db).await?;
                    if found_group.is_none() {
                        return Err(sea_orm::DbErr::Custom("group does not exist".to_string()));
                    }
                }
                active_model.group_id = Set(group_id);
            }
            
            let updated = active_model.update(db).await?;
            Ok(Some(updated))
        } else {
            Ok(None)
        }
    }

    /// Deletes a user from the database by their ID.
    /// 
    /// This method permanently removes a user record from the database.
    /// 
    /// # Arguments
    /// 
    /// * `db` - A reference to the database connection
    /// * `id` - The unique identifier of the user to delete
    /// 
    /// # Returns
    /// 
    /// * `Ok(true)` - If the user was successfully deleted
    /// * `Ok(false)` - If no user with the given ID exists
    /// * `Err(sea_orm::DbErr)` - Database error if the operation fails
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// match UserService::delete_user(&db, 1).await {
    ///     Ok(true) => println!("User deleted successfully"),
    ///     Ok(false) => println!("User not found"),
    ///     Err(e) => println!("Failed to delete user: {}", e),
    /// }
    /// ```
    pub async fn delete_user(db: &DatabaseConnection, id: i32) -> Result<bool, sea_orm::DbErr> {
        let result = User::delete_by_id(id).exec(db).await?;
        Ok(result.rows_affected > 0)
    }
}
