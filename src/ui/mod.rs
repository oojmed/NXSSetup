pub mod checkbox;
pub mod text;
pub mod separator;
pub mod title;
pub mod subtitle;

pub mod group;
pub mod category;
pub mod window;

pub mod collider;

pub mod utils;
pub mod colors;

pub trait Render {
    fn render(& mut self);
}

pub trait Interact {
    fn interact(& mut self);
}

pub mod prelude {
    pub use crate::ui::Render;
    pub use crate::ui::Interact;
    
    pub use crate::ui::checkbox;
    pub use crate::ui::text;
    pub use crate::ui::separator;
    pub use crate::ui::collider;
    pub use crate::ui::title;
    pub use crate::ui::subtitle;

    pub use crate::ui::group;
    pub use crate::ui::category;
    pub use crate::ui::window;

    pub use crate::ui::utils;
    pub use crate::ui::colors;
}
