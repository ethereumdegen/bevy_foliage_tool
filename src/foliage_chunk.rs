use crate::foliage_layer::FoliageLayerSystemSet;
use crate::foliage_config::FoliageConfigResource;
use crate::FoliageTypesResource;
use crate::foliage_layer::FoliageLayer;
use crate::foliage_layer::FoliageDensityMapU8;
use crate::foliage_layer::FoliageBaseHeightMapU16;
use crate::foliage_proto;
use crate::foliage_proto::FoliageProto;
use bevy::prelude::*;



pub(crate) fn foliage_chunks_plugin(app: &mut App) {
    app
    	
        .add_systems(Update, 
            handle_chunk_rebuilds
            .in_set(FoliageChunkSystemSet)
            .before(FoliageLayerSystemSet)
            )
    	;



    }


#[derive(SystemSet,Clone,Debug,Hash,PartialEq,Eq)]
pub struct FoliageChunkSystemSet;

#[derive(Component)]
pub struct FoliageChunk {

    pub chunk_offset: IVec2 ,
}




#[derive(Component)]
pub struct FoliageChunkNeedsRebuild;




fn handle_chunk_rebuilds(

    mut commands: Commands , 

    chunks_query: Query< (Entity,&FoliageChunk,&Parent), With<FoliageChunkNeedsRebuild> >,

    foliage_layer_query: Query<(&FoliageLayer, &  FoliageDensityMapU8, &FoliageBaseHeightMapU16)>, //chunks parent should have terrain data
     
    foliage_types_resource: Res<FoliageTypesResource>,

    foliage_config_resource: Res<FoliageConfigResource> ,


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

        let foliage_index = &foliage_layer.foliage_index; 

        let foliage_types_manifest = &foliage_types_resource.0;
        let Some( foliage_type_definition ) = foliage_types_manifest.foliage_definitions.get( *foliage_index ) else {

            warn!("Cannot build foliage chunk - missing foliage type definition for index {}", foliage_index);
            continue;
        };


        let foliage_config = &foliage_config_resource.0;
        let height_scale = foliage_config.height_scale; 


        for x in 0 .. chunk_dimensions.x { 

            for y in  0.. chunk_dimensions.y {

                let data_x_index = x + chunk_data_offset.x;
                let data_y_index = y + chunk_data_offset.y;  

                let chunk_density_at_point = density_map[data_y_index as usize][data_x_index as usize];
                let chunk_base_height_at_point =  base_height_map[data_y_index as usize][data_x_index as usize];

                if chunk_density_at_point <= 0 {continue};

//                info!("chunk_density_at_point {:?}", chunk_density_at_point);

                //combine with noise here ,  then spawn foliage    proto  


                let foliage_proto_translation = Vec3::new( 
                    x  as f32, 
                    chunk_base_height_at_point  as f32 * height_scale , 
                    y  as f32 
                );

                commands.spawn( SpatialBundle {
                    transform: Transform::from_translation(foliage_proto_translation),  
                    ..default()
                } ).insert( FoliageProto {
                    foliage_definition: foliage_type_definition.clone()
                } )
                .insert( Name::new("foliage_proto"))
                .set_parent( chunk_entity  );


                
            } 

        }


    }





}