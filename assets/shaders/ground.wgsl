#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
    pbr_types::STANDARD_MATERIAL_FLAGS_UNLIT_BIT,
}
@group(#{MATERIAL_BIND_GROUP}) @binding(100) var ground_map: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(101) var ground_sampler: sampler;

@fragment
fn fragment(in: VertexOutput, @builtin(front_facing) front: bool) -> FragmentOutput {
    var pbr = pbr_input_from_standard_material(in,front);
    // Coordonnées réduites par patch en f64, dans Mars ; jamais la position monde f32.
    let p = vec3<f32>(in.uv,in.uv_b.x);
    let body_normal = normalize(cross(dpdx(p),dpdy(p)));
    let squared = body_normal*body_normal;
    var weights = squared*squared;
    weights /= max(dot(weights,vec3<f32>(1.0)),0.00001);
    let tile = textureSample(ground_map,ground_sampler,p.yz)*weights.x
        + textureSample(ground_map,ground_sampler,p.xz)*weights.y
        + textureSample(ground_map,ground_sampler,p.xy)*weights.z;
    pbr.material.base_color = vec4<f32>(pbr.material.base_color.rgb*tile.rgb,1.0);
    var out: FragmentOutput;
    if (pbr.material.flags & STANDARD_MATERIAL_FLAGS_UNLIT_BIT) == 0u {
        out.color = apply_pbr_lighting(pbr);
    } else {
        out.color = pbr.material.base_color;
    }
    out.color = main_pass_post_lighting_processing(pbr,out.color);
    return out;
}
