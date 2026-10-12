//! Projection radiale du cube : géométrie f64, sans ECS, GPU ni contacts.
use bevy::math::DVec3;
use std::collections::HashMap;

/// Ordre explicite conservé pour les graines historiques.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CubeFace {
    PositiveX = 0,
    NegativeX = 1,
    PositiveY = 2,
    NegativeY = 3,
    PositiveZ = 4,
    NegativeZ = 5,
}

impl CubeFace {
    pub const ALL: [Self; 6] = [
        Self::PositiveX,
        Self::NegativeX,
        Self::PositiveY,
        Self::NegativeY,
        Self::PositiveZ,
        Self::NegativeZ,
    ];

    /// (normale, axe u, axe v) ; u × v = normale extérieure.
    fn basis(self) -> (DVec3, DVec3, DVec3) {
        match self {
            Self::PositiveX => (DVec3::X, DVec3::NEG_Z, DVec3::Y),
            Self::NegativeX => (DVec3::NEG_X, DVec3::Z, DVec3::Y),
            Self::PositiveY => (DVec3::Y, DVec3::X, DVec3::NEG_Z),
            Self::NegativeY => (DVec3::NEG_Y, DVec3::X, DVec3::Z),
            Self::PositiveZ => (DVec3::Z, DVec3::X, DVec3::Y),
            Self::NegativeZ => (DVec3::NEG_Z, DVec3::NEG_X, DVec3::Y),
        }
    }

    /// u et v sans unité, dans [-1, 1] ; résultat unitaire.
    pub fn direction(self, u: f64, v: f64) -> Option<DVec3> {
        if !(-1.0..=1.0).contains(&u) || !(-1.0..=1.0).contains(&v) {
            return None;
        }
        let (n, a, b) = self.basis();
        Some((n + a * u + b * v).normalize())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FaceCoordinates {
    pub face: CubeFace,
    pub u: f64,
    pub v: f64,
}

impl FaceCoordinates {
    /// Accepte toute direction finie non nulle, même non normalisée.
    /// En cas d'égalité de composantes absolues, priorité X puis Y puis Z.
    /// Aux arêtes/coins, la direction est unique mais la face ne l'est pas.
    pub fn from_direction(direction: DVec3) -> Option<Self> {
        let scale = direction.abs().max_element();
        if !direction.is_finite() || scale == 0.0 {
            return None;
        }
        // Évite débordement/sous-débordement de la norme.
        let d = direction / scale;
        let a = d.abs();
        let face = if a.x >= a.y && a.x >= a.z {
            if d.x >= 0.0 {
                CubeFace::PositiveX
            } else {
                CubeFace::NegativeX
            }
        } else if a.y >= a.z {
            if d.y >= 0.0 {
                CubeFace::PositiveY
            } else {
                CubeFace::NegativeY
            }
        } else if d.z >= 0.0 {
            CubeFace::PositiveZ
        } else {
            CubeFace::NegativeZ
        };
        let (n, u, v) = face.basis();
        let denominator = d.dot(n);
        Some(Self {
            face,
            u: d.dot(u) / denominator,
            v: d.dot(v) / denominator,
        })
    }

    pub fn direction(self) -> Option<DVec3> {
        self.face.direction(self.u, self.v)
    }
}

/// Le paramètre d'arête croît suivant v pour ±u, suivant u pour ±v.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PatchEdge {
    NegativeU,
    PositiveU,
    NegativeV,
    PositiveV,
}

impl PatchEdge {
    pub const ALL: [Self; 4] = [
        Self::NegativeU,
        Self::PositiveU,
        Self::NegativeV,
        Self::PositiveV,
    ];

    fn opposite(self) -> Self {
        match self {
            Self::NegativeU => Self::PositiveU,
            Self::PositiveU => Self::NegativeU,
            Self::NegativeV => Self::PositiveV,
            Self::PositiveV => Self::NegativeV,
        }
    }

    /// Coordonnées entières sur une grille de segments × segments.
    pub fn sample(self, index: u32, segments: u32) -> Option<(u32, u32)> {
        if segments == 0 || index > segments {
            return None;
        }
        Some(match self {
            Self::NegativeU => (0, index),
            Self::PositiveU => (segments, index),
            Self::NegativeV => (index, 0),
            Self::PositiveV => (index, segments),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PatchNeighbor {
    pub key: PatchKey,
    /// Arête du voisin qui rejoint le patch source.
    pub edge: PatchEdge,
    /// Sur cette arête, t_voisin = 1 - t_source, sinon t_voisin = t_source.
    pub reversed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PatchKey {
    pub face: CubeFace,
    pub level: u8,
    pub x: u32,
    pub y: u32,
}

impl PatchKey {
    pub const MAX_LEVEL: u8 = 24;

    pub fn count(self) -> Option<u32> {
        if self.level > Self::MAX_LEVEL {
            return None;
        }
        let count = 1_u32 << self.level;
        (self.x < count && self.y < count).then_some(count)
    }

    /// Fraction locale du patch [0, 1]² → direction unitaire.
    pub fn direction(self, s: f64, t: f64) -> Option<DVec3> {
        let count = self.count()? as f64;
        if !(0.0..=1.0).contains(&s) || !(0.0..=1.0).contains(&t) {
            return None;
        }
        self.face.direction(
            2.0 * (self.x as f64 + s) / count - 1.0,
            2.0 * (self.y as f64 + t) / count - 1.0,
        )
    }

    pub fn center_direction(self) -> Option<DVec3> {
        self.direction(0.5, 0.5)
    }

    pub fn neighbor(self, edge: PatchEdge) -> Option<PatchNeighbor> {
        let count = self.count()?;
        let internal = match edge {
            PatchEdge::NegativeU if self.x > 0 => Some((self.x - 1, self.y)),
            PatchEdge::PositiveU if self.x + 1 < count => Some((self.x + 1, self.y)),
            PatchEdge::NegativeV if self.y > 0 => Some((self.x, self.y - 1)),
            PatchEdge::PositiveV if self.y + 1 < count => Some((self.x, self.y + 1)),
            _ => None,
        };
        if let Some((x, y)) = internal {
            return Some(PatchNeighbor {
                key: Self { x, y, ..self },
                edge: edge.opposite(),
                reversed: false,
            });
        }
        let (normal, u, v) = self.face.basis();
        let (across, tangent, along) = match edge {
            PatchEdge::NegativeU => (-u, v, self.y),
            PatchEdge::PositiveU => (u, v, self.y),
            PatchEdge::NegativeV => (-v, u, self.x),
            PatchEdge::PositiveV => (v, u, self.x),
        };
        let face = CubeFace::ALL.into_iter().find(|f| f.basis().0 == across)?;
        let (_, target_u, target_v) = face.basis();
        let target_edge = if normal.dot(target_u) == -1.0 {
            PatchEdge::NegativeU
        } else if normal.dot(target_u) == 1.0 {
            PatchEdge::PositiveU
        } else if normal.dot(target_v) == -1.0 {
            PatchEdge::NegativeV
        } else {
            PatchEdge::PositiveV
        };
        let target_tangent = match target_edge {
            PatchEdge::NegativeU | PatchEdge::PositiveU => target_v,
            _ => target_u,
        };
        let reversed = tangent.dot(target_tangent) < 0.0;
        let along = if reversed { count - 1 - along } else { along };
        let (x, y) = match target_edge {
            PatchEdge::NegativeU => (0, along),
            PatchEdge::PositiveU => (count - 1, along),
            PatchEdge::NegativeV => (along, 0),
            PatchEdge::PositiveV => (along, count - 1),
        };
        Some(PatchNeighbor {
            key: Self {
                face,
                level: self.level,
                x,
                y,
            },
            edge: target_edge,
            reversed,
        })
    }

    /// Coordonnées cubiques entières : identiques sur un bord commun,
    /// sans arrondi de coordonnées flottantes ni tolérance de soudure.
    fn lattice(self, i: u32, j: u32, segments: u32) -> Option<[i64; 3]> {
        let count = self.count()? as i64;
        if segments == 0 || segments > PatchPair::MAX_SEGMENTS || i > segments || j > segments {
            return None;
        }
        let segments = segments as i64;
        let d = count * segments;
        let u = 2 * (self.x as i64 * segments + i as i64) - d;
        let v = 2 * (self.y as i64 * segments + j as i64) - d;
        let (n, a, b) = self.face.basis();
        let cube = n * d as f64 + a * u as f64 + b * v as f64;
        Some([cube.x as i64, cube.y as i64, cube.z as i64])
    }

    /// Sommet en mètres dans les axes du corps, rayon moyen sans relief.
    pub fn vertex(self, i: u32, j: u32, segments: u32, radius_m: f64) -> Option<DVec3> {
        if !radius_m.is_finite() || radius_m <= 0.0 {
            return None;
        }
        let [x, y, z] = self.lattice(i, j, segments)?;
        Some(DVec3::new(x as f64, y as f64, z as f64).normalize() * radius_m)
    }

    /// Identique au mélange historique ; réservé aux clés valides.
    /// Ne pas utiliser une graine de patch pour déplacer les sommets de bord.
    pub fn deterministic_seed(self, seed: u64) -> u64 {
        let value = seed
            ^ ((self.face as u64) << 56)
            ^ ((self.level as u64) << 48)
            ^ ((self.x as u64) << 24)
            ^ self.y as u64;
        let mut z = value.wrapping_add(0x9e3779b97f4a7c15);
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
}

/// Deux patches de même niveau, sommets/indices communs sur le raccord.
/// Maximum 8 385 sommets uniques, 16 384 triangles ; aucune génération globale.
#[derive(Debug)]
pub struct PatchPair {
    pub keys: [PatchKey; 2],
    pub neighbor: PatchNeighbor,
    pub positions_m: Vec<DVec3>,
    /// Deux groupes d'indices CCW vus depuis l'extérieur.
    pub indices: [Vec<u32>; 2],
    /// Indices communs, ordonnés selon le paramètre de l'arête source.
    pub seam: Vec<u32>,
}

impl PatchPair {
    pub const MAX_SEGMENTS: u32 = 64;

    pub fn new(source: PatchKey, edge: PatchEdge, segments: u32, radius_m: f64) -> Option<Self> {
        if segments == 0
            || segments > Self::MAX_SEGMENTS
            || !radius_m.is_finite()
            || radius_m <= 0.0
        {
            return None;
        }
        let neighbor = source.neighbor(edge)?;
        let keys = [source, neighbor.key];
        let side = (segments + 1) as usize;
        let mut positions_m = Vec::with_capacity(2 * side * side - side);
        let mut vertices = HashMap::with_capacity(positions_m.capacity());
        let mut indices = [Vec::new(), Vec::new()];
        let mut seam = Vec::with_capacity(side);
        for (patch, key) in keys.into_iter().enumerate() {
            let mut grid = Vec::with_capacity(side * side);
            for j in 0..=segments {
                for i in 0..=segments {
                    let lattice = key.lattice(i, j, segments)?;
                    let index = *vertices.entry(lattice).or_insert_with(|| {
                        let index = positions_m.len() as u32;
                        positions_m.push(key.vertex(i, j, segments, radius_m).unwrap());
                        index
                    });
                    grid.push(index);
                }
            }
            indices[patch].reserve(6 * segments as usize * segments as usize);
            for j in 0..segments as usize {
                for i in 0..segments as usize {
                    let a = grid[j * side + i];
                    let b = grid[j * side + i + 1];
                    let c = grid[(j + 1) * side + i];
                    let d = grid[(j + 1) * side + i + 1];
                    indices[patch].extend_from_slice(&[a, b, c, b, d, c]);
                }
            }
            if patch == 0 {
                for k in 0..=segments {
                    let (i, j) = edge.sample(k, segments)?;
                    seam.push(grid[j as usize * side + i as usize]);
                }
            }
        }
        Some(Self {
            keys,
            neighbor,
            positions_m,
            indices,
            seam,
        })
    }
}
