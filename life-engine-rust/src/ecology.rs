#[derive(Clone, Debug)]
pub struct Climate { pub temperature: f32, pub radiation: f32, pub toxicity: f32, pub fertility: f32 }
#[derive(Clone, Debug)]
pub struct Resource { pub energy: f32, pub water: f32, pub mineral: f32 }
#[derive(Clone, Debug)]
pub struct Ecosystem { pub climate: Climate, pub resources: Resource, pub predation: bool, pub seasons: bool }
impl Default for Ecosystem { fn default()->Self{Self{climate:Climate{temperature:0.5,radiation:0.0,toxicity:0.0,fertility:1.0},resources:Resource{energy:1000.0,water:1000.0,mineral:1000.0},predation:true,seasons:true}} }
impl Ecosystem { pub fn tick(&mut self){ self.resources.energy=(self.resources.energy+2.0*self.climate.fertility).min(100000.0); self.resources.water=(self.resources.water+1.0).min(100000.0); } }
