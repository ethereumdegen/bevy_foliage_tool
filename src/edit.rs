 
 
 
use crate::foliage_scene::{FoliageRoot, FoliageScene};
use crate::foliage_density::FoliageDensityMapsComponent;
//use crate::FoliageConfigResource;
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

use bevy::prelude::*;

use core::fmt::{self, Display, Formatter};

use bevy::platform::collections::hash_map::HashMap;

//use crate::foliage::{FoliageDataEvent,    FoliageData    };
 

use anyhow::{Context, Result};

use serde::{Deserialize, Serialize};
use serde_json;

use rand::Rng;

use core::cmp::{max, min};

pub(crate) fn bevy_foliage_edits_plugin(app: &mut App) {
    app.add_event::<EditFoliageEvent>()
        .add_event::<FoliageCommandEvent>()
        .add_event::<FoliageBrushEvent>()


      .add_systems(Update, 

        (apply_tool_edits, apply_command_events).chain() 




        )


       ;
}

#[derive(Debug, Clone)]
pub enum EditingTool {
    // SetFoliageIndex { foliage_index: u8 },        // height, radius, save to disk
    SetFoliageDensity { foliage_index: u8, density: u8 },
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
pub struct EditFoliageEvent {
    pub entity: Entity, //not used
    pub tool: EditingTool,
    pub radius: f32,
    pub brush_hardness: f32, //1.0 is full
    pub coordinates: Vec2,
    pub brush_type: BrushType,
}

#[derive(Event, Debug, Clone)]
pub enum FoliageBrushEvent {
    EyeDropFoliageDensity { density: u8 },
    //  EyeDropSplatMap { r: u8, g: u8, b: u8 },
}

#[derive(Event, Debug, Clone)]
pub enum FoliageCommandEvent {
    SaveAll,
}
 

pub fn apply_command_events(
    
     foliage_root_query: Query< (  &FoliageRoot,&FoliageScene,  &FoliageDensityMapsComponent ) >,
    //  foliage_density_map: Res < FoliageDensityResource >,
 
    //foliage_config_resource: Res<FoliageConfigResource>,

    mut ev_reader: EventReader<FoliageCommandEvent>,
) {
    for ev in ev_reader.read() {
        /*let Some((foliage_data, foliage_config)) = foliage_data_query
            .get_single().ok() else {continue};
        */

        match ev {
            FoliageCommandEvent::SaveAll => {
                //let file_name = format!("{}.png", chunk.chunk_id);
                // let asset_folder_path = PathBuf::from("assets/");



            for (foliage_root, foliage_scene_data, foliage_density_map  )   in foliage_root_query .iter(){
               // let foliage_config = &foliage_config_resource.0;



                let foliage_density_map_path = &foliage_scene_data.foliage_density_data_path ;



                /*
                        Need to save the density maps hashmap as a binary  ! 
                */

                if let Some( path) = foliage_density_map_path {

                       let saved =   foliage_density_map .save_to_disk(  path );


                       info!(  "Saved foliage density data {:?}" , saved  );
                   }

                }
 

            }
        }
    }

    //  Ok(())
}

pub fn apply_tool_edits(
    //mut  foliage_density_map: ResMut< FoliageDensityResource >,

 /*   mut foliage_layer_query: Query<(
        &FoliageLayer,
        &mut FoliageDensityMapU8,
       //&FoliageBaseHeightMapU16,
    )>, //chunks parent should have terrain data
*/

    //foliage_config_resource: Res<FoliageConfigResource>,


     mut foliage_root_query: Query< (  &FoliageRoot,&FoliageScene,  &mut FoliageDensityMapsComponent ) >,

    mut ev_reader: EventReader<EditFoliageEvent>,
    mut evt_writer: EventWriter<FoliageBrushEvent>,
) {
    for ev in ev_reader.read() {
        let tool_coords = ev.coordinates;
        let radius = ev.radius;
        let brush_hardness = ev.brush_hardness;
        let brush_type = &ev.brush_type;

       
        let tool_coords_local = tool_coords;



        let Ok( (foliage_root, foliage_scene, mut foliage_density_map ) ) = foliage_root_query.get_single_mut() else {
            warn!("no single foliage root found ");
            continue ; 
        };

      //  let foliage_config = &foliage_config_resource.0;

        let foliage_dimensions = &foliage_scene.boundary_dimensions;

      
        match &ev.tool {
            EditingTool::SetFoliageDensity {
                foliage_index,
                density: new_density,
            } => {



           


                    let Some( mut existing_foliage_layer_density_data ) = foliage_density_map.0.get_mut( &(*foliage_index as usize ) ) else {
                        warn!( "no foliage layer data to mutate ! " );
                        continue; 
                    };
 
                    

                        let density_data = &mut existing_foliage_layer_density_data.0;

                        match brush_type {
                            BrushType::SetExact => {
                                for x in 0..foliage_dimensions.x as usize {
                                    for y in 0..foliage_dimensions.y as usize {
                                        let local_coords = Vec2::new(x as f32, y as f32);
                                        let distance = tool_coords_local.distance(local_coords);

                                        if distance < radius {
                                            let hardness_multiplier = get_hardness_multiplier(
                                                distance,
                                                radius,
                                                brush_hardness,
                                            );
                                            let original_density = density_data[y][x];
                                            density_data[y][x] = apply_hardness_multiplier(
                                                original_density as f32,
                                                *new_density as f32,
                                                hardness_multiplier,
                                            )
                                                as u8;
                                        }
                                    }
                                }
                            }
                            BrushType::EyeDropper => {
                                let x = tool_coords_local.x as usize;
                                let y = tool_coords_local.y as usize;

                                if x < foliage_dimensions.x as usize
                                    && y < foliage_dimensions.y as usize
                                {
                                    let local_data = density_data[y][x];
                                    evt_writer.send(FoliageBrushEvent::EyeDropFoliageDensity {
                                        density: local_data,
                                    });
                                }
                            }
                            _ => warn!("Brush type not implemented!"),
                        }


 

 
            }
        }
        //  }
        // }
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
 