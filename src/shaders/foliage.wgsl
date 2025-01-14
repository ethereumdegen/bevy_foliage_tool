 
//see bindings in terrain_material.rs 
 
 //https://github.com/nicopap/bevy_mod_paramap/blob/main/src/parallax_map.wgsl

 // https://github.com/mikeam565/first-game/blob/main/assets/shaders/grass_shader.wgsl


#import bevy_pbr::mesh_functions::{mesh_position_local_to_clip, get_world_from_local}
 
 #import bevy_pbr::{
    
      mesh_view_bindings::view,
        mesh_view_bindings::globals,
         
      pbr_bindings,
      pbr_types,
        pbr_functions, 

    pbr_fragment::pbr_input_from_standard_material,
      pbr_functions::{ 
      prepare_world_normal,
      apply_normal_mapping,
      calculate_view

      },
    // we can optionally modify the lit color before post-processing is applied
    pbr_types::{STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT,STANDARD_MATERIAL_FLAGS_UNLIT_BIT},
}




#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{ VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{ FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing, alpha_discard},
}
#endif





// #import bevy_shader_utils::perlin_noise_2d::perlin_noise_2d


#import bevy_core_pipeline::tonemapping::tone_mapping
  
 #import bevy_pbr::pbr_types::StandardMaterial
 

 //https://dev.to/mikeam565/rust-game-dev-log-6-custom-vertex-shading-using-extendedmaterial-4312
//https://github.com/DGriffin91/bevy_mod_standard_material/blob/main/assets/shaders/pbr.wgsl




//@group(1) @binding(0)
//var base_color:  vec4<f32>;


// @group(1) @binding(0) var<uniform> base_material: StandardMaterial;
 

 
 

//should consider adding splat painting to this ..   performs a color shift 

  

 //mod the UV using parallax 
  // https://github.com/nicopap/bevy_mod_paramap/blob/main/src/parallax_map.wgsl

 //later ? 



// https://bevyengine.org/examples/shaders/shader-instancing/


// see https://github.com/bevyengine/bevy/blob/1030a99b8e2680a7e696d6433b79f5671768231c/crates/bevy_pbr/src/render/forward_io.wgsl#L32-L56

 


  





#ifdef PREPASS_PIPELINE




struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) blend_color: vec4<f32>,
  
};


@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;


    return out;
}



#else





struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) blend_color: vec4<f32>,
    @location(2) uv: vec2<f32>,
};



// could use the default  and toggle IFDEF ? 
struct VertexOutput {
    // This is `clip position` when the struct is used as a vertex stage output
    // and `frag coord` when used as a fragment stage input
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>, 
    @location(2) uv: vec2<f32>, 
    @location(5) color: vec4<f32>, 
}



@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;




    let time_base =  ( globals.time  ) * 1.0  ;

    let sinewave_time = sin(  time_base  );

    var local_psn_output = vertex.position;

    local_psn_output.y = vertex.position.y * (1.0 + sin(  time_base +  vertex.position.x) *  0.20); 
    local_psn_output.x = vertex.position.x * (1.0 + cos(  time_base +  vertex.position.y) *  0.10 * vertex.position.y); 

    out.position = mesh_position_local_to_clip(
        get_world_from_local(vertex.instance_index),
        vec4<f32>(local_psn_output, 1.0),
    );



  
        //define vertex color based on height  but not in prepass! 

    out.color =  mix( vec4<f32>(0.6,0.6,0.6,1.0),  vec4<f32>(1.0,1.0,1.0,1.0) , local_psn_output.y  ) ;
   
   
    //out.position = vertex.position;
    out.uv = vertex.uv ; 
   
    return out;
}


#endif





@fragment
fn fragment(
     
     in: VertexOutput,
         @builtin(front_facing) is_front: bool,
) -> @location(0) vec4<f32> {

    

    #ifdef PREPASS_PIPELINE
    let vertex_color = vec4<f32>(1.0,1.0,1.0,1.0);
    return  vertex_color;
    #else
    let vertex_color = in.color;
     


    let uv_transform = pbr_bindings::material.uv_transform; 
    var uv = (uv_transform * vec3(in.uv, 1.0)).xy;
 
    

      var bias  = view.mip_bias;
 
        // this is how you access std material stuff in an ext when using a vertex pass ! 
     var color = pbr_bindings::material.base_color;

      
      let tex_color  = textureSample(
            pbr_bindings::base_color_texture,
            pbr_bindings::base_color_sampler,
            uv,
             
        );
  

    color  *= tex_color ; 
    color  *= vertex_color; 
     
    //manual alpha mask 
     if  ( color.a < 0.2 ) {
      discard;
     }
    
    return  color;


    #endif
}

 

 