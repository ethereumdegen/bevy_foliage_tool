use crate::foliage_scene::FoliageRoot;
use crate::foliage_scene::FoliageScene;
use crate::foliage_density::FoliageDensityMapsComponent;
use crate::foliage_chunk_layer::FoliageChunkLayer;
 

use rand::Rng;
use bevy::utils::HashMap ;

use crate::foliage_proto;
use crate::foliage_proto::FoliageProto;
use crate::foliage_proto::FoliageProtoBundle;
use crate::foliage_viewer::FoliageViewer;
use crate::noise::NoiseResource;
 
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


         ).chain() .run_if( any_with_component::<FoliageRoot> ) ) ;

       app.add_systems(
         PostUpdate,
        (

    
        handle_chunk_rebuilds, 


         ).chain().run_if( any_with_component::<FoliageRoot> )     );

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

       foliage_root_query: Query< (  &FoliageRoot, &FoliageScene,  &  FoliageDensityMapsComponent ) >,

    //foliage_config_resource: Res<FoliageConfigResource>,
) {

  
    let Some(foliage_viewer_entity) = foliage_viewer_query.get_single().ok() else {
        return;
    };

    let Some(viewer_xform) = global_xform_query.get(foliage_viewer_entity).ok() else {
        return;
    };


    let Ok( (foliage_root, foliage_scene,   foliage_density_map ) ) = foliage_root_query.get_single () else {
            warn!("no single foliage root found ");
            return  ; 
        };




    let viewer_translation = viewer_xform.translation();

  //  let foliage_config = &foliage_config_resource.0;

    let Some(max_render_distance) = foliage_scene.render_distance else {
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
            // Only insert the component if the entity exists and is valid
            if cmd.id() == chunk_entity {
                info!("Inserting ForceRebuildFoliageChunk for entity {:?}", chunk_entity);
                cmd.insert(ForceRebuildFoliageChunk);
            } else {
                warn!("Entity {:?} exists in query but not in world, skipping rebuild", chunk_entity);
            }
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


     
       foliage_root_query: Query< ( Entity,  &FoliageRoot, &FoliageScene,  &  FoliageDensityMapsComponent ) >,

     

) {





    let Ok( ( foliage_scene_root_entity, foliage_root, foliage_scene,   foliage_density_map ) ) = foliage_root_query.get_single () else {
            warn!("no single foliage root found ");
            return  ; 
        };

 




              for (chunk_entity, foliage_chunk, heightmap, dimensions, chunk_active) in chunk_query.iter() {
                    // Check if entity still exists and is valid before proceeding
                    if let Some(mut cmd) = commands.get_entity(chunk_entity) {
                        // Verify entity still exists in world
                        if cmd.id() == chunk_entity {
                            info!("Rebuilding foliage chunk {:?}", chunk_entity);
                            cmd.despawn_descendants();
                            cmd.remove::<ForceRebuildFoliageChunk>();
                        } else {
                            warn!("Entity {:?} exists in query but not in world, skipping rebuild process", chunk_entity);
                            continue;
                        }
                    } else {
                        warn!("Could not get entity {:?} for rebuild, likely already despawned", chunk_entity);
                        continue;
                    }

                    // Only proceed with rebuild if chunk is activated
                    if chunk_active == &FoliageChunkActive::Deactivated {
                        debug!("Skipping rebuild for deactivated chunk {:?}", chunk_entity);
                        continue;
                    }


                 for (layer_index,foliage_layer_density_map) in   foliage_density_map.0.iter() {


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


                 }



                    //this breaks shit ? 

                // if let Some( mut cmd ) = commands.get_entity( chunk_entity ){
                   //  cmd.set_parent( foliage_scene_root_entity ); 

                        
                  //} 







    }









}
 