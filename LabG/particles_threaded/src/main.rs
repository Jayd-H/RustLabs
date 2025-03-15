const NUM_PARTICLES: usize = 100;
const ENCLOSURE_SIZE: f32 = 10.0;
const NUM_OF_THREADS: usize = 4;

#[derive(Debug, Copy, Clone)]
pub struct Particle {
    x: f32,
    y: f32,
}

impl Particle {
    fn new(x: f32, y: f32) -> Self {
        Particle { x, y }
    }
}

struct ParticleSystem {
    particles: Vec<Particle>,
}

impl ParticleSystem {
    fn new() -> Self {
        let mut particles = Vec::with_capacity(NUM_PARTICLES);
        
        for i in 0..NUM_PARTICLES {
            let row = i / 10;
            let col = i % 10;
            
            let x = (col as f32) * ENCLOSURE_SIZE / 10.0 + 0.5;
            let y = (row as f32) * ENCLOSURE_SIZE / 10.0 + 0.5;
            
            particles.push(Particle::new(x, y));
        }
        
        ParticleSystem { particles }
    }
    
    fn move_particles(&mut self) {
        for particle in &mut self.particles {
            let dx = (rand::random::<f32>() - 0.5) * 0.2;
            let dy = (rand::random::<f32>() - 0.5) * 0.2;
            
            particle.x = f32::min(f32::max(particle.x + dx, 0.0), ENCLOSURE_SIZE);
            particle.y = f32::min(f32::max(particle.y + dy, 0.0), ENCLOSURE_SIZE);
        }
    }
    
    fn move_particles_threaded(&mut self) {
        let mut pool = scoped_threadpool::Pool::new(NUM_OF_THREADS as u32);
        
        let chunk_size = (NUM_PARTICLES + NUM_OF_THREADS - 1) / NUM_OF_THREADS;
        
        pool.scoped(|scope| {
            for chunk in self.particles.chunks_mut(chunk_size) {
                scope.execute(move || {
                    thread_main(chunk, ENCLOSURE_SIZE);
                });
            }
        });
    }
    
   fn run_simulation(&mut self, threaded: bool) -> usize {
    use std::time::{Duration, Instant};
    
    let simulation_duration = Duration::from_secs(10);
    let start_time = Instant::now();
    let mut steps = 0;
    
    while start_time.elapsed() < simulation_duration {
        if threaded {
            self.move_particles_threaded();
        } else {
            self.move_particles();
        }
        steps += 1;
    }
    
    let elapsed = start_time.elapsed();
    println!("Simulation completed {} steps in {:?}", steps, elapsed);
    steps
}
}

pub fn thread_main(list: &mut [Particle], enclosure_size: f32) {
    for particle in list {
        let dx = (rand::random::<f32>() - 0.5) * 0.2;
        let dy = (rand::random::<f32>() - 0.5) * 0.2;
        
        particle.x = f32::min(f32::max(particle.x + dx, 0.0), enclosure_size);
        particle.y = f32::min(f32::max(particle.y + dy, 0.0), enclosure_size);
    }
}

fn main() {
    // Run non-threaded version
    let mut particle_system = ParticleSystem::new();
    
    println!("Initial state (non-threaded) - showing first 5 particles:");
    for i in 0..5 {
        println!("Particle {}: ({:.2}, {:.2})", 
                 i, 
                 particle_system.particles[i].x, 
                 particle_system.particles[i].y);
    }
    
    println!("\nRunning non-threaded simulation for 10 seconds...");
    let non_threaded_steps = particle_system.run_simulation(false);
    
    println!("\nFinal state (non-threaded) - showing first 5 particles:");
    for i in 0..5 {
        println!("Particle {}: ({:.2}, {:.2})", 
                 i, 
                 particle_system.particles[i].x, 
                 particle_system.particles[i].y);
    }
    
    let avg_x = particle_system.particles.iter().map(|p| p.x).sum::<f32>() / NUM_PARTICLES as f32;
    let avg_y = particle_system.particles.iter().map(|p| p.y).sum::<f32>() / NUM_PARTICLES as f32;
    println!("\nAverage position of all particles (non-threaded): ({:.2}, {:.2})", avg_x, avg_y);
    
    // Run threaded version
    let mut particle_system_threaded = ParticleSystem::new();
    
    println!("\nInitial state (threaded) - showing first 5 particles:");
    for i in 0..5 {
        println!("Particle {}: ({:.2}, {:.2})", 
                 i, 
                 particle_system_threaded.particles[i].x, 
                 particle_system_threaded.particles[i].y);
    }
    
    println!("\nRunning multi-threaded simulation for 10 seconds...");
    let threaded_steps = particle_system_threaded.run_simulation(true);
    
    println!("\nFinal state (threaded) - showing first 5 particles:");
    for i in 0..5 {
        println!("Particle {}: ({:.2}, {:.2})", 
                 i, 
                 particle_system_threaded.particles[i].x, 
                 particle_system_threaded.particles[i].y);
    }
    
    let avg_x = particle_system_threaded.particles.iter().map(|p| p.x).sum::<f32>() / NUM_PARTICLES as f32;
    let avg_y = particle_system_threaded.particles.iter().map(|p| p.y).sum::<f32>() / NUM_PARTICLES as f32;
    println!("\nAverage position of all particles (threaded): ({:.2}, {:.2})", avg_x, avg_y);
    
    // Performance comparison
    println!("\nPerformance comparison:");
    println!("- Non-threaded: {} steps", non_threaded_steps);
    println!("- Threaded: {} steps", threaded_steps);
    println!("- Speedup: {:.2}x", threaded_steps as f32 / non_threaded_steps as f32);
}