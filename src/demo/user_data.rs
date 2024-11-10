use crate::demo::d_003_bouncy::{BouncyUserData, BOUNCY_USER_DATA};
use crate::demo::d_888_luminance_vs_intensity::{GhostsUserData, GHOSTS_USER_DATA};

// Shape the user-defined data, accessible via `ctx.user_data`
// `ctx.user_data` can be used to store arbitrary data that needs to live as long as the context itself.
// E.g. the state of the animation, the position of the object, or anything else that needs
// to be persisted between frames.
pub struct DemoUserData {
    pub bouncy: BouncyUserData,
    pub ghosts: GhostsUserData,
}

// Populate the user data with values defined in the corresponding demo modules
impl Default for DemoUserData {
    fn default() -> Self {
        DemoUserData {
            bouncy: BOUNCY_USER_DATA,
            ghosts: GHOSTS_USER_DATA,
        }
    }
}