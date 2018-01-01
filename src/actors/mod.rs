mod projectile;
pub use self::projectile::Projectile;

use na;

pub trait Actor {
    fn update(&mut self, delta_nanos: f64);
    fn borrow_position(&mut self) -> &na::Vector2<f64>;
}

