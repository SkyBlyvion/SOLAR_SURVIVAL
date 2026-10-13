#import bevy_pbr::mesh_view_bindings::{view, fog}
#ifdef TONEMAP_IN_SHADER
#import bevy_core_pipeline::tonemapping::tone_mapping
#endif
struct SkyUniform {
    inertial_from_local: mat4x4<f32>, up: vec4<f32>, sun: vec4<f32>, zenith: vec4<f32>, horizon: vec4<f32>,
    directions: array<vec4<f32>, 10>, colors: array<vec4<f32>, 10>, lighting: array<vec4<f32>, 10>,
}
@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> sky: SkyUniform;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var stars: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var stars_sampler: sampler;
struct Output { @builtin(position) position: vec4<f32> }
@vertex
fn vertex(@location(0) position: vec3<f32>) -> Output {
    return Output(vec4<f32>(position.xy, 0.0, 1.0));
}
fn finish_color(color: vec3<f32>) -> vec4<f32> {
#ifdef TONEMAP_IN_SHADER
    return tone_mapping(vec4<f32>(color,1.0),view.color_grading);
#else
    return vec4<f32>(color,1.0);
#endif
}
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {
    let uv = (input.position.xy - view.viewport.xy) / view.viewport.zw;
    let p = view.view_from_clip * vec4<f32>(uv * vec2<f32>(2.0,-2.0) + vec2<f32>(-1.0,1.0), 1.0, 1.0);
    let ray = normalize((view.world_from_view * vec4<f32>(p.xyz / p.w, 0.0)).xyz);
    let pixel = max(length(dpdx(ray)), length(dpdy(ray)));
    let height = dot(ray, sky.up.xyz);
    let day = sky.up.w;
    let haze = exp(-max(height,0.0)*3.5);
    var color = mix(sky.zenith.rgb, fog.base_color.rgb, haze);
    // Tangente radiale idéale : fond sans support, raccordé au brouillard.
    if height < 0.0 { return finish_color(fog.base_color.rgb); }
    let sun_separation = length(ray-sky.sun.xyz);
    let dusk = 1.0-smoothstep(0.0,0.22,abs(sky.sun.w));
    // Halo atmosphérique de présentation : doux, sans agrandir le disque physique.
    let glow = exp(-sun_separation*sun_separation/0.025) * max(day,dusk*0.7);
    color = mix(color, vec3<f32>(0.24,0.28,0.32), glow * dusk * 0.65 * smoothstep(0.0,0.08,height));
    color += vec3<f32>(0.07,0.055,0.035)*glow*day*smoothstep(0.0,0.08,height);
    let inertial = normalize((sky.inertial_from_local * vec4<f32>(ray,0.0)).xyz);
    let star_uv = vec2<f32>(atan2(inertial.z,inertial.x) / 6.283185307 + 0.5, 0.5 - asin(clamp(inertial.y,-1.0,1.0)) / 3.141592654);
    color += textureSampleLevel(stars,stars_sampler,star_uv,0.0).rgb * pow(1.0-day,4.0) * 0.7;
    for (var i = 0u; i < 10u; i += 1u) {
        let body = sky.directions[i];
        let separation = length(ray-body.xyz);
        let radius = 2.0*sin(body.w*0.5);
        let sun = sky.colors[i].w > 0.5;
        let point = radius < pixel*0.5 && !sun;
        let footprint = select(radius,pixel*0.65,point);
        let coverage = 1.0-smoothstep(max(0.0,footprint-pixel*0.5),footprint+pixel*0.5,separation);
        var body_color = vec3<f32>(8.0,5.0,2.0);
        if !sun {
            let tangent = (ray-body.xyz)/max(radius,1e-9);
            let normal = normalize(tangent-body.xyz*sqrt(max(0.0,1.0-dot(tangent,tangent))));
            let lambert = max(dot(normal,sky.lighting[i].xyz),0.0);
            body_color = sky.colors[i].rgb*select(0.018+1.65*lambert,(0.1+sky.lighting[i].w)*pow(1.0-day,4.0)*0.7,point);
        }
        color = mix(color,body_color,coverage);
    }
    return finish_color(color);
}
