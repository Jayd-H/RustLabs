// Lab G - Threaded Colliding Particle Sim SUPER THREADED
// Jayden Holdsworth - 15/03/2025
use rand;
use scoped_threadpool::Pool;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

const NUM_PARTICLES: usize = 10000;
const ENCLOSURE_SIZE: f32 = 10.0;
const NUM_MOVEMENT_THREADS: usize = 4;
const NUM_COLLISION_THREADS: usize = 4;
const SIMULATION_STEPS: usize = 1000;
const COLLISION_DISTANCE_SQR: f32 = 0.01;
const CELL_SIZE: f32 = 0.5;
const GRID_DIM: usize = (ENCLOSURE_SIZE / CELL_SIZE) as usize + 1;

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
        
        dsqr < COLLISION_DISTANCE_SQR
    }
    
    fn get_cell_coords(&self) -> (usize, usize) {
        let x_cell = (self.x / CELL_SIZE).min(GRID_DIM as f32 - 1.0) as usize;
        let y_cell = (self.y / CELL_SIZE).min(GRID_DIM as f32 - 1.0) as usize;
        (x_cell, y_cell)
    }
    
    fn might_collide(&self, other: &Particle) -> bool {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        
        // quick check before full distance calculation
        if dx.abs() > CELL_SIZE || dy.abs() > CELL_SIZE {
            return false;
        }
        
        true
    }
}

//* Spatial Grid Class */
struct SpatialGrid {
    cells: Vec<Vec<usize>>,
}

impl SpatialGrid {
    fn new() -> Self {
        let mut cells = Vec::with_capacity(GRID_DIM * GRID_DIM);
        for _ in 0..(GRID_DIM * GRID_DIM) {
            cells.push(Vec::new());
        }
        
        SpatialGrid { cells }
    }
    
    fn clear(&mut self) {
        for cell in &mut self.cells {
            cell.clear();
        }
    }
    
    fn add_particle(&mut self, particle_idx: usize, particle: &Particle) {
        let (x_cell, y_cell) = particle.get_cell_coords();
        let cell_idx = y_cell * GRID_DIM + x_cell;
        self.cells[cell_idx].push(particle_idx);
    }
    
    fn get_neighbor_indices(&self, particle: &Particle) -> Vec<usize> {
        let (x_cell, y_cell) = particle.get_cell_coords();
        let mut neighbors = Vec::new();
        
        // check 3x3 neighborhood of cells
        for dy in -1..=1 {
            for dx in -1..=1 {
                let nx = x_cell as isize + dx;
                let ny = y_cell as isize + dy;
                
                // Skip out-of-bounds cells
                if nx < 0 || ny < 0 || nx >= GRID_DIM as isize || ny >= GRID_DIM as isize {
                    continue;
                }
                
                let cell_idx = (ny as usize) * GRID_DIM + (nx as usize);
                neighbors.extend(&self.cells[cell_idx]);
            }
        }
        
        neighbors
    }
}

//* Particle System Class */
struct ParticleSystem {
    current_particles: Vec<Particle>,
    next_particles: Vec<Particle>,
    spatial_grid: SpatialGrid,
    collision_count: AtomicUsize,
}

impl ParticleSystem {
    fn new() -> Self {
        let mut current_particles = Vec::with_capacity(NUM_PARTICLES);
        
        // Initialize particles in a grid pattern
        for i in 0..NUM_PARTICLES {
            let row = i / 10;
            let col = i % 10;
            
            let x = (col as f32) * ENCLOSURE_SIZE / 10.0 + 0.5;
            let y = (row as f32) * ENCLOSURE_SIZE / 10.0 + 0.5;
            
            current_particles.push(Particle::new(x, y));
        }
        
        // Create second buffer for double-buffering
        let next_particles = current_particles.clone();
        
        ParticleSystem { 
            current_particles,
            next_particles,
            spatial_grid: SpatialGrid::new(),
            collision_count: AtomicUsize::new(0),
        }
    }
    
    fn update_spatial_grid(&mut self) {
        self.spatial_grid.clear();
        
        for (idx, particle) in self.current_particles.iter().enumerate() {
            self.spatial_grid.add_particle(idx, particle);
        }
    }
    
    fn run_simulation_concurrent(&mut self, steps: usize) {
        println!("\nRunning concurrent simulation for {} steps...", steps);
        let timer = Instant::now();
        
        let mut movement_pool = Pool::new(NUM_MOVEMENT_THREADS as u32);
        let mut collision_pool = Pool::new(NUM_COLLISION_THREADS as u32);
        
        for step in 1..=steps {
            // update grid for collision detection
            self.update_spatial_grid();
            
            // move particles in parallel
            movement_pool.scoped(|scope| {
                let chunk_size = (NUM_PARTICLES + NUM_MOVEMENT_THREADS - 1) / NUM_MOVEMENT_THREADS;
                
                for (thread_idx, chunk) in self.next_particles.chunks_mut(chunk_size).enumerate() {
                    let current_particles = &self.current_particles;
                    
                    scope.execute(move || {
                        for (i, particle) in chunk.iter_mut().enumerate() {
                            let global_idx = thread_idx * chunk_size + i;
                            if global_idx < current_particles.len() {
                                // Copy from current state
                                *particle = current_particles[global_idx];
                                
                                // Move the particle in next state
                                let dx = (rand::random::<f32>() - 0.5) * 0.2;
                                let dy = (rand::random::<f32>() - 0.5) * 0.2;
                                
                                particle.x = f32::min(f32::max(particle.x + dx, 0.0), ENCLOSURE_SIZE);
                                particle.y = f32::min(f32::max(particle.y + dy, 0.0), ENCLOSURE_SIZE);
                            }
                        }
                    });
                }
            });
            
            // check collisions in parallel using spatial grid
            collision_pool.scoped(|scope| {
                let chunk_size = (NUM_PARTICLES + NUM_COLLISION_THREADS - 1) / NUM_COLLISION_THREADS;
                
                for thread_idx in 0..NUM_COLLISION_THREADS {
                    let current_particles = &self.current_particles;
                    let spatial_grid = &self.spatial_grid;
                    let collision_counter = &self.collision_count;
                    
                    scope.execute(move || {
                        let start_idx = thread_idx * chunk_size;
                        let end_idx = (start_idx + chunk_size).min(NUM_PARTICLES);
                        
                        if start_idx < end_idx {
                            for i in start_idx..end_idx {
                                let particle_i = &current_particles[i];
                                
                                // get nearby particles from grid
                                let neighbors = spatial_grid.get_neighbor_indices(particle_i);
                                
                                for &j in &neighbors {
                                    // skip self-comparison and duplicates
                                    if i == j || j < i {
                                        continue;
                                    }
                                    
                                    let particle_j = &current_particles[j];
                                    
                                    // check for collision
                                    if particle_i.might_collide(particle_j) && particle_i.collide(particle_j) {
                                        collision_counter.fetch_add(1, Ordering::Relaxed);
                                    }
                                }
                            }
                        }
                    });
                }
            });
            
            // swap buffers for next step
            std::mem::swap(&mut self.current_particles, &mut self.next_particles);
            
            if step % 100 == 0 {
                println!("Completed {} steps", step);
            }
        }
        
        let elapsed = timer.elapsed();
        println!("Simulation complete. Total collisions: {}", 
                 self.collision_count.load(Ordering::Relaxed));
        println!("Simulation took: {}.{:03} seconds", elapsed.as_secs(), elapsed.subsec_millis());
    }
    
    fn print_state(&self, count: usize, prefix: &str) {
        println!("\n{} - showing first {} particles:", prefix, count);
        for i in 0..count.min(self.current_particles.len()) {
            println!("Particle {}: ({:.2}, {:.2})", 
                    i, 
                    self.current_particles[i].x, 
                    self.current_particles[i].y);
        }
    }
    
    fn print_statistics(&self) {
        let avg_x = self.current_particles.iter().map(|p| p.x).sum::<f32>() / NUM_PARTICLES as f32;
        let avg_y = self.current_particles.iter().map(|p| p.y).sum::<f32>() / NUM_PARTICLES as f32;
        println!("\nAverage position of all particles: ({:.2}, {:.2})", avg_x, avg_y);
    }
}

//* Main */
fn main() {
    let mut particle_system = ParticleSystem::new();
    
    particle_system.print_state(5, "Initial state");
    
    particle_system.run_simulation_concurrent(SIMULATION_STEPS);
    
    particle_system.print_state(5, "Final state");
    particle_system.print_statistics();
    
    println!("\nConcurrent simulation completed successfully!");
}