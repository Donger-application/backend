use crate::domain::group::group_entity::Entity as Group;
use crate::domain::role::role_entity::Entity as Role;
use crate::domain::user::user_entity;
use crate::domain::user::user_entity::{
    ActiveModel as UserActiveModel, Entity as User, Model as UserModel,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, Order, QueryFilter, QueryOrder, Set};
pub struct UserService;

impl UserService {
    pub async fn get_all_users(db: &DatabaseConnection) -> Result<Vec<UserModel>, sea_orm::DbErr> {
        User::find().all(db).await
    }

    pub async fn get_user_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<UserModel>, sea_orm::DbErr> {
        User::find_by_id(id).one(db).await
    }

    pub async fn get_user_by_email(
        db: &DatabaseConnection,
        email: &str,
    ) -> Result<Option<UserModel>, sea_orm::DbErr> {
        User::find()
            .filter(user_entity::Column::Email.eq(email))
            .one(db)
            .await
    }

    pub async fn get_user_by_name(
        db: &DatabaseConnection,
        name: &str,
    ) -> Result<Vec<UserModel>, sea_orm::DbErr> {
        User::find()
            .filter(user_entity::Column::Name.contains(name))
            .all(db)
            .await
    }

    pub async fn get_user_by_display_id(
        db: &DatabaseConnection,
        user_display_id: &str,
    ) -> Result<Option<UserModel>, sea_orm::DbErr> {
        User::find()
            .filter(user_entity::Column::UserDisplayId.eq(user_display_id))
            .one(db)
            .await
    }

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
        let found_user = User::find()
            .filter(user_entity::Column::Email.eq(email.clone()))
            .one(db)
            .await?;
        if found_user.is_some() {
            return Err(sea_orm::DbErr::Custom(
                "user with this email already exists".to_string(),
            ));
        }

        // Check if user_display_id already exists
        let found_display_id = User::find()
            .filter(user_entity::Column::UserDisplayId.eq(user_display_id.clone()))
            .one(db)
            .await?;
        if found_display_id.is_some() {
            return Err(sea_orm::DbErr::Custom(
                "user with this display ID already exists".to_string(),
            ));
        }

        // Check if name already exists
        let found_name = User::find()
            .filter(user_entity::Column::Name.eq(name.clone()))
            .one(db)
            .await?;
        if found_name.is_some() {
            return Err(sea_orm::DbErr::Custom(
                "user with this name already exists".to_string(),
            ));
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
                        .one(db)
                        .await?;
                    if found_name.is_some() {
                        return Err(sea_orm::DbErr::Custom(
                            "user with this name already exists".to_string(),
                        ));
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
                        .one(db)
                        .await?;
                    if found_user.is_some() {
                        return Err(sea_orm::DbErr::Custom(
                            "user with this email already exists".to_string(),
                        ));
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
                        .one(db)
                        .await?;
                    if found_display_id.is_some() {
                        return Err(sea_orm::DbErr::Custom(
                            "user with this display ID already exists".to_string(),
                        ));
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

    pub async fn delete_user(db: &DatabaseConnection, id: i32) -> Result<bool, sea_orm::DbErr> {
        let result = User::delete_by_id(id).exec(db).await?;
        Ok(result.rows_affected > 0)
    }

    pub async fn get_user_indebt(group_id:i32, db: &DatabaseConnection) -> Result<Vec<UserModel>, sea_orm::DbErr> {
        User::find().filter(user_entity::Column::GroupId.eq(group_id)).order_by(user_entity::Column::Balance, Order::Asc).all(db).await
    }
}
