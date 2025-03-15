// Lab G - Threaded Colliding Particle Sim
// Jayden Holdsworth - 15/03/2025
use rand;
use scoped_threadpool;
use std::time::Instant;

const NUM_PARTICLES: usize = 100;
const ENCLOSURE_SIZE: f32 = 10.0;
const NUM_MOVEMENT_THREADS: usize = 4;
const SIMULATION_STEPS: usize = 100;

//* Particle Class */
#[derive(Debug, Copy, Clone)]
pub struct Particle {
    x: f32,
    y: f32,
}

impl Particle {
    fn new(x: f32, y: f32) -> Self {
        Particle { x, y }
    }
    
    fn collide(&self, other: &Particle) -> bool {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dsqr = dx * dx + dy * dy;
        
        dsqr < 0.01
    }
}

//* Particle System Class */
struct ParticleSystem {
    particles: Vec<Particle>,
}

impl ParticleSystem {
    fn new() -> Self {
        let mut particles = Vec::with_capacity(NUM_PARTICLES);
        // particles are created in a grid to avoid initial collisions
        for i in 0..NUM_PARTICLES {
            let row = i / 10;
            let col = i % 10;
            
            let x = (col as f32) * ENCLOSURE_SIZE / 10.0 + 0.5;
            let y = (row as f32) * ENCLOSURE_SIZE / 10.0 + 0.5;
            
            particles.push(Particle::new(x, y));
        }
        
        ParticleSystem { 
            particles,
        }
    }
    
    fn move_particles_threaded(&mut self) {
        let mut pool = scoped_threadpool::Pool::new(NUM_MOVEMENT_THREADS as u32);
        
        let chunk_size = (NUM_PARTICLES + NUM_MOVEMENT_THREADS - 1) / NUM_MOVEMENT_THREADS;
        
        pool.scoped(|scope| {
            for chunk in self.particles.chunks_mut(chunk_size) {
                scope.execute(move || {
                    for particle in chunk {
                        let dx = (rand::random::<f32>() - 0.5) * 0.2;
                        let dy = (rand::random::<f32>() - 0.5) * 0.2;
                        
                        particle.x = f32::min(f32::max(particle.x + dx, 0.0), ENCLOSURE_SIZE);
                        particle.y = f32::min(f32::max(particle.y + dy, 0.0), ENCLOSURE_SIZE);
                    }
                });
            }
        });
    }
    
    // checks for collisions between particles on a separate thread
    fn check_collisions_threaded(&self, step: usize) -> usize {
        let mut total_collisions = 0;
        let mut pool = scoped_threadpool::Pool::new(1);
        
        pool.scoped(|scope| {
            let particles = &self.particles;
            let collisions_ref = &mut total_collisions;
            
            scope.execute(move || {
                let mut step_collisions = 0;
                
                for i in 0..particles.len() {
                    for j in (i+1)..particles.len() {
                        if particles[i].collide(&particles[j]) {
                            step_collisions += 1;
                        }
                    }
                }
                
                *collisions_ref = step_collisions;
                if step_collisions > 0 {
                    println!("Step: {} - Collisions: {}", step, step_collisions,);
                }
            });
        });
        
        total_collisions
    }
    
    fn run_simulation(&mut self, steps: usize) {
        println!("\nRunning simulation for {} steps...", steps);
        let timer = Instant::now();
        let mut total_collisions = 0;
        
        for step in 1..=steps {
            self.move_particles_threaded();
            
            total_collisions += self.check_collisions_threaded(step);
            
            if step % 100 == 0 {
                println!("Completed {} steps", step);
            }
        }
        
        let elapsed = timer.elapsed();
        println!("Simulation complete. Total collisions: {}", total_collisions);
        println!("Simulation took: {}.{:03} seconds", elapsed.as_secs(), elapsed.subsec_millis());
    }
}

//* Main */
fn main() {
    let mut particle_system = ParticleSystem::new();
    
    println!("Initial state - showing first 5 particles:");
    for i in 0..5 {
        println!("Particle {}: ({:.2}, {:.2})", 
                 i, 
                 particle_system.particles[i].x, 
                 particle_system.particles[i].y);
    }
    
    particle_system.run_simulation(SIMULATION_STEPS);
    
    println!("\nFinal state - showing first 5 particles:");
    for i in 0..5 {
        println!("Particle {}: ({:.2}, {:.2})", 
                 i, 
                 particle_system.particles[i].x, 
                 particle_system.particles[i].y);
    }
    
    let avg_x = particle_system.particles.iter().map(|p| p.x).sum::<f32>() / NUM_PARTICLES as f32;
    let avg_y = particle_system.particles.iter().map(|p| p.y).sum::<f32>() / NUM_PARTICLES as f32;
    println!("\nAverage position of all particles: ({:.2}, {:.2})", avg_x, avg_y);
}