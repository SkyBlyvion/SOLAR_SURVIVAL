#import bevy_pbr::forward_io::VertexOutput

struct PlanetUniform {
    sun: vec4<f32>,
    color_kind: vec4<f32>,
    parameters: vec4<f32>,
}
@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> planet: PlanetUniform;

fn hash(p: vec3<f32>) -> f32 {
    return fract(sin(dot(p,vec3<f32>(127.1,311.7,74.7))) * 43758.5453);
}
fn noise(p: vec3<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f*f*(vec3<f32>(3.0)-2.0*f);
    return mix(mix(mix(hash(i),hash(i+vec3<f32>(1.0,0.0,0.0)),u.x),
        mix(hash(i+vec3<f32>(0.0,1.0,0.0)),hash(i+vec3<f32>(1.0,1.0,0.0)),u.x),u.y),
        mix(mix(hash(i+vec3<f32>(0.0,0.0,1.0)),hash(i+vec3<f32>(1.0,0.0,1.0)),u.x),
        mix(hash(i+vec3<f32>(0.0,1.0,1.0)),hash(i+vec3<f32>(1.0,1.0,1.0)),u.x),u.y),u.z);
}
fn fbm(p: vec3<f32>) -> f32 {
    return 0.57*noise(p)+0.28*noise(p*2.1)+0.15*noise(p*4.3);
}
@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let kind = u32(planet.color_kind.w+0.5);
    let longitude = mesh.uv.x * 6.2831853;
    let latitude = (mesh.uv.y-0.5) * 3.1415926;
    let p = vec3<f32>(cos(latitude)*cos(longitude),sin(latitude),cos(latitude)*sin(longitude));
    let detail = fbm(p*5.0+vec3<f32>(planet.parameters.x*0.013));
    var color = planet.color_kind.rgb * (0.65+detail*0.65);
    if kind == 0u {
        return vec4<f32>(vec3<f32>(8.0,3.8,0.8)*(0.75+detail*0.5),1.0);
    }
    if kind == 3u {
        let continents = fbm(p*3.0+vec3<f32>(12.0,6.0,1.0));
        let land = smoothstep(0.50,0.54,continents);
        color = mix(vec3<f32>(0.012,0.07,0.25),mix(vec3<f32>(0.05,0.18,0.06),vec3<f32>(0.35,0.28,0.12),detail),land);
        let cloud = smoothstep(0.61,0.76,fbm(p*11.0+vec3<f32>(5.0)));
        color = mix(color,vec3<f32>(0.8,0.88,0.95),cloud*0.85);
        color = mix(color,vec3<f32>(0.8,0.9,1.0),smoothstep(0.88,0.96,abs(p.y)));
    }
    if kind == 4u {
        color = mix(vec3<f32>(0.22,0.055,0.018),vec3<f32>(0.72,0.30,0.10),detail);
        color = mix(color,vec3<f32>(0.78,0.79,0.74),smoothstep(0.94,0.98,abs(p.y)));
    }
    if kind >= 5u && kind <= 8u {
        let band = sin(p.y*70.0+detail*2.0)*0.5+0.5;
        color = planet.color_kind.rgb*(0.55+band*0.55);
        if kind == 5u {
            let storm = exp(-pow((p.y+0.23)*15.0,2.0)-pow((p.x-0.82)*18.0,2.0)-pow((p.z-0.45)*18.0,2.0));
            color = mix(color,vec3<f32>(0.52,0.12,0.045),storm);
        }
    }
    let normal = normalize(mesh.world_normal);
    let daylight = max(dot(normal,normalize(planet.sun.xyz)),0.0);
    let view = normalize(-mesh.world_position.xyz);
    let rim = pow(1.0-max(dot(normal,view),0.0),4.0);
    var atmosphere = vec3<f32>(0.0);
    if kind == 3u { atmosphere = vec3<f32>(0.035,0.16,0.36)*rim*daylight; }
    return vec4<f32>(color*(0.018+1.65*daylight)+atmosphere,1.0);
}
