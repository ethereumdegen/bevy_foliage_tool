
 ## Bevy Foliage Tool

![foliage_tool](https://github.com/user-attachments/assets/6fd00873-a788-48d2-b71f-3b2565b0483a)


### Installing
```
cargo add bevy_foliage_tool
```

 



### Run example 

```
cargo run --example basic
```






### Integration


In your main.rs : 

```

    app 

    .add_plugins(BevyFoliageToolPlugin {

            foliage_config_path: "assets/foliage/foliage_config.ron".to_string()

        } ) 
        
   .add_plugins(BevyFoliageProtoPlugin )

   ;

```




Then make a system to load+register foliage assets:  (for BevyFoliageProtoPlugin)



```


 app  .add_systems(Startup, register_foliage_assets) ;



 ...




fn register_foliage_assets(

    asset_server: Res <AssetServer>, 

    mut assets_resource: ResMut<FoliageAssetsResource>, 

    mut next_state: ResMut<NextState<FoliageAssetsState>>, 

) {


    let green_material: StandardMaterial = Color::srgb(0.4, 0.7, 0.6) .into();

    assets_resource.register_foliage_mesh("grass1", asset_server.load( "foliage/meshes/grass1.obj" ));

    assets_resource.register_foliage_material("standard_green", asset_server.add( green_material ));


    next_state.set( FoliageAssetsState::Loaded );
}



```






 
### Description 

 A very bevy-centric foliage painting plugin that takes advantage of entities, components and systems as much as possible to be as easy to understand and interact with as possible. 
  

 An example of it being used in the bevy_mesh_terrain_editor to allow the editor to paint regions 
 
 ![image](https://github.com/ethereumdegen/bevy_regions/assets/6249263/00192676-9010-4727-9cca-6ee2bbb55c96)

 


### Do you like this crate?  Support the creator on Patreon
https://www.patreon.com/infernaltoast 
 
 [![creatinggames-preview](https://github.com/user-attachments/assets/7e7904c1-5f2b-47b6-84dd-5626cb7baca0)](https://www.patreon.com/infernaltoast)

 
 
