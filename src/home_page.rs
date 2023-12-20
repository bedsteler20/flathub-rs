use crate::AppHit;

pub struct HomePage {
    pub popular_apps: Vec<AppHit>,
    pub new_apps: Vec<AppHit>,
    pub updated_apps: Vec<AppHit>,
    pub verified_apps: Vec<AppHit>,
}
