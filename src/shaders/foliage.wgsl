 
//see bindings in terrain_material.rs 
 
 //https://github.com/nicopap/bevy_mod_paramap/blob/main/src/parallax_map.wgsl

 // https://github.com/mikeam565/first-game/blob/main/assets/shaders/grass_shader.wgsl


#import bevy_pbr::mesh_functions::{mesh_position_local_to_clip, get_world_from_local}
 
 #import bevy_pbr::{
    forward_io::{  VertexOutput, FragmentOutput},
      mesh_view_bindings::view,
        mesh_view_bindings::globals,
         
      pbr_bindings,
    
    pbr_fragment::pbr_input_from_standard_material,
      pbr_functions::{alpha_discard,calculate_tbn_mikktspace,apply_pbr_lighting, main_pass_post_lighting_processing,
      prepare_world_normal,
      apply_normal_mapping,
      calculate_view

      },
    // we can optionally modify the lit color before post-processing is applied
    pbr_types::{STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT,STANDARD_MATERIAL_FLAGS_UNLIT_BIT},
}



// #import bevy_shader_utils::perlin_noise_2d::perlin_noise_2d


#import bevy_core_pipeline::tonemapping::tone_mapping
  
 #import bevy_pbr::pbr_types::StandardMaterial
 

 //https://dev.to/mikeam565/rust-game-dev-log-6-custom-vertex-shading-using-extendedmaterial-4312
//https://github.com/DGriffin91/bevy_mod_standard_material/blob/main/assets/shaders/pbr.wgsl




//@group(1) @binding(0)
//var base_color:  vec4<f32>;


//@group(1) @binding(0) var<uniform> base_material: StandardMaterial;
 

 
 

//should consider adding splat painting to this ..   performs a color shift 

  

 //mod the UV using parallax 
  // https://github.com/nicopap/bevy_mod_paramap/blob/main/src/parallax_map.wgsl

 //later ? 



// https://bevyengine.org/examples/shaders/shader-instancing/


// see https://github.com/bevyengine/bevy/blob/1030a99b8e2680a7e696d6433b79f5671768231c/crates/bevy_pbr/src/render/forward_io.wgsl#L32-L56


/*
@vertex
fn vertex(
         vertex: Vertex,
         @builtin(instance_index) instance_index: u32,
 
       //   @builtin(instance_index) instance_index: u32,
       ) -> VertexOutput {
    

    var out: VertexOutput;

    let wind_speed = 0.5;
    var wind_strength = 0.25 ;

    let wind_amount = cos(globals.time * wind_speed);

    let wind: vec2<f32> = vec2f( wind_amount , wind_amount);
    
    //   var noise = perlin_noise_2d(vec2<f32>(vertex_no_morph.world_position.x/50.0 + globals.time * 0.5, vertex_no_morph.world_position.z/50.0 + globals.time * 0.5));



  //  var position_field_offset = vec3<f32>(vertex.position.x, 0., vertex.position.z);
    //position_field_offset = position_field_offset - vec3f(config.wind,0.);

    var position =vertex.position; 

   
     // ---WIND---
    // only applies wind if the vertex is not on the bottom of the grass (or very small)
    let offset =  wind ;
    let final_strength = max(0.,log(vertex.position.y + 1.))  * wind_strength;
    position.x += offset.x * final_strength;
    position.z += offset.y * final_strength;
    
    // ---CLIP_POSITION---
  //  out.position = mesh_position_local_to_clip(get_model_matrix( instance_index ), vec4<f32>(position, 1.0));

     // var model = mesh_functions::get_model_matrix(instance_index);

    // out.world_position = mesh_functions::mesh_position_local_to_world(model, vec4<f32>( position , 1.0));
   // out.position = mesh_functions::position_world_to_clip(out.world_position.xyz);

   //clip psn out ! 
      out. position = mesh_position_local_to_clip(get_world_from_local(
         instance_index // 0u ? 


        ), vec4<f32>(position, 1.0));

    //    out.color = base_material.base_color;

    return out;
}
*/



struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) blend_color: vec4<f32>,
};



@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    let time_base =  ( globals.time  ) * 1.0  ;

    let sinewave_time = sin(  time_base  );

    var local_psn_output = vertex.position;

    local_psn_output.y = local_psn_output.y + sin(  time_base +  local_psn_output.x) *  0.20; 

    out.position = mesh_position_local_to_clip(
        get_world_from_local(vertex.instance_index),
        vec4<f32>(local_psn_output, 1.0),
    );
   

   
    return out;
}






@fragment
fn fragment(
     
     in: VertexOutput,
         @builtin(front_facing) is_front: bool,
) -> @location(0) vec4<f32> {
    

    var pbr_input = pbr_input_from_standard_material(in, is_front);


    let uv = in.uv  ;
 

     var color =  pbr_input.material.base_color ;
    color.r = 1.0;
    color.a = 1.0;
 
    return  color;
}

 


 /*


//from warbler grass.. 



@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    var position_field_offset = vec3<f32>(vertex.xz_position.x, 0., vertex.xz_position.y);
    position_field_offset = position_field_offset - vec3f(config.wind,0.);

    let density_offset = density_map_offset(position_field_offset.xz) / 1.;
    position_field_offset += vec3<f32>(density_offset.x, 0., density_offset.y);

    // ---Y_POSITIONS---
    position_field_offset.y = texture2d_offset(y_texture, position_field_offset.xz).r * aabb.vect.y;
    
    // ---NORMAL---
    var normal = sqrt(texture2d_offset(t_normal, vertex.xz_position.xy).xyz); // Get normal scaled over grass field in linear space
    normal = normal * 2. - vec3f(1.);
    normal = normalize(normal);
    let rotation_matrix = rotate_align(vec3<f32>(0.0, 1.0, 0.0), normal); // Calculate rotation matrix to align grass with normal
    
    // ---HEIGHT---
    var height = 0.;
    #ifdef HEIGHT_TEXTURE
        height = (texture2d_offset(height_texture, position_field_offset.xz).r + 4.) / 3.;
    #else
        height = height_uniform.height;
    #endif
    var position = rotation_matrix * (vertex.vertex_position * vec3<f32>(1., height, 1.)) + position_field_offset;
    // ---WIND---
    // only applies wind if the vertex is not on the bottom of the grass (or very small)
    let offset = wind_offset(position_field_offset.xz);
    let strength = max(0.,log(vertex.vertex_position.y + 1.));
    position.x += offset.x * strength;
    position.z += offset.y * strength;
    
    // ---CLIP_POSITION---
    out.clip_position = mesh_position_local_to_clip(get_model_matrix(instance_index.index), vec4<f32>(position, 1.0));

    // ---COLOR---
    var lambda = clamp(vertex.vertex_position.y, 0., 1.) ;

    out.color = mix(color.bottom_color, color.main_color, lambda) ;
    return out;
}



 */