

use crate::foliage_chunk::FoliageChunkSystemSet;
use crate::foliage_assets::FoliageAssetsState;
use bevy::prelude::* ;

use crate::{  foliage_assets::FoliageAssetsResource, foliage_types::FoliageDefinition};




pub(crate) fn foliage_proto_plugin(app: &mut App) {
    app
    	


    	.add_systems(Update, (attach_mesh_to_protos, attach_material_to_protos).chain() 
    		  .run_if(in_state(FoliageAssetsState::Loaded))
    		  .after( FoliageChunkSystemSet )
    		)
    	;



    }



#[derive(Component,Debug,Clone)]
pub struct FoliageProto {

	pub foliage_definition: FoliageDefinition

}



 

fn attach_mesh_to_protos(
	mut commands: Commands, 
	proto_query: Query<(Entity,&FoliageProto), Without<Handle<Mesh>>>,

	foliage_assets_resource: Res<FoliageAssetsResource> , 


) {

	for (proto_entity, proto) in proto_query.iter(){

		let foliage_def = &proto.foliage_definition; 

		let mesh_name = &foliage_def.mesh_name;

		if let Some(mesh_name) = mesh_name {

			let mesh_handle = foliage_assets_resource.foliage_mesh_handles.get( mesh_name );

			if let Some(mesh_handle) = mesh_handle {
				commands.entity(proto_entity).try_insert(  mesh_handle.clone() );
			}
			
		} 



	}


}


fn attach_material_to_protos(
	mut commands: Commands, 
	proto_query: Query<(Entity,&FoliageProto), Without<Handle<StandardMaterial>>>,

	foliage_assets_resource: Res<FoliageAssetsResource> , 


) {


	for (proto_entity, proto) in proto_query.iter(){

		let foliage_def = &proto.foliage_definition; 

		let material_name = &foliage_def.material_name;


			if let Some(material_name) = material_name {

			let material_handle = foliage_assets_resource.foliage_material_handles.get( material_name );

			if let Some(material_handle) = material_handle {
				commands.entity(proto_entity).try_insert(  material_handle.clone() );
			}
			
		}




	}

}