extern crate minifb;
extern crate rand;

use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};

const NUM_PARTICLES: usize = 100;
const ENCLOSURE_SIZE: f32 = 10.0;
const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 800;
const PARTICLE_SIZE: usize = 5;

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
    
    fn run_simulation(&mut self) {
        let simulation_duration = Duration::from_secs(10);
        let start_time = Instant::now();
        
        while start_time.elapsed() < simulation_duration {
            self.move_particles();
        }
    }
    
    fn run_simulation_with_visualization(&mut self) {
        let mut window = Window::new(
            "Particle Simulation",
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        let mut buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];
        
        let start_time = Instant::now();
        let simulation_duration = Duration::from_secs(10);
        
        let mut last_render_time = Instant::now();
        let render_interval = Duration::from_millis(16);
        
        while window.is_open() && !window.is_key_down(Key::Escape) && start_time.elapsed() < simulation_duration {
            self.move_particles();
            
            if last_render_time.elapsed() >= render_interval {
                for i in buffer.iter_mut() {
                    *i = 0;
                }
                
                for (i, particle) in self.particles.iter().enumerate() {
                    let px = (particle.x / ENCLOSURE_SIZE * WINDOW_WIDTH as f32) as usize;
                    let py = (particle.y / ENCLOSURE_SIZE * WINDOW_HEIGHT as f32) as usize;
                    
                    let color = 0xFF000000 | 
                                (((i as u32 * 50) % 256) << 16) | 
                                (((i as u32 * 100) % 256) << 8) | 
                                ((i as u32 * 150) % 256);
                    
                    for dy in 0..PARTICLE_SIZE {
                        for dx in 0..PARTICLE_SIZE {
                            let draw_x = px.saturating_add(dx).min(WINDOW_WIDTH - 1);
                            let draw_y = py.saturating_add(dy).min(WINDOW_HEIGHT - 1);
                            let idx = draw_y * WINDOW_WIDTH + draw_x;
                            if idx < buffer.len() {
                                buffer[idx] = color;
                            }
                        }
                    }
                }
                
                window.update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();
                last_render_time = Instant::now();
            }
        }
    }
}

fn main() {
    let mut particle_system = ParticleSystem::new();
    
    println!("Initial state - showing first 5 particles:");
    for i in 0..5 {
        println!("Particle {}: ({:.2}, {:.2})", 
                 i, 
                 particle_system.particles[i].x, 
                 particle_system.particles[i].y);
    }
    
    particle_system.move_particles();
    
    println!("\nAfter movement - showing first 5 particles:");
    for i in 0..5 {
        println!("Particle {}: ({:.2}, {:.2})", 
                 i, 
                 particle_system.particles[i].x, 
                 particle_system.particles[i].y);
    }
    
    println!("\nRunning simulation with visualization for 10 seconds...");
    particle_system.run_simulation_with_visualization();
    println!("Simulation complete");
    
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