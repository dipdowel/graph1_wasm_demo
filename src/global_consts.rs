/// Width of the window, in pixels
pub const WIN_WIDTH: u32 = 480;
/// Height of the window, in pixels
pub const WIN_HEIGHT: u32 = 240;
/// Framebuffer size, in pixels (u32)
pub const BUF_LEN: usize = (WIN_WIDTH * WIN_HEIGHT) as usize;
/// Framebuffer size, in bytes
pub const BUF_SIZE_BYTES: usize = BUF_LEN * 4;
