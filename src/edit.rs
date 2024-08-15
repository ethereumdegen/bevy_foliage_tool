use crate::regionmap::RegionMapU8;
use std::fs::File;
use std::io::BufWriter;
use std::ops::{Add, Div, Neg};
use std::path::{Path, PathBuf};

use bevy::ecs::entity::Entity;
use bevy::math::Vec2;

use bevy::ecs::event::Event;
use bevy::prelude::EventReader;

use bevy::asset::{AssetServer, Assets};
use bevy::render::render_resource::{Extent3d, TextureFormat};
use bevy::render::texture::Image;

use bevy::prelude::*;
 
use core::fmt::{self, Display, Formatter};

  
use crate::regions::{RegionDataEvent, RegionPlaneMesh, RegionsData, RegionsDataMapResource};
use crate::regions_config::RegionsConfig;
use crate::regions_material::RegionsMaterialExtension;

 
 
use anyhow::{Context, Result};

use serde::{Deserialize, Serialize};
use serde_json;

use rand::Rng;

use core::cmp::{max, min};


pub struct BevyRegionEditsPlugin {
    
}

impl Default for BevyRegionEditsPlugin {
    fn default() -> Self {
        Self {
             
        }
    }
}
impl Plugin for BevyRegionEditsPlugin {
    fn build(&self, app: &mut App) {


      app.add_event::<EditRegionEvent>();
       app.add_event::<RegionCommandEvent>();
       app.add_event::<RegionBrushEvent>();
        app.add_systems(Update, apply_tool_edits); //put this in a sub plugin ?
        app.add_systems(Update, apply_command_events);


    }
}

#[derive(Debug, Clone)]
pub enum EditingTool {
    SetRegionMap { region_index: u8 },        // height, radius, save to disk 
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum BrushType {
    #[default]
    SetExact, // hardness ?
    Smooth,
    //Noise,
    EyeDropper,
}

impl Display for BrushType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let label = match self {
            BrushType::SetExact => "SetExact",
            BrushType::Smooth => "Smooth",
          //  BrushType::Noise => "Noise",
            BrushType::EyeDropper => "EyeDropper",
        };

        write!(f, "{}", label)
    }
}

// entity, editToolType, coords, magnitude
#[derive(Event, Debug, Clone)]
pub struct EditRegionEvent {
    pub entity: Entity, //should always be the plane 
    pub tool: EditingTool,
    pub radius: f32,
    pub brush_hardness: f32, //1.0 is full
    pub coordinates: Vec2,
    pub brush_type: BrushType,
}

#[derive(Event, Debug, Clone)]
pub enum RegionBrushEvent {
    EyeDropRegionIndex { region_index: u8 },
  //  EyeDropSplatMap { r: u8, g: u8, b: u8 },
}

#[derive(Event, Debug, Clone)]
pub enum RegionCommandEvent {
    SaveAll ,  
}

pub fn apply_command_events(
    asset_server: Res<AssetServer>,

   // mut chunk_query: Query<(&Chunk, &mut ChunkData, &Parent, &Children)>, //chunks parent should have terrain data

    mut images: ResMut<Assets<Image>>,
    mut region_materials: ResMut<Assets<RegionsMaterialExtension>>,

    mut region_maps_res: ResMut<RegionsDataMapResource>, //like height map resource 

    region_data_query: Query<(&RegionsData, &RegionsConfig)>,

    
    mut ev_reader: EventReader<RegionCommandEvent>,
) {
    for ev in ev_reader.read() {
       
           

            let Some((region_data, region_config)) = region_data_query
                    .get_single().ok() else {continue};



            match ev {
                RegionCommandEvent::SaveAll => {
                    //let file_name = format!("{}.png", chunk.chunk_id);
                     let asset_folder_path = PathBuf::from("assets");
                    let region_texture_path = &region_config.region_texture_path;
                     
                    
                //    info!("path {:?}",region_data_path);
                      if let Some(region_data) =
                          &  region_maps_res.regions_data_map
                        {

                        save_region_index_map_to_disk(
                                &region_data,
                                asset_folder_path.join( region_texture_path ),
                        );
                    }
                     
 

                     

                    println!("saved region data ");
                
            }
          }
        }
     

    //  Ok(())
}

pub fn apply_tool_edits(
  
    region_data_query: Query<(&mut RegionsData, &RegionsConfig)> , 

   

    mut region_map_data_res: ResMut<RegionsDataMapResource>,
 
     region_plane_mesh_query: Query<(Entity,   &GlobalTransform), With<RegionPlaneMesh>>,



    mut ev_reader: EventReader<EditRegionEvent>,

    mut evt_writer: EventWriter<RegionBrushEvent>,

    mut region_data_event_writer: EventWriter<RegionDataEvent>
) {
    for ev in ev_reader.read() {
        eprintln!("-- {:?} -- region edit event!", &ev.tool);

       let Some((region_data, region_config)) = region_data_query
                    .get_single().ok() else {
                          warn!("no regions entity found" );
                        continue
                    };



        let intersected_entity = &ev.entity;

       
       let Some((region_plane_entity,  _ )) = region_plane_mesh_query.get(intersected_entity.clone()).ok() else {
        warn!("region plane not intersected");
        continue
    } ;
            //let mut chunk_entities_within_range: Vec<Entity> = Vec::new();

            let   plane_dimensions = region_config.boundary_dimensions.clone(); //compute me from  config
          

      

             let tool_coords: &Vec2 = &ev.coordinates;
             info!("tool coords {:?}", tool_coords);

            

            
            let average_height = 0; //for now  // total_height as f32 / heights_len as f32;
            // ------
            let radius = &ev.radius;
            let brush_type = &ev.brush_type;

              info!("Region Set Exact 1 ");

               let Some(region_map_data) =
                                &mut region_map_data_res.regions_data_map
                            else {
                                warn!("regions data map is null ");
                                continue
                            }; 

              let mut region_index_map_changed = false;

            let brush_hardness = &ev.brush_hardness;
            
               
                    match &ev.tool {
                        EditingTool::SetRegionMap { region_index } => {
                             


                           

                                let tool_coords: &Vec2 = &ev.coordinates;

                                let tool_coords_local: &Vec2 = &ev.coordinates;

                          
                                //need to make an array of all of the data indices of the terrain that will be set .. hm ?
                                let img_data_length = region_map_data.len();

                             

                                let radius_clone = radius.clone();

                                info!("Region Set Exact 2 ");

                                match brush_type {
                                    BrushType::SetExact => {
                                        for x in 0..img_data_length {
                                            for y in 0..img_data_length {
                                                let local_coords = Vec2::new(x as f32, y as f32);

                                                 // info!("local_coords {:?} ", local_coords);

                                                let hardness_multiplier = get_hardness_multiplier(
                                                    tool_coords_local.distance(local_coords),
                                                    radius_clone,
                                                    *brush_hardness,
                                                );
                                                let original_region_index = region_map_data[y][x];


                                                 //  info!("tool_coords_local {:?} ", tool_coords_local);


                                                if tool_coords_local.distance(local_coords)
                                                    < radius_clone
                                                {
                                                    let new_region_index = region_index.clone();


                                                    region_map_data[y][x] =
                                                        apply_hardness_multiplier(
                                                            original_region_index as f32,
                                                            new_region_index as f32,
                                                            hardness_multiplier,
                                                        )
                                                            as u8;
                                                    region_index_map_changed = true;

                                                   // info!("region_index_map_changed {:?} ",new_region_index);


                                                }
                                            }
                                        }
                                    }

                                    BrushType::Smooth => {
                                        for x in 0..img_data_length {
                                            for y in 0..img_data_length {
                                                let local_coords = Vec2::new(x as f32, y as f32);
                                                if tool_coords_local.distance(local_coords)
                                                    < *radius
                                                {
                                                    let hardness_multiplier =
                                                        get_hardness_multiplier(
                                                            tool_coords_local
                                                                .distance(local_coords),
                                                            radius_clone,
                                                            *brush_hardness,
                                                        );

                                                    let original_region_index = region_map_data[y][x];
                                                    // Gather heights of the current point and its neighbors within the brush radius

                                                    let new_region_index = ((average_height as f32
                                                        + original_region_index as f32)
                                                        / 2.0)
                                                        as u8;
                                                    region_map_data[y][x] =
                                                        apply_hardness_multiplier(
                                                            original_region_index as f32,
                                                            new_region_index as f32,
                                                            hardness_multiplier,
                                                        )
                                                            as u8;
                                                    region_index_map_changed = true;
                                                }
                                            }
                                        }
                                    }

                                     

                                    BrushType::EyeDropper => {
                                        // Check if the clicked coordinates are within the current chunk
                                         
                                            
                                            let x = tool_coords_local.x as usize;
                                            let y = tool_coords_local.y as usize;

                                            if x < img_data_length && y < img_data_length {
                                              

                                                let local_index_data = region_map_data[y][x];
                                                evt_writer.send(
                                                    RegionBrushEvent::EyeDropRegionIndex   {
                                                        region_index: local_index_data,
                                                    },
                                                );
                                            }
                                        
                                    }
                                }

                              
                            }
                       
                     



                    } //match
                
              if region_index_map_changed {

                             

                                   region_data_event_writer.send(

                                         RegionDataEvent::RegionMapNeedsReloadFromResourceData
                                    );

                                }
    }
}

fn get_hardness_multiplier(pixel_distance: f32, brush_radius: f32, brush_hardness: f32) -> f32 {
    // Calculate the distance as a percentage of the radius
    let distance_percent = pixel_distance / brush_radius;
    let adjusted_distance_percent = f32::min(1.0, distance_percent); // Ensure it does not exceed 1

    // Calculate the fade effect based on brush hardness
    // When hardness is 0, this will linearly interpolate from 1 at the center to 0 at the edge
    // When hardness is between 0 and 1, it adjusts the fade effect accordingly
    let fade_effect = 1.0 - adjusted_distance_percent;

    // Apply the brush hardness to scale the fade effect, ensuring a minimum of 0
    f32::max(
        0.0,
        fade_effect * (1.0 + brush_hardness) - (adjusted_distance_percent * brush_hardness),
    )
}

fn apply_hardness_multiplier(
    original_height: f32,
    new_height: f32,
    hardness_multiplier: f32,
) -> f32 {
    original_height + (new_height - original_height) * hardness_multiplier
}



//move this to region_map.rs ? 

// outputs as R16 grayscale
pub fn save_region_index_map_to_disk<P>(
    region_map_data: &RegionMapU8, // Adjusted for direct Vec<Vec<u16>> input
    save_file_path: P,
) where
    P: AsRef<Path>,
{
    let region_map_data = region_map_data.clone();

    let height = region_map_data.len();
    let width = region_map_data.first().map_or(0, |row| row.len());

    let file = File::create(save_file_path).expect("Failed to create file");
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width as u32, height as u32);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight); // Change to 8-bit depth
    let mut writer = encoder.write_header().expect("Failed to write PNG header");

    // Flatten the Vec<Vec<u8>> to a Vec<u8> for the PNG encoder
    let buffer: Vec<u8> = region_map_data.iter().flatten().cloned().collect();

    // Write the image data
    writer
        .write_image_data(&buffer)
        .expect("Failed to write PNG data");
}
