use crate::foliage_chunk_layer::FoliageChunkLayer;
//use crate::foliage_layer::FoliageBaseNormalMapU16;
use crate::foliage_config::FoliageConfigResource;
use crate::foliage_density::FoliageDensityResource;
//use crate::foliage_layer::FoliageBaseHeightMapU16;
//use crate::foliage_layer::FoliageDensityMapU8;
//use crate::foliage_layer::FoliageLayer;
//use crate::foliage_layer::FoliageLayerSystemSet;
//use crate::foliage_scene::FoliageSceneData;

use rand::Rng;
use bevy::utils::HashMap ;

use crate::foliage_proto;
use crate::foliage_proto::FoliageProto;
use crate::foliage_proto::FoliageProtoBundle;
use crate::foliage_viewer::FoliageViewer;
use crate::noise::NoiseResource;
use crate::FoliageTypesResource;
use bevy::prelude::*;

pub(crate) fn foliage_chunks_plugin(app: &mut App) {
    app

    .register_type::<FoliageChunkActive>()
    .add_systems(
         Update,
        (

     //   compute_normals_from_height, 
       handle_chunk_changed, 
       


         ).chain(), // .in_set(FoliageChunkSystemSet)
                                                                  // .before(FoliageLayerSystemSet),
    );


      app.add_systems(
          Update,
        (

    
       
         update_chunk_active,
         update_chunk_visible  


         ).chain()  .run_if( 
          resource_exists::<FoliageConfigResource>
          .and( resource_exists::<FoliageTypesResource>  )
          .and( resource_exists::< FoliageDensityResource > )   )

         , // .in_set(FoliageChunkSystemSet)
                                                                  // .before(FoliageLayerSystemSet),
    );

       app.add_systems(
         PostUpdate,
        (

    
        handle_chunk_rebuilds, 


         ).chain()  .run_if( 
          resource_exists::<FoliageConfigResource>
          .and( resource_exists::<FoliageTypesResource>  )
          .and( resource_exists::< FoliageDensityResource > )   )

         , // .in_set(FoliageChunkSystemSet)
                                                                  // .before(FoliageLayerSystemSet),
    );

}

#[derive(SystemSet, Clone, Debug, Hash, PartialEq, Eq)]
pub struct FoliageChunkSystemSet;







#[derive(Component)]
#[require( FoliageChunkLayerChildren , FoliageChunkActive )]
pub struct FoliageChunk {
    pub chunk_id: u32 ,
}


#[derive(Component)]
pub struct FoliageHeightMapData ( pub Vec<Vec<u16>> );

#[derive(Component)]
pub struct FoliageDimensionsData ( pub IVec2 );



#[derive(Component)]
pub struct FoliageDataSource ( pub Entity );



#[derive(Component,Default)]
pub struct FoliageChunkLayerChildren  ( pub HashMap< usize, Entity  > );




#[derive(Component,Default,PartialEq, Eq, Reflect  )]
pub enum FoliageChunkActive {
    #[default]
    Deactivated,
    Activated, 

}





#[derive(Component)]
pub struct FoliageChunkNeedsRebuild;


/*

terrain chunks are 128 x 128 ! 

*/

fn update_chunk_active(
    foliage_viewer_query: Query<Entity, With<FoliageViewer>>,

    global_xform_query: Query<&GlobalTransform>,

    mut foliage_chunk_query: Query<(Entity, &FoliageChunk,   &mut FoliageChunkActive )>,

    foliage_config_resource: Res<FoliageConfigResource>,
) {

  
    let Some(foliage_viewer_entity) = foliage_viewer_query.get_single().ok() else {
        return;
    };

    let Some(viewer_xform) = global_xform_query.get(foliage_viewer_entity).ok() else {
        return;
    };

    let viewer_translation = viewer_xform.translation();

    let foliage_config = &foliage_config_resource.0;

    let Some(max_render_distance) = foliage_config.render_distance else {
        return;
    };
        
    for (foliage_chunk_entity, _foliage_chunk,  mut chunk_active ) in foliage_chunk_query.iter_mut() {
        let Some(chunk_xform) = global_xform_query.get(foliage_chunk_entity).ok() else {
            continue;
        };
        let chunk_translation = chunk_xform.translation();

        let chunk_dimensions = Vec3::new(128.0, 0.0, 128.0);

        let chunk_center_translation = chunk_translation + chunk_dimensions / 2.0;

        let distance = chunk_center_translation.distance(viewer_translation);
           
        if distance <= max_render_distance {
         //    *visibility = Visibility::Inherited;
             if *chunk_active != FoliageChunkActive::Activated {

               
                * chunk_active = FoliageChunkActive::Activated; 

            }


        } else {
            //*visibility = Visibility::Hidden;
              if *chunk_active !=  FoliageChunkActive::Deactivated {

                     
                    * chunk_active = FoliageChunkActive::Deactivated; 

              }
        }
    }
}


fn update_chunk_visible(


    mut foliage_chunk_query: Query<(Entity, &FoliageChunk,   &  FoliageChunkActive, &mut Visibility ) , Changed< FoliageChunkActive> > 

 ) {


     for (_foliage_chunk_entity, _foliage_chunk,    chunk_active , mut visibility ) in foliage_chunk_query.iter_mut() { 


        match chunk_active {

            FoliageChunkActive::Activated => *visibility = Visibility::Inherited,

            FoliageChunkActive::Deactivated => *visibility = Visibility::Hidden 

        }




     }





}


#[derive(Component)]
pub struct ForceRebuildFoliageChunk ;
 



fn handle_chunk_changed(
    mut commands: Commands,
  
    chunk_query: Query<(

        Entity,  

        &FoliageChunk  ,

       

    ) , Or<(  Changed<FoliageDimensionsData > , Changed<FoliageHeightMapData>, Changed<FoliageChunkActive> ) >>, //chunks parent should have terrain data

  
) {



    /*
        
        insert ForceChunkRebuild !! which deletes all children (chunk layers) and re-creates them ! 

    */

    for (chunk_entity,  _foliage_chunk  ) in chunk_query.iter(){

         if let Some(mut cmd) = commands.get_entity( chunk_entity ){

            println!(" Changed<FoliageDimensionsData > , Changed<FoliageHeightMapData> ");
            cmd.insert(  ForceRebuildFoliageChunk );
         }


    }
   






}

fn handle_chunk_rebuilds(

    mut commands: Commands,
  
    chunk_query: Query<(

        Entity,  

        &FoliageChunk  ,

      
        &FoliageHeightMapData,
        &FoliageDimensionsData, 

        &FoliageChunkActive, 

 
    ) ,    With<ForceRebuildFoliageChunk >  > , //chunks parent should have terrain data


    foliage_density_resource: Res<  FoliageDensityResource >,
  
     

) {


    // delete all chunk_layer children 

  /*
      for (chunk_entity,  foliage_chunk, heightmap, dimensions , chunk_active  ) in chunk_query.iter(){

          if let Some( mut cmd ) = commands.get_entity( chunk_entity ){
 
                cmd.despawn_descendants();

                cmd.remove::<ForceRebuildFoliageChunk>() ;






          }


      }*/


    
    // let foliage_density_map = foliage_density_resource.0 ; 

      //recreate them !! 






              for (chunk_entity,  foliage_chunk, heightmap, dimensions, chunk_active ) in chunk_query.iter(){


                    if let Some( mut cmd ) = commands.get_entity( chunk_entity ){
 
                            cmd.despawn_descendants();

                            cmd.remove::<ForceRebuildFoliageChunk>() ;
 
                      }


                if  chunk_active == & FoliageChunkActive::Deactivated {continue};


                 for (layer_index,foliage_layer_density_map) in   foliage_density_resource.0.iter() {


                        commands.spawn(

                            (
                                Name::new( format!("Foliage Chunk Layer {}", layer_index ) ),
                                FoliageChunkLayer {
                                    //chunk_id: foliage_chunk.chunk_id,
                                    layer_index: *layer_index 

                                } ,
                                Visibility::default(),
                                Transform::default() 


                                //insert density map !? 

                            )


                         ).set_parent(chunk_entity);




                         /*  if let Some( mut cmd ) = commands.get_entity( chunk_entity ){
                              cmd.set_parent( foliage_scene_root_entity ); 

                                
                          } */


                      }





    }









}

/*
fn decode_normal(encoded_normal: u16) -> Vec3 {

     if encoded_normal == 0 {
        // Special case for a flat surface / missing data 
        return Vec3::Y;
    }


    let x = ((encoded_normal >> 8) & 0xFF) as f32 / 255.0 * 2.0 - 1.0; // Extract high 8 bits for x and scale to [-1, 1]
    let z = (encoded_normal & 0xFF) as f32 / 255.0 * 2.0 - 1.0;         // Extract low 8 bits for z and scale to [-1, 1]
    
    Vec3::new(x, 1.0, z).normalize()
}



// put this in a thread pattern ?  kinda cpu intensive
fn compute_normals_from_height(

    mut commands: Commands, 

      foliage_layer_query: Query<(
       // &mut FoliageBaseNormalMapU16,
       Entity,  &FoliageBaseHeightMapU16,
    ), (Changed<FoliageBaseHeightMapU16> , Without<FoliageBaseNormalMapU16> ) >,


) {
    for ( chunk_entity,  height_map) in foliage_layer_query.iter () {
        let height_data = &height_map.0;
        let height = height_data.len();
        let width = height_data[0].len();

        let mut normals = vec![vec![0u16; width]; height];

        for y in 1..(height - 1) {
            for x in 1..(width - 1) {
                let height_left = height_data[y][x - 1] as f32;
                let height_right = height_data[y][x + 1] as f32;
                let height_up = height_data[y - 1][x] as f32;
                let height_down = height_data[y + 1][x] as f32;

                // Compute slope in X and Z directions
                let slope_x = (height_right - height_left) / 2.0;
                let slope_z = (height_down - height_up) / 2.0;

                // Calculate the normal
                let normal = Vec3::new(-slope_x, 1.0, -slope_z).normalize();

                // Encode the normal into a u16 format
                let encoded_normal = encode_normal(normal);

                normals[y][x] = encoded_normal;
            }
        }

     

        if let Some(mut cmd) = commands.get_entity(chunk_entity){
              info!("built  normal map ");
            cmd.insert((

                FoliageBaseNormalMapU16 ( normals ),

                FoliageChunkNeedsRebuild 

                ));
        }
    }
}

fn encode_normal(normal: Vec3) -> u16 {
    // Convert a normal vector to an encoded u16 representation
    let x = ((normal.x.clamp(-1.0, 1.0) + 1.0) / 2.0 * 255.0).round() as u16;
    let z = ((normal.z.clamp(-1.0, 1.0) + 1.0) / 2.0 * 255.0).round() as u16;

    (x << 8) | z
}*/
