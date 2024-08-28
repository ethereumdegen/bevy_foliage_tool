use crate::foliage_chunk::{FoliageChunkNeedsRebuild,FoliageChunk};
use crate:: FoliageConfigResource ;
use bevy::prelude::*;

use bevy::utils::HashMap;
use serde::{Serialize,Deserialize};





pub(crate) fn foliage_layer_plugin(app: &mut App ) {
    app
    		

    	.add_systems(Update, unpack_foliage_layer_data_components)
    	;



  }




/// There is one foliage layer for each foliage index and it is the parent to many chunks 
#[derive(Component,Clone,Debug,Serialize,Deserialize)]
pub struct FoliageLayer {

	pub dimensions: IVec2,
	pub chunk_rows: usize ,
	pub foliage_index: usize, //refer to the config 

	//pub density_map: FoliageDensityMapU8,
	//pub base_height_map: FoliageBaseHeightMapU8, 

	 
}

 
/// A component typically on the FoliageLayer entity
#[derive(Component,Debug,Clone,Serialize,Deserialize )]
pub struct FoliageBaseHeightMapU8(pub Vec<Vec<u8>>);

impl FoliageBaseHeightMapU8 {

	pub fn new(dimensions: IVec2) -> Self {

		let (width, height) = (dimensions.x as usize, dimensions.y as usize);
        let map = vec![vec![0u8; width]; height];
        Self(map)

	}

}


/// A component typically on the FoliageLayer entity
#[derive(Component,Debug,Clone,Serialize,Deserialize )]
pub struct FoliageDensityMapU8(pub Vec<Vec<u8>>);

impl FoliageDensityMapU8 {

	pub fn new(dimensions: IVec2) -> Self {

			let (width, height) = (dimensions.x as usize, dimensions.y as usize);
        let map = vec![vec![0u8; width]; height];
        Self(map)

	}

}



// this is just used for saving 
#[derive( Component, Clone,Debug,Serialize,Deserialize)]
pub struct FoliageLayerData {
	
	pub foliage_index: usize, //refer to the config 

	 
	pub density_map: FoliageDensityMapU8,
	pub base_height_map: FoliageBaseHeightMapU8, 

	 
}

impl FoliageLayerData{

	pub fn new(foliage_index: usize, boundary_dimensions: IVec2 ) -> Self {

		Self {

			foliage_index,
		//	dimensions: boundary_dimensions.clone(), 
			density_map: FoliageDensityMapU8::new( boundary_dimensions )  ,
			base_height_map : FoliageBaseHeightMapU8::new( boundary_dimensions ) 
		}

	}

}


fn unpack_foliage_layer_data_components(

   mut commands: Commands, 

   foliage_layer_data_query: Query<(Entity, &FoliageLayerData)>,

    foliage_config_resource: Res<FoliageConfigResource>,
   // foliage_types_resource: Res<FoliageTypesResource>



){

	// this is a beautiful abomination 

	for (foliage_layer_entity, foliage_layer_data) in foliage_layer_data_query.iter(){

 

      	let density_map_data = &foliage_layer_data.density_map;
      	let base_height_map_data = &foliage_layer_data.base_height_map;
      	let foliage_index = &foliage_layer_data.foliage_index; 
       
    		let foliage_config = &foliage_config_resource.0;
    		let chunk_rows = foliage_config.chunk_rows;

    		let dimensions = foliage_config.boundary_dimensions; 

       let Some(mut foliage_layer_cmd)	= commands.get_entity(foliage_layer_entity) else {continue};
          	info!("unpacking foliage layer data ");
   
    	


      		foliage_layer_cmd
      		.remove::<FoliageLayerData>()
		      
      		/*.with_children( |child_builder| 
	      		{
	      			  for (layer_index,layer_data) in layers_data_array {
	      						   let layer_entity = child_builder.spawn( 
	      							 	(
	      									SpatialBundle::default() , 
	      									layer_data.clone()
	      								)

	      							   ).id();


	      						   foliage_layer_entities_map.insert( *layer_index , layer_entity) ;
 						}
	      		}
      		 )*/

      		.insert(FoliageLayer {	
      			dimensions:  dimensions.clone() ,
      			chunk_rows: chunk_rows.clone(),
      			foliage_index : foliage_index.clone() ,

      			 })
      		.insert(density_map_data.clone())
      		.insert(base_height_map_data.clone())
      		.insert(Name::new("foliage_layer"))
      		.despawn_descendants()
      		 ; 

      		 //spawn foliage chunks ? 

      		 for x in 0..chunk_rows {

      		 	for y in 0..chunk_rows {


      		 	   	let chunk_offset = IVec2::new(x as i32,y as i32);


				        let boundary_dimensions = &dimensions;
				        

				        let chunk_dimensions = IVec2::new( 
				            boundary_dimensions.x /  chunk_rows as i32 , 
				            boundary_dimensions.y /  chunk_rows as i32  
				        );


      		 		let chunk_translation = Vec3::new(
      		 			(chunk_offset.x * chunk_dimensions.x) as f32,
      		 			0.0,
      		 			(chunk_offset.y * chunk_dimensions.y) as f32,
      		 			);

      		 		let _new_chunk = commands.spawn(
      		 			SpatialBundle {
      		 				transform: Transform::from_translation(chunk_translation),
      		 				..default()
      		 			}

      		 		 )
      		 		.insert(FoliageChunk {
      		 			chunk_offset 
      		 		})
      		 		.insert(FoliageChunkNeedsRebuild)
      		 		.insert(Name::new("foliage_chunk"))
      		 		.set_parent(  foliage_layer_entity  )
      		 		.id(); 



      		 	}

      		 }

 



	}









}