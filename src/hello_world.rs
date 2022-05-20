//! Hello world - my first blog

#[doc = include_str!("./md/hello_world/hello_world_intro.md")]
pub const P0_Introduction: u8 = 0;

/// math support:
/// $$m = \frac{m_0}{\sqrt{1-\frac{v^2}{c^2}}}$$
pub const P1_Equation: u8 = 0;


#[doc = include_str!("./md/hello_world/hello_world_summary.md")]
pub const P2_Summary: u8 = 0;