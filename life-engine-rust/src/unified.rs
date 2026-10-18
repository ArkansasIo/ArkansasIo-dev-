use rand::{Rng, SeedableRng};
use crate::{genome::Genome, neural::{decide,BrainAction}, ecology::Ecosystem, physics::{Particle,Vec2,integrate}};

#[derive(Clone, Debug)]
pub struct Agent { pub id:u64, pub parent:Option<u64>, pub genome:Genome, pub brain_inputs:Vec<f32>, pub energy:f32, pub age:u32, pub alive:bool, pub particle:Particle }
impl Agent {
 pub fn random(id:u64,rng:&mut impl Rng)->Self{Self{id,parent:None,genome:Genome::random(rng),brain_inputs:vec![0.0;3],energy:50.0,age:0,alive:true,particle:Particle{position:Vec2{x:rng.random_range(-50.0..50.0),y:rng.random_range(-50.0..50.0)},velocity:Vec2::default(),mass:1.0,health:10.0}}}
 pub fn tick(&mut self,eco:&Ecosystem,rng:&mut impl Rng){if !self.alive{return} self.age+=1;self.energy-=0.2;self.brain_inputs[0]=(eco.resources.energy/1000.0).min(1.0);self.brain_inputs[1]=eco.climate.toxicity;let action=decide(&self.brain_inputs);let d=match action{BrainAction::SeekFood=>1.0,BrainAction::FleePredator=>-1.0,BrainAction::Explore=>0.5,_=>0.0};self.particle.velocity.x+=rng.random_range(-1.0..1.0)*d;self.particle.velocity.y+=rng.random_range(-1.0..1.0)*d;integrate(&mut self.particle,1.0,Vec2{x:0.0,y:-0.01});if self.age>self.genome.lifespan||self.energy<=0.0||self.particle.health<=0.0{self.alive=false;}}
}

pub struct LifeEngine { pub tick:u64,pub agents:Vec<Agent>,pub ecosystem:Ecosystem,pub max_population:usize }
impl LifeEngine {
 pub fn new(seed:u64,population:usize)->Self{let mut rng=rand::rngs::StdRng::seed_from_u64(seed);let agents=(0..population).map(|i|Agent::random(i as u64,&mut rng)).collect();Self{tick:0,agents,ecosystem:Ecosystem::default(),max_population:population*4}}
 pub fn step(&mut self,seed:u64){let mut rng=rand::rngs::StdRng::seed_from_u64(seed^self.tick);self.tick+=1;self.ecosystem.tick();for a in &mut self.agents{a.tick(&self.ecosystem,&mut rng);}self.reproduce(&mut rng);self.agents.retain(|a|a.alive);}
 fn reproduce(&mut self,rng:&mut impl Rng){let parents:Vec<Agent>=self.agents.iter().filter(|a|a.energy>25.0&&a.alive).cloned().collect();for p in parents{if self.agents.len()>=self.max_population{break}if rng.random_bool(0.02){let mut child=p.clone();child.id=self.agents.len() as u64+1;child.parent=Some(p.id);child.age=0;child.energy=p.energy*0.5;child.genome.mutate(rng);child.particle.position.x+=rng.random_range(-5.0..5.0);self.agents.push(child);}}}
 pub fn population(&self)->usize{self.agents.len()}
}
