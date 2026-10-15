#import bevy_pbr::mesh_view_bindings::{view, fog}
#ifdef TONEMAP_IN_SHADER
#import bevy_core_pipeline::tonemapping::tone_mapping
#endif
struct SkyUniform {
    inertial_from_local: mat4x4<f32>, body_from_local: mat4x4<f32>, weather: vec4<f32>, observer: vec4<f32>, up: vec4<f32>, sun: vec4<f32>, zenith: vec4<f32>, horizon: vec4<f32>,
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
// Bruit de valeur borné, sans texture/volume ni trigonométrie dans le hash.
fn cloud_hash(cell: vec3<i32>) -> f32 {
    var h = bitcast<u32>(cell.x) * 1597334677u ^ bitcast<u32>(cell.y) * 3812015801u ^ bitcast<u32>(cell.z) * 2798796415u;
    h = (h ^ (h >> 16u)) * 2246822519u;
    h = (h ^ (h >> 13u)) * 3266489917u;
    return f32((h ^ (h >> 16u)) & 16777215u) / 16777215.0;
}
fn cloud_noise(p: vec3<f32>) -> f32 {
    let i = vec3<i32>(floor(p));
    let f = fract(p);
    let t = f*f*(3.0-2.0*f);
    let a = mix(cloud_hash(i),cloud_hash(i+vec3<i32>(1,0,0)),t.x);
    let b = mix(cloud_hash(i+vec3<i32>(0,1,0)),cloud_hash(i+vec3<i32>(1,1,0)),t.x);
    let c = mix(cloud_hash(i+vec3<i32>(0,0,1)),cloud_hash(i+vec3<i32>(1,0,1)),t.x);
    let d = mix(cloud_hash(i+vec3<i32>(0,1,1)),cloud_hash(i+vec3<i32>(1,1,1)),t.x);
    return mix(mix(a,b,t.y),mix(c,d,t.y),t.z);
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
    // Couche de glace mince à altitude artistique 20 km, ancrée dans Mars.
    // Intersection de rayon avec une coquille en unités de 20 km, aucun volume GPU.
    let body_ray = (sky.body_from_local * vec4<f32>(ray,0.0)).xyz;
    let projection = dot(sky.observer.xyz,body_ray);
    let distance = -projection + sqrt(max(0.0,projection*projection + sky.observer.w*sky.observer.w-dot(sky.observer.xyz,sky.observer.xyz)));
    let cloud_p = sky.observer.xyz + body_ray * distance;
    let wind = vec3<f32>(sin(sky.weather.z)*2.0,cos(sky.weather.z)*2.0,sin(sky.weather.w)*2.0);
    let broad = cloud_noise(cloud_p*vec3<f32>(12.0,7.0,4.0)+wind);
    let detail = cloud_noise(cloud_p*vec3<f32>(30.0,18.0,9.0)+wind*2.0);
    let cloud = smoothstep(0.43,0.75,broad*0.7+detail*0.3) * sky.weather.y * smoothstep(0.0,0.12,height);
    let cloud_light = max(day, (1.0-smoothstep(0.0,0.14,abs(sky.sun.w)))*0.4);
    color = mix(color,vec3<f32>(0.4,0.42,0.43)*cloud_light+vec3<f32>(0.001),cloud);
    let extinction = exp(-sky.weather.x * 0.25 / (height+0.12));
    let sun_separation = length(ray-sky.sun.xyz);
    let dusk = 1.0-smoothstep(0.0,0.22,abs(sky.sun.w));
    // Halo atmosphérique de présentation : doux, sans agrandir le disque physique.
    let glow = exp(-sun_separation*sun_separation/0.025) * max(day,dusk*0.7);
    color = mix(color, vec3<f32>(0.10,0.25,0.44), glow * dusk * 0.85 * smoothstep(0.0,0.025,height));
    color += vec3<f32>(0.07,0.055,0.035)*glow*day*smoothstep(0.0,0.08,height);
    let inertial = normalize((sky.inertial_from_local * vec4<f32>(ray,0.0)).xyz);
    let star_uv = vec2<f32>(atan2(inertial.z,inertial.x) / 6.283185307 + 0.5, 0.5 - asin(clamp(inertial.y,-1.0,1.0)) / 3.141592654);
    color += textureSampleLevel(stars,stars_sampler,star_uv,0.0).rgb * pow(1.0-day,4.0) * 0.9 * extinction * (1.0-cloud);
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
        // La poussière et les voiles atténuent aussi les disques ; rayon solaire inchangé.
        let transmission = extinction * (1.0-cloud);
        color = mix(color,body_color*transmission+color*(1.0-transmission),coverage);
    }
    return finish_color(color);
}
