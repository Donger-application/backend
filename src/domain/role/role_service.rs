use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, ColumnTrait, QueryFilter};
use crate::domain::role::role_entity::{Entity as Role, Model as RoleModel, ActiveModel as RoleActiveModel};
use crate::domain::role::role_entity;

/// Service layer for role management operations.
/// 
/// This service provides business logic for role-related operations including
/// CRUD operations and role validation. It acts as an intermediary between
/// the controller layer and the data access layer.
/// 
/// # Examples
/// 
/// ```rust
/// use sea_orm::DatabaseConnection;
/// 
/// let db = get_database_connection().await;
/// 
/// // Get all roles
/// let roles = RoleService::get_all_roles(&db).await?;
/// 
/// // Create a new role
/// let new_role = RoleService::create_role(&db, "admin".to_string()).await?;
/// ```
pub struct RoleService;

impl RoleService {
    /// Retrieves all roles from the database.
    /// 
    /// This method fetches all role records from the database without any filtering.
    /// 
    /// # Arguments
    /// 
    /// * `db` - A reference to the database connection
    /// 
    /// # Returns
    /// 
    /// * `Ok(Vec<RoleModel>)` - A vector containing all role models
    /// * `Err(sea_orm::DbErr)` - Database error if the operation fails
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let roles = RoleService::get_all_roles(&db).await?;
    /// println!("Found {} roles", roles.len());
    /// ```
    pub async fn get_all_roles(db: &DatabaseConnection) -> Result<Vec<RoleModel>, sea_orm::DbErr> {
        Role::find().all(db).await
    }

    /// Retrieves a role by its unique identifier.
    /// 
    /// This method fetches a single role record from the database using the provided ID.
    /// 
    /// # Arguments
    /// 
    /// * `db` - A reference to the database connection
    /// * `id` - The unique identifier of the role to retrieve
    /// 
    /// # Returns
    /// 
    /// * `Ok(Some(RoleModel))` - The role model if found
    /// * `Ok(None)` - If no role with the given ID exists
    /// * `Err(sea_orm::DbErr)` - Database error if the operation fails
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// match RoleService::get_role_by_id(&db, 1).await? {
    ///     Some(role) => println!("Found role: {}", role.name),
    ///     None => println!("Role not found"),
    /// }
    /// ```
    pub async fn get_role_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<RoleModel>, sea_orm::DbErr> {
        Role::find_by_id(id).one(db).await
    }

    /// Searches for roles by name using partial matching.
    /// 
    /// This method performs a case-sensitive search for roles whose names contain
    /// the provided search term. It returns all matching roles.
    /// 
    /// # Arguments
    /// 
    /// * `db` - A reference to the database connection
    /// * `name` - The search term to match against role names
    /// 
    /// # Returns
    /// 
    /// * `Ok(Vec<RoleModel>)` - A vector containing all matching role models
    /// * `Err(sea_orm::DbErr)` - Database error if the operation fails
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let admin_roles = RoleService::get_role_by_name(&db, "admin").await?;
    /// for role in admin_roles {
    ///     println!("Found admin role: {}", role.name);
    /// }
    /// ```
    pub async fn get_role_by_name(db: &DatabaseConnection, name: &str) -> Result<Vec<RoleModel>, sea_orm::DbErr> {
        Role::find().filter(role_entity::Column::Name.contains(name)).all(db).await
    }

    /// Creates a new role in the database.
    /// 
    /// This method creates a new role with the provided name. It includes validation
    /// to ensure that no role with the same name already exists.
    /// 
    /// # Arguments
    /// 
    /// * `db` - A reference to the database connection
    /// * `name` - The name for the new role
    /// 
    /// # Returns
    /// 
    /// * `Ok(RoleModel)` - The newly created role model
    /// * `Err(sea_orm::DbErr)` - Database error if the operation fails, including
    ///   custom error for duplicate role names
    /// 
    /// # Errors
    /// 
    /// This function will return an error if:
    /// * A role with the same name already exists
    /// * Database operation fails
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// match RoleService::create_role(&db, "moderator".to_string()).await {
    ///     Ok(role) => println!("Created role: {}", role.name),
    ///     Err(e) => println!("Failed to create role: {}", e),
    /// }
    /// ```
    pub async fn create_role(db: &DatabaseConnection, name: String) -> Result<RoleModel, sea_orm::DbErr> {
        // check for duplication
        let found_role = Role::find().filter(role_entity::Column::Name.eq(name.clone())).one(db).await?;
        if found_role.is_some() {
            return Err(sea_orm::DbErr::Custom("role already exists".to_string()));
        }
        // insert to database
        let active_model = RoleActiveModel {
            name: Set(name),
            ..Default::default()
        };
        active_model.insert(db).await
    }

    /// Updates an existing role in the database.
    /// 
    /// This method updates the name of an existing role identified by its ID.
    /// If the role doesn't exist, it returns `None`.
    /// 
    /// # Arguments
    /// 
    /// * `db` - A reference to the database connection
    /// * `id` - The unique identifier of the role to update
    /// * `name` - The new name for the role
    /// 
    /// # Returns
    /// 
    /// * `Ok(Some(RoleModel))` - The updated role model if found and updated
    /// * `Ok(None)` - If no role with the given ID exists
    /// * `Err(sea_orm::DbErr)` - Database error if the operation fails
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// match RoleService::update_role(&db, 1, "super_admin".to_string()).await? {
    ///     Some(role) => println!("Updated role: {}", role.name),
    ///     None => println!("Role not found"),
    /// }
    /// ```
    pub async fn update_role(db: &DatabaseConnection, id: i32, name: String) -> Result<Option<RoleModel>, sea_orm::DbErr> {
        if let Some(role) = Role::find_by_id(id).one(db).await? {
            let mut active_model: RoleActiveModel = role.into();
            active_model.name = Set(name);
            let updated = active_model.update(db).await?;
            Ok(Some(updated))
        } else {
            Ok(None)
        }
    }
}
