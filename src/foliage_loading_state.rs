

use bevy::prelude::*;



pub(crate) fn foliage_loading_state_plugin(app: &mut App) {
    app
    	.init_state::<FoliageLoadingState>()

    	;



  }




#[derive(Hash,Eq,PartialEq,Clone,Debug,States,Default)]
pub enum FoliageLoadingState {
	#[default]
	Init,
	Complete
}