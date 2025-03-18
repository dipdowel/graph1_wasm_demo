use crate::demo::common::basic_concepts::{BasicConceptsUserData, BASIC_CONCEPTS_USER_DATA};
use crate::demo::d_000_intro::{get_user_data, IntroUserData};
use crate::demo::d_002_basic_concepts_pt2::get_snake;
use crate::demo::d_004_bouncy::{BouncyUserData, BOUNCY_USER_DATA};
use crate::demo::d_005_alpha::{AlphaUserData, ALPHA_USER_DATA};
use crate::demo::d_006_luminance_vs_intensity::{GhostsUserData, GHOSTS_USER_DATA};
use crate::demo::elements::snake::Snake;

// Shape the user-defined data, accessible via `ctx.user_data`
// `ctx.user_data` can be used to store arbitrary data that needs to live as long as the context itself.
// E.g. the state of the animation, the position of the object, or anything else that needs
// to be persisted between frames.
pub struct DemoUserData<'a> {
    pub intro: IntroUserData<'a>,
    pub bouncy: BouncyUserData,
    pub alpha: AlphaUserData,
    pub ghosts: GhostsUserData,
    pub basic_concepts_pt1: BasicConceptsUserData,
    pub snake: Snake
}

// Populate the user data with values defined in the corresponding demo modules
impl<'a> Default for DemoUserData<'a> {
    fn default() -> Self {
        DemoUserData {
            bouncy: BOUNCY_USER_DATA,
            alpha: ALPHA_USER_DATA,
            ghosts: GHOSTS_USER_DATA,
            basic_concepts_pt1: BASIC_CONCEPTS_USER_DATA,
            snake: get_snake(),
            intro: get_user_data::<'a>(),
        }
    }
}
