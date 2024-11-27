use graph1::text::font::PixelFont;
use graph1::text::printer;
use graph1::utils::color::palettes::RetroNeon;

//---------------------------------------------------------------------
// Configure the user data for typing text in Basic Concepts pt. 1
pub struct BasicConceptsUserData {
    pub text_color_props: printer::ColorProperties<'static>,
    pub text_font: Option<PixelFont>,
}

pub const BASIC_CONCEPTS_USER_DATA: BasicConceptsUserData = BasicConceptsUserData {
    text_color_props: printer::ColorProperties {
        color: Some(RetroNeon::LASER_LIME),
        color_transformer: None,
        data: None,
    },
    // Let's put an instantiated font into the user data
    // so that we don't have to instantiate it on every frame
    text_font: None,
};
//---------------------------------------------------------------------