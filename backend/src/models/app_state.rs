#[derive(Clone)]
pub struct AppState {
    // Key: project id; Value: live state
    pub sb_client: SupabaseClient,
    pub auth_client: AuthClient,
}
