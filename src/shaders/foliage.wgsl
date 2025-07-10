 
//see bindings in terrain_material.rs 
 
 //https://github.com/nicopap/bevy_mod_paramap/blob/main/src/parallax_map.wgsl

 // https://github.com/mikeam565/first-game/blob/main/assets/shaders/grass_shader.wgsl


#import bevy_pbr::mesh_functions::{mesh_position_local_to_clip, get_world_from_local , mesh_position_local_to_world}
 
 #import bevy_pbr::{
    
      mesh_view_bindings::view,
        mesh_view_bindings::globals,

        mesh_view_bindings as view_bindings,

         
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
    prepass_io::{  FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput,  FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing, apply_fog,  alpha_discard},
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

 

@group(2) @binding(20) var fog_cloud_noise: texture_2d<f32>;
@group(2) @binding(21) var fog_cloud_noise_sampler: sampler;


  





#ifdef PREPASS_PIPELINE




struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) blend_color: vec4<f32>,
  
};


@vertex
fn vertex(vertex: Vertex) -> bevy_pbr::prepass_io::VertexOutput {

    var out: bevy_pbr::prepass_io::VertexOutput;




    let time_base =  1.0  ;

    let sinewave_time = sin(  time_base  );

    var local_psn_output = vertex.position;

    local_psn_output.y = vertex.position.y * (1.0 + sin(  time_base +  vertex.position.x) *  0.20); 
    local_psn_output.x = vertex.position.x * (1.0 + cos(  time_base +  vertex.position.y) *  0.10 * vertex.position.y); 

    out.position = mesh_position_local_to_clip(
        get_world_from_local(vertex.instance_index),
        vec4<f32>(local_psn_output, 1.0),
    );


    return out;
}



#else





struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) blend_color: vec4<f32>,
    @location(2) uv: vec2<f32>,
};


/*
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
*/


@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;




    let time_base =  ( globals.time  ) * 1.0  ;

    let sinewave_time = sin(  time_base  );

    var local_psn_output = vertex.position;

    local_psn_output.y = vertex.position.y * (1.0 + sin(  time_base +  vertex.position.x) *  0.20); 
    local_psn_output.x = vertex.position.x * (1.0 + cos(  time_base +  vertex.position.y) *  0.10 * vertex.position.y); 

    //very important that we do this ! 
    out.world_position  =   mesh_position_local_to_world( 
        get_world_from_local(vertex.instance_index),   //  mat4x4<f32>
        vec4<f32>(local_psn_output, 1.0)   // vertex position 
        );


    out.position = mesh_position_local_to_clip(
        get_world_from_local(vertex.instance_index), //  mat4x4<f32>
        vec4<f32>(local_psn_output, 1.0),  // vertex position 
    );



  
        //define vertex color based on height  but not in prepass! 

  //  out.color =  mix( vec4<f32>(0.6,0.6,0.6,1.0),  vec4<f32>(1.0,1.0,1.0,1.0) , local_psn_output.y  ) ;
   
   
    //out.position = vertex.position;
    out.uv = vertex.uv ; 
   
    return out;
}


#endif






#ifdef PREPASS_PIPELINE
     @fragment
    fn  fragment(
     in: bevy_pbr::prepass_io::VertexOutput,
      
    ) -> @location(0) vec4<f32> {

       bevy_pbr::pbr_prepass_functions::prepass_alpha_discard(in);
    

         var out: vec4<f32>;

         return out ; 
     }

#else 


    @fragment
    fn fragment(
         
         in: VertexOutput,
             @builtin(front_facing) is_front: bool,
    ) -> @location(0) vec4<f32> {

        
     

        let uv_transform = pbr_bindings::material.uv_transform; 
        var uv = (uv_transform * vec3(in.uv, 1.0)).xy;
     
            
            // for now ? 
       let vertex_color = mix( vec4<f32>(1.0,1.0,1.0,1.0),  vec4<f32>(0.5,0.5,0.5,1.0) ,  uv.y  ) ;

         
        
        //how can i do a gradient in the vertical based on local_position !? 



          var bias  = view.mip_bias;
     
            // this is how you access std material stuff in an ext when using a vertex pass ! 
         var blended_color = pbr_bindings::material.base_color;   // this is nice and green ! 

          
          let tex_color  = textureSample(
                pbr_bindings::base_color_texture,
                pbr_bindings::base_color_sampler,
                uv,
                 
            );
      

        blended_color  *= tex_color ; 
         blended_color  *= vertex_color; 







         //manual alpha mask 
          if  ( blended_color.a < 0.2 ) {
          discard;
         }




         //use this along with a time offset to sample the fog noise map (uv)  to simulate darkening due to clouds above  !

         let world_position = in.world_position; 

          let fog_cloud_time_base = ( globals.time   * 0.01 )   % 1.0 ;

          let fog_cloud_world_pos_offset = vec2<f32>( abs(world_position.x ) , abs(world_position.z )  ) * 0.01 ;
          let fog_cloud_scroll =  vec2<f32>( fog_cloud_time_base  ,  fog_cloud_time_base  )  ;

            //aso need sine wave time shit on this uv input 
        var  fog_cloud_noise_uv = fog_cloud_world_pos_offset + fog_cloud_scroll ; 
        
         fog_cloud_noise_uv.x = fog_cloud_noise_uv.x % 1.0;  
         fog_cloud_noise_uv.y = fog_cloud_noise_uv.y % 1.0;  

        let fog_cloud_sample = textureSample(fog_cloud_noise, fog_cloud_noise_sampler, fog_cloud_noise_uv)  ;


      //  let fog_cloud_output = fog_cloud_sample.r;
         let highlight_color = vec4<f32>(1.0, 1.0, 1.0, 1.0);
         let shadow_color = vec4<f32>(0.25,0.25,0.25, 1.0);
      
         

        let fog_cloud_color = mix(shadow_color, highlight_color, fog_cloud_sample.r  );

        blended_color = blended_color * fog_cloud_color; 
        



        


      //  var pbr_input = pbr_input_from_standard_material(in, is_front);
          // pbr_input.material.base_color = blended_color ; // vec4<f32>(1.0, 1.0, 1.0, 1.0);



            //var pbr_out: FragmentOutput; 
           // pbr_out.color =  apply_pbr_lighting(pbr_input);  // ??? make this more efficient ? 
          //  let lighting_average  = (pbr_out.color.r + pbr_out.color.g + pbr_out.color.b ) / 3.0 ;


            // could do cel shader quantization here ? 

 

        //     pbr_out.color =  pbr_out.color* blended_color ;
          //  pbr_input.material.base_color = blended_color;


        //apply atmospheric fog ! 
        //the fog doesnt have proper falloff !? 
    //    pbr_out.color = main_pass_post_lighting_processing(pbr_input, pbr_out.color);


         let white_color = vec4<f32>(1.0, 1.0, 1.0, 1.0);  //use for fog ? 



      #ifdef DISTANCE_FOG
         let fog_output = apply_fog(view_bindings::fog,  blended_color , world_position.xyz, view_bindings::view.world_position.xyz);


        let final_color =   fog_output ;    

        #else 

         let final_color =   blended_color ;    



       #endif    
       
      
        
             return   final_color;
        
       
    }

 

 #endif