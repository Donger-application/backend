use crate::interfaces::rest::role_controller::__path_create_role;
use crate::interfaces::dtos::role_dto::{CreateRoleDto, RoleDto};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(create_role),
    components(
        schemas(CreateRoleDto, RoleDto)
    )
)]
pub struct ApiDoc;