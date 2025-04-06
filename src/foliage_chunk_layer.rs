use bevy::ecs::relationship::Relationship;
use crate::foliage_scene::FoliageRoot;
use crate::foliage_scene::FoliageScene;
use crate::foliage_density::FoliageDensityMapsComponent;
use crate::foliage_proto::FoliageProtoBundle;
use crate::foliage_types::FoliageTypesManifest;
use rand::Rng;
use crate::foliage_chunk::FoliageHeightMapData;
use crate::foliage_chunk::FoliageDimensionsData;
use crate::foliage_density::FoliageDensityMapU8;
 
use crate::noise::NoiseResource;
 
use crate::foliage_chunk::{FoliageChunk, FoliageChunkNeedsRebuild};
 
use bevy::prelude::*;

use bevy::platform_support::collections::hash_map::HashMap;
use serde::{Deserialize, Serialize};

pub(crate) fn foliage_chunk_layer_plugin(app: &mut App) {
    app 

     .add_systems(
         Update,
        (
            build_chunk_layers,
            //propogate_density_updates,
            //handle_foliage_layer_rebuild,
        )
            .chain()   )  ; 
}




#[derive(Component)] 
pub struct FoliageChunkLayer {

   // pub chunk_id: u32,  //get from parent !
    pub layer_index: usize, 

 }



/// only rebuild if Visible?   could be an optimization...   but also need to rebuild when vis changes then 

fn build_chunk_layers (


    //parent must be a foliage chunk! 
        chunk_layer_query: Query  < ( Entity, &FoliageChunkLayer  , &ChildOf  ) , Added< FoliageChunkLayer > >,

        chunk_query: Query< ( &FoliageChunk, &FoliageHeightMapData, &FoliageDimensionsData ) > ,
 
         global_xform_query: Query  <   &GlobalTransform     >,


       //  foliage_density_resource: Res<FoliageDensityResource>,
       //   foliage_types_resource: Res<FoliageTypesResource>,

     //   foliage_config_resource: Res<FoliageConfigResource>,

     foliage_root_query: Query< (  &FoliageRoot,&FoliageScene,  &  FoliageDensityMapsComponent, &FoliageTypesManifest ) >,


  //  foliage_scene_data_resource: Res<FoliageSceneData > ,  //foliage layer data !!! 

    noise_resource: Res<NoiseResource>,
    image_assets: Res<Assets<Image>>,

    mut commands: Commands, 

){

    for  (chunk_layer_entity, chunk_layer, chunk_layer_parent )  in chunk_layer_query.iter(){

       let Ok( (foliage_root, foliage_scene,   foliage_density_map, foliage_types  ) ) = foliage_root_query.get_single() else {
            warn!("no single foliage root found ");
            continue ; 
        };



        let Ok(  ( foliage_chunk, height_map_data, dimensions_data )  ) = chunk_query.get( chunk_layer_parent.get() ) else {


            continue; 
        };   


        let global_xform = global_xform_query.get( chunk_layer_entity );

       // println!("build foliage chunk layer ! ");


       // if let Some(global_xform) = global_xform .ok() {

        //    println!( "global xform {:?}", global_xform.translation() );

       // }

     let chunk_id = foliage_chunk.chunk_id; // need this for height data 
        let layer_index = chunk_layer.layer_index; // need this for density data 


        let Some( foliage_definition_for_layer) = foliage_types.foliage_definitions.get( layer_index ) else {

             warn!( "no  foliage_definition_for_layer to build chunk layer " );
            continue;

        };


            //this data is the density for the ENTIRE LAYER 
          let Some( layer_density_map) = &foliage_density_map.0.get( &layer_index ) else {

            warn!( "no layer density map to build chunk layer " );
            continue;

          };







          let chunk_height_data = & height_map_data;   // this data is specific to the chunk!! 
        

             let noise_texture_handle = &noise_resource.density_noise_texture;
              let noise_texture = image_assets
            .get(noise_texture_handle)
            .expect("no noise texture");


            let chunk_rows = foliage_scene.chunk_rows as i32 ;
            let height_scale = foliage_scene.height_scale; 


            let layer_dimensions = dimensions_data.0; 
            let chunk_dimensions = IVec2::new( layer_dimensions.x / 1  ,layer_dimensions.y / 1 ) ;





              let chunk_density_map = layer_density_map.get_sub_section_by_chunk_id(
                chunk_id, 
                chunk_rows as u32, 
                chunk_dimensions
              ); 

           //   info!( " chunk_dimensions {}", chunk_dimensions );

            /*
    
                a terrain layer is 1024 x 1024 

                a terrain chunk is 128 x 128 



                a foliage layer is 1024 x 1024 
                a foliage chunk is 128 x 128   (8 by 8 ) 
            */
            // a terrain chunk is 


            // i dont understand this yet 


            let step_by  =  2 ; 

             for x in (0..chunk_dimensions.x ).step_by(  step_by  as usize )  {
                 for y in (0..chunk_dimensions.y).step_by( step_by  as usize )   {


                   let transform_opt =   get_foliage_node_spawn_using_noise( 
                       // chunk_layer_entity, // parent

                        x as usize,
                        y as usize,


                        chunk_height_data,
                        &chunk_density_map,

                        noise_texture,

                        height_scale

                    );


                   if let Some(xform) = transform_opt {

                         //  info!(  "spawning foliage proto at {:?}" , xform);

                            commands.spawn(



                                (
                                    xform,  


                                    //maybe impl from_world so i can pass in the index  ? 
                                    FoliageProtoBundle::new ( foliage_definition_for_layer.clone() ),

                                    Name::new(format!("foliage_node_{}_{}", x, y)),
                                    Visibility::default(),

                                )

                            ).set_parent( chunk_layer_entity );



                   }

                   /*else {

                    info!( "not spawning a foliage proto " )

                   }*/

               /* let data_x_index = x + chunk_data_offset.x;
                let data_y_index = y + chunk_data_offset.y;

                let chunk_density_at_point =
                    density_map[data_y_index as usize][data_x_index as usize];
                let chunk_base_height_at_point = base_height_map.as_ref().map( |m: &&Vec<Vec<u16>>| m[data_y_index as usize][data_x_index as usize] ).unwrap_or( 0 );
           
                let chunk_base_normal_at_point:u16 = base_normal_map.as_ref().map( |m: &&Vec<Vec<u16>> | m[data_y_index as usize][data_x_index as usize] ).unwrap_or( 0 );

                if chunk_density_at_point <= 0 {
                    continue;
                };

                let chunk_density_scaled = chunk_density_at_point as f32 / max_chunk_density;

                //this is probably wrong
                let noise_tex_data_index = data_y_index * chunk_dimensions.y + data_x_index;

                let noise_sample_at_point = noise_texture.data[noise_tex_data_index as usize];

                let noise_sample_scaled = noise_sample_at_point as f32 / max_noise_value;
                //  info!("noise_sample_at_point {} {}", noise_sample_at_point , noise_sample_scaled);

                if chunk_density_scaled < noise_sample_scaled {
                    continue;
                }*/

                }
            }


    }




}


/*

This may or may not spawn a foliage node at (x,y) !  Depending on the noise and the density . 

*/
fn get_foliage_node_spawn_using_noise(
   // parent_entity: Entity,
    x: usize,
    y: usize,
    chunk_height_data: &FoliageHeightMapData,
    chunk_density_map: &FoliageDensityMapU8,
    noise_texture: &Image,
    height_scale: f32,
) -> Option< Transform > {
   
        let max_chunk_density = 255.0;
        let max_noise_value = 255.0;
        
        // Get density at the current point
        let density_at_point = chunk_density_map.0[y][x];
        
        // If density is zero, don't spawn anything
       
        
        // Get height at the current point
        let height_at_point = chunk_height_data.0[y][x] ;

         
        
        // Scale density to 0.0-1.0 range
          let density_scaled =  density_at_point as f32   / max_chunk_density ;


          let Some( noise_texture_data )  = &noise_texture.data else {
            return None ;
          }  ;

   //  let density_scaled = 0.9 ; 
        
        // Sample noise texture for this position
        // This calculation may need adjustment based on your texture coordinates
        let noise_tex_data_index = (y * 256 + x) as usize % noise_texture_data.len();
        let noise_sample_at_point = noise_texture_data[noise_tex_data_index];
        let noise_sample_scaled = noise_sample_at_point as f32 / max_noise_value;
        
        // Skip if density is less than noise (for natural distribution)
        if density_scaled < noise_sample_scaled {

            //   println!( " density, noise  {} {} {} ", density_at_point, density_scaled, noise_sample_scaled );
            return None;
        }
        
        // Generate random offsets for natural variation
        let mut rng = rand::thread_rng();
        let random_float_x: f32 = rng.r#gen::<f32>() - 0.5;
        let random_float_y: f32 = rng.r#gen::<f32>() - 0.5; // for rotation in radians
        let random_float_z: f32 = rng.r#gen::<f32>() - 0.5;
        
        // Calculate foliage position with some noise-based offset
        let foliage_offset = Vec3::new(
            noise_sample_scaled, 
            0.0, 
            1.0 - noise_sample_scaled
        ) * 2.0;
        
        // Calculate final translation
        let foliage_translation = Vec3::new(
            x as f32 , // + foliage_offset.x + random_float_x,
            height_at_point as f32 * height_scale,
            y as f32 , // + foliage_offset.z + random_float_z,
        );
        
        // Optional: Get normal from height map for better placement
        let normal = get_normal_from_height_data( chunk_height_data , height_scale , x, y ).unwrap_or(Vec3::Y);
        
        // Create rotation based on normal and add some randomness
        let normal_rotation = Quat::from_rotation_arc(Vec3::Y, normal);
      //  let y_rotation = Quat::from_rotation_y(
     //       noise_sample_scaled * std::f32::consts::PI + random_float_y
      //  );
        let final_rotation = normal_rotation; //* y_rotation;
        
        // Spawn the foliage entity
        
        Some( Transform::from_translation(foliage_translation).with_rotation(final_rotation)   )

    
}



fn get_normal_from_height_data(
    height_data: &FoliageHeightMapData,
    height_scale: f32, 
    x: usize,
    y: usize,
) -> Option<Vec3> {
     let height_map = &height_data.0;
    let rows = height_map.len();
    let cols = height_map[0].len();
    
    // Check bounds - return default normal if we're at the edge
    if y == 0 || y >= rows - 1 || x == 0 || x >= cols - 1 {
        return Some(Vec3::Y); // Default to straight up normal for edges
    }
        
   // let height_max = 65025; 


    // Get height values from the neighboring cells
    // Using a 3x3 grid centered at (x,y)
    let h_center = height_map[y][x] as f32;
    let h_left = height_map[y][x - 1] as f32;
    let h_right = height_map[y][x + 1] as f32;
    let h_up = height_map[y - 1][x] as f32;
    let h_down = height_map[y + 1][x] as f32;
    
    // Grid spacing (distance between adjacent points)
    let grid_spacing = 1.0;
    
    // Calculate partial derivatives using central difference method
    // This gives more accurate results than using forward or backward differences
    let dx = (h_right - h_left) / (2.0 * grid_spacing);
    let dz = (h_down - h_up) / (2.0 * grid_spacing);
    
    // For a heightmap, the normal vector is perpendicular to the surface tangent
    // For a surface defined by y = f(x,z), the normal is (-df/dx, 1, -df/dz)
    // But we need to scale the slopes to avoid overly aggressive rotations
    
    // Scale factor to dampen the slope influence
    // Higher values = gentler slopes/rotations
    let slope_scale_factor = 1.0 / height_scale; //  1000.0;    // inverse of height scale !! 
    
    let normal = Vec3::new(
        -dx / slope_scale_factor, 
        1.0, 
        -dz / slope_scale_factor
    );
    
    // Normalize to unit length
    Some(normal.normalize())
}



      /*  commands
            .spawn((
                Transform::from_translation(foliage_translation).with_rotation(final_rotation),
                // You'll need to adapt this to your actual component bundle
                FoliageProtoBundle::new_from_density(density_scaled, noise_sample_scaled),
                Name::new(format!("foliage_node_{}_{}", x, y)),
                Visibility::default(),
            ))
            .set_parent(parent_entity);*/

 