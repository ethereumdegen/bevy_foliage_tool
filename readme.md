
 ## Bevy Foliage Tool


 
### Description 

 A very bevy-centric foliage painting plugin that takes advantage of entities, components and systems as much as possible to be as easy to understand and interact with as possible. 

Create, edit, save, load, render your foliage all with one tool.  Built with GPU mesh+material instancing in mind.   Supports multiple layers of foliage, each with any mesh + material provided by you.  The provided grass shader is toon-colored and waves in the wind. 
   
![grass2](https://github.com/user-attachments/assets/49ff52c2-4c63-4bc9-9fba-48b8b105ae4d)


 An example of the tool being used in the spirit_editor:
  


 
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




Then, define your foliage types : one for each layer 

```
 (
    foliage_definitions: [

      //layer index 0 
        (
            name: "Grass",

            mesh_name: Some("grass1"),

            material_name: Some("standard_green")
            
           
        ),
        
        


	]
)

```



### Do you like this crate?  Support the creator on Patreon
https://www.patreon.com/infernaltoast 
 
 [![creatinggames-preview](https://github.com/user-attachments/assets/7e7904c1-5f2b-47b6-84dd-5626cb7baca0)](https://www.patreon.com/infernaltoast)

 
 
