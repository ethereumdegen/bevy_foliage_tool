use crate::foliage_layer::FoliageLayer;
use crate::foliage_layer::FoliageDensityMapU8;
use crate::foliage_layer::FoliageBaseHeightMapU8;
use bevy::prelude::*;



pub(crate) fn foliage_chunks_plugin(app: &mut App) {
    app
    	
        .add_systems(Update, handle_chunk_rebuilds)
    	;



    }



#[derive(Component)]
pub struct FoliageChunk {

    pub chunk_offset: IVec2 ,
}




#[derive(Component)]
pub struct FoliageChunkNeedsRebuild;




fn handle_chunk_rebuilds(

    mut commands: Commands , 

    chunks_query: Query< (Entity,&FoliageChunk,&Parent), With<FoliageChunkNeedsRebuild> >,

    foliage_layer_query: Query<(&FoliageLayer, &  FoliageDensityMapU8, &FoliageBaseHeightMapU8)>, //chunks parent should have terrain data
    

 


){


    for (chunk_entity, foliage_chunk, parent) in chunks_query.iter(){

        let parent_entity = parent.get();


        let Some( (foliage_layer,foliage_density_map_comp, foliage_base_height_comp) )
        = foliage_layer_query.get(parent_entity).ok() else {continue};



        if let Some(mut cmd) = commands.get_entity(chunk_entity){

            cmd
            .despawn_descendants()
            .remove::<FoliageChunkNeedsRebuild>();

        }

        let density_map = &foliage_density_map_comp.0;
        let base_height_map = &foliage_base_height_comp.0;

        let boundary_dimensions = &foliage_layer.dimensions;
        let chunk_rows = &foliage_layer.chunk_rows; 

        let chunk_dimensions = IVec2::new( 
            boundary_dimensions.x / *chunk_rows as i32 , 
            boundary_dimensions.y / *chunk_rows as i32  
        );

        let chunk_offset = &foliage_chunk.chunk_offset;

        let chunk_data_offset = IVec2::new(
            chunk_offset.x * chunk_dimensions.x,
            chunk_offset.y * chunk_dimensions.y
        ) ;



        for x in chunk_data_offset.x .. chunk_data_offset.x + chunk_dimensions.x { 

            for y in chunk_data_offset.y .. chunk_data_offset.y + chunk_dimensions.y {

                let chunk_density_at_point = density_map[y as usize][x as usize];
                let chunk_base_height_at_point =  base_height_map[y as usize][x as usize];

                if chunk_density_at_point <= 0 {continue};

                info!("chunk_density_at_point {:?}", chunk_density_at_point);

                //combine with noise here ,  then spawn foliage    proto  


                
            } 

        }


    }





}