

use bevy::prelude::* ;

use crate::foliage_types::FoliageDefinition;

#[derive(Component,Debug,Clone)]
pub struct FoliageProto {

	pub foliage_definition: FoliageDefinition

}