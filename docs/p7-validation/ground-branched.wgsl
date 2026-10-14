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
    let fourth = squared*squared;
    var weights = fourth*fourth;
    weights /= max(dot(weights,vec3<f32>(1.0)),0.00001);
    // Les composantes <0,1% sont visuellement négligeables. Préparer les dérivées
    // avant les branches : textureSampleGrad reste valide en contrôle divergent.
    weights *= step(vec3<f32>(0.001),weights);
    weights /= max(dot(weights,vec3<f32>(1.0)),0.00001);
    let dx_p = dpdx(p);
    let dy_p = dpdy(p);
    var tile = vec4<f32>(0.0);
    if weights.x > 0.0 { tile += textureSampleGrad(ground_map,ground_sampler,p.yz,dx_p.yz,dy_p.yz)*weights.x; }
    if weights.y > 0.0 { tile += textureSampleGrad(ground_map,ground_sampler,p.xz,dx_p.xz,dy_p.xz)*weights.y; }
    if weights.z > 0.0 { tile += textureSampleGrad(ground_map,ground_sampler,p.xy,dx_p.xy,dy_p.xy)*weights.z; }
    pbr.material.base_color = vec4<f32>(pbr.material.base_color.rgb*tile.rgb,1.0);
    // Microrelief artistique, limité à 12 mm, sans déplacement de géométrie/contact.
    let height = tile.a * 0.012;
    let dx = dpdx(in.world_position.xyz);
    let dy = dpdy(in.world_position.xyz);
    let r1 = cross(dy,pbr.N);
    let r2 = cross(pbr.N,dx);
    let det = dot(dx,r1);
    let gradient = (dpdx(height)*r1 + dpdy(height)*r2)*sign(det)/max(abs(det),0.00000001);
    pbr.N = normalize(pbr.N-gradient);
    var out: FragmentOutput;
    if (pbr.material.flags & STANDARD_MATERIAL_FLAGS_UNLIT_BIT) == 0u {
        out.color = apply_pbr_lighting(pbr);
    } else {
        out.color = pbr.material.base_color;
    }
    out.color = main_pass_post_lighting_processing(pbr,out.color);
    return out;
}
