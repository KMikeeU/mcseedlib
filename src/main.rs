
mod features;
use features::endspikefeature::EndSpikeFeature;

mod chunk;
use chunk::slimechunk::SlimeChunk;

mod biomes;
use biomes::theendbiomesource::TheEndBiomeSource;
use biomes::biome;

mod random;
use random::*;

use simplexnoisesampler::SimplexNoiseSampler;
use jrandom::*;

extern crate image;
use image::{ImageBuffer, RgbImage};

fn main() {
    let world_seed: u64 = 1337;
    println!("Running tests on world seed '{}'", world_seed);
    
    
    // ----------------
    // End spikes
    // ----------------
    let spikes = EndSpikeFeature::new(world_seed);
    
    println!("--- End Spikes ---");
    println!("> Generating EndSpikes for world seed '{}'", world_seed);
    println!("> SpikeSeed is '{}'", spikes.spike_seed);
    
    let spikes = spikes.generate();
    
    for spike in spikes {
        println!("{}", spike);
    }
    
    
    // ----------------
    // Slime chunks
    // ----------------
    let slimechunk = SlimeChunk::new(world_seed);
    
    println!("--- Slime Chunks ---");
    // Tests for slime chunks in chunk coordinates from 0 to 20 on both axis
    for z in 0..20 {
        for x in 0..20 {
            print!("{}", match slimechunk.is_slime_chunk(x, z) {true => "X", false => " "} )
        }
        println!();
    }
    
    // ----------------
    // End biome map
    // ----------------
    let endgen = TheEndBiomeSource::new(world_seed);
    let noise = SimplexNoiseSampler::new(Random::new(world_seed));
    
    // The center of the map
    let pos: [i32;2] = [0, 0];
    // The size
    let dim: [u32;2] = [256, 256];
    
    let offset: [i32;2] = [-1 * dim[0] as i32 / 2, -1 * dim[1] as i32 / 2];
    

    println!("--- End Biomes ---");
    let mut end_out: RgbImage = ImageBuffer::new(dim[0], dim[1]);
    
    println!("Building image of TheEnd...");
    
    for x in 0..dim[0] as i32 {
        for z in 0..dim[1] as i32 {
            end_out.put_pixel(x as u32, z as u32, match endgen.get_biome(
                offset[0]*4 + x*4 + pos[0] / 4,
                0,
                offset[1]*4 + z*4 + pos[1] / 4,
                &noise) {
                    // Some colors
                    biome::TheEnd::Center => image::Rgb([128u8, 128u8, 255u8]),
                    biome::TheEnd::Highlands => image::Rgb([195u8, 189u8, 137u8]),
                    biome::TheEnd::Midlands => image::Rgb([235u8, 248u8, 182u8]),
                    biome::TheEnd::Barrens => image::Rgb([144u8, 144u8, 114u8]),
                    _ => image::Rgb([0, 0, 42])
            });
        }
    }
        
    let filename = "theend.png";
    println!("Saving map as {}...", filename);
    end_out.save(filename).unwrap();
    println!("--- Done! :) ---");
}