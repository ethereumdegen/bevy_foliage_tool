
 ## Bevy Regions


### Installing
```
cargo add bevy_regions
```

 

### Bevy Versions
 

Regions 0.13.x -> Bevy 0.13.x


### Run example 

```
cargo run --example basic
```

 
### Description 

 A very bevy-centric region painting plugin that takes advantage of entities, components and systems as much as possible to be as easy to understand and interact with as possible. 
  

 An example of it being used in the bevy_mesh_terrain_editor to allow the editor to paint regions 
 
 ![image](https://github.com/ethereumdegen/bevy_regions/assets/6249263/00192676-9010-4727-9cca-6ee2bbb55c96)

## Texture Types 

*Region Map Texture*
The source of region index ! This uses a U8 texture which signifies the region index at any particular position. 
 
 
 

 

### Reference Shader Material 
see https://github.com/bevyengine/bevy/blob/main/examples/shader/shader_material.rs



### Editor (WIP)
https://github.com/ethereumdegen/bevy_mesh_terrain_editor

 
 
 
