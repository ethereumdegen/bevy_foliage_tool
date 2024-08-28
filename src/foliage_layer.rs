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
	//pub foliage_index: usize, //refer to the config 

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
			density_map: FoliageDensityMapU8::new( boundary_dimensions )  ,
			base_height_map : FoliageBaseHeightMapU8::new( boundary_dimensions ) 
		}

	}

}


fn unpack_foliage_layer_data_components(

   mut commands: Commands, 

   foliage_layer_data_query: Query<(Entity, &FoliageLayerData)>,




){

	// this is a beautiful abomination 

	for (foliage_layer_entity, foliage_layer_data) in foliage_layer_data_query.iter(){

 

      	let density_map_data = &foliage_layer_data.density_map;
      	let base_height_map_data = &foliage_layer_data.base_height_map;

       
    

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

      		.insert(FoliageLayer {	 })
      		.insert(density_map_data.clone())
      		.insert(base_height_map_data.clone())


      		 ; 

      		 //spawn foliage chunks ? 

 



	}









}