use na;

#[derive(Clone,Debug)]
pub struct Projectile {
    pub position: na::Vector2<f64>,
    pub velocity: na::Vector2<f64>
}

impl Projectile {
    pub fn new(position: na::Vector2<f64>, velocity: na::Vector2<f64>) -> Projectile {
        Projectile { position, velocity }
    }
}

impl super::Actor for Projectile {
    fn update(&mut self, delta_nanos: f64) {
        let delta_velocity = self.velocity * delta_nanos;
        self.position += delta_velocity
    }

    fn borrow_position(&mut self) -> &na::Vector2<f64> {
        &self.position
    }
}