#[derive(Clone, Copy, Debug, Default)]
pub struct Vec2 { pub x: f32, pub y: f32 }
impl Vec2 { pub fn add(self, b: Vec2) -> Vec2 { Vec2{x:self.x+b.x,y:self.y+b.y} } pub fn scale(self,s:f32)->Vec2{Vec2{x:self.x*s,y:self.y*s}} }
#[derive(Clone, Debug)]
pub struct Particle { pub position: Vec2, pub velocity: Vec2, pub mass: f32, pub health: f32 }
#[derive(Clone, Copy, Debug)]
pub enum Material { Soft, Rigid, Fluid }
pub fn integrate(p: &mut Particle, dt: f32, gravity: Vec2) { p.velocity = p.velocity.add(gravity.scale(dt)); p.position = p.position.add(p.velocity.scale(dt)); }
pub fn damage(p: &mut Particle, amount: f32) { p.health = (p.health - amount).max(0.0); }
