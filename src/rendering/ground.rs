//! Projection triplanaire métrique fixe dans les axes du corps martien.
use bevy::{
    math::DVec3,
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
};
pub(super) type GroundMaterial = ExtendedMaterial<StandardMaterial, GroundTexture>;
#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub(super) struct GroundTexture {
    #[texture(100)]
    #[sampler(101)]
    pub albedo: Handle<Image>,
}
impl MaterialExtension for GroundTexture {
    fn fragment_shader() -> ShaderRef {
        "shaders/ground.wgsl".into()
    }
}
pub(super) const PERIOD_M: f64 = 4.0;
/// Réduire seulement l'origine : modulo par sommet provoquerait une interpolation cassée.
/// Les patches voisins diffèrent d'un nombre entier de périodes, y compris aux faces/coins.
pub(super) fn texture_point(point: DVec3, origin: DVec3) -> DVec3 {
    (point - origin
        + DVec3::new(
            origin.x.rem_euclid(PERIOD_M),
            origin.y.rem_euclid(PERIOD_M),
            origin.z.rem_euclid(PERIOD_M),
        ))
        / PERIOD_M
}
pub(super) fn coordinates(mesh: &mut Mesh, points: impl Iterator<Item = DVec3>, origin: DVec3) {
    let points: Vec<_> = points.map(|p| texture_point(p, origin)).collect();
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        points
            .iter()
            .map(|p| [p.x as f32, p.y as f32])
            .collect::<Vec<_>>(),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_1,
        points.iter().map(|p| [p.z as f32, 0.0]).collect::<Vec<_>>(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::planets::{CubeFace, PatchEdge, PatchKey};
    #[test]
    fn phase_is_continuous_across_all_face_edges_and_reduced_before_f32() {
        for face in [
            CubeFace::PositiveX,
            CubeFace::NegativeX,
            CubeFace::PositiveY,
            CubeFace::NegativeY,
            CubeFace::PositiveZ,
            CubeFace::NegativeZ,
        ] {
            for edge in [
                PatchEdge::NegativeU,
                PatchEdge::PositiveU,
                PatchEdge::NegativeV,
                PatchEdge::PositiveV,
            ] {
                let key = PatchKey {
                    face,
                    level: 15,
                    x: if edge == PatchEdge::PositiveU {
                        32767
                    } else {
                        0
                    },
                    y: if edge == PatchEdge::PositiveV {
                        32767
                    } else {
                        0
                    },
                };
                let other = key.neighbor(edge).unwrap();
                let a = key.center_direction().unwrap() * 3389500.0;
                let b = other.key.center_direction().unwrap() * 3389500.0;
                for i in 0..=32 {
                    let (u, v) = match edge {
                        PatchEdge::NegativeU => (0, i),
                        PatchEdge::PositiveU => (32, i),
                        PatchEdge::NegativeV => (i, 0),
                        PatchEdge::PositiveV => (i, 32),
                    };
                    let p = key.vertex(u, v, 32, 3389500.0).unwrap();
                    let first = texture_point(p, a).as_vec3().as_dvec3();
                    let second = texture_point(p, b).as_vec3().as_dvec3();
                    let delta = first - second;
                    assert!((delta - delta.round()).abs().max_element() < 0.00005);
                    assert!(first.abs().max_element() < 200.0);
                }
            }
        }
    }
}
