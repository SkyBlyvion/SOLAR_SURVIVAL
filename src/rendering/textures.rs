//! Textures locales préparées hors jeu, mipmaps sRGB et budget vérifié côté GPU.
use bevy::{
    asset::RenderAssetUsages,
    image::{ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
    render::{
        Render, RenderApp, RenderSystems,
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_asset::RenderAssets,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
        texture::GpuImage,
    },
};

pub const TEXTURE_BUDGET: u64 = 128 * 1024 * 1024;
#[derive(Resource, Clone, ExtractResource)]
pub(super) struct P7Textures {
    pub earth: Handle<Image>,
    pub mars: Handle<Image>,
    pub ground: Handle<Image>,
    pub wall: Handle<Image>,
    pub roof: Handle<Image>,
    pub crate_map: Handle<Image>,
    pub sign: Handle<Image>,
    pub mercury: Handle<Image>,
    pub venus: Handle<Image>,
    pub jupiter: Handle<Image>,
    pub saturn: Handle<Image>,
    pub uranus: Handle<Image>,
    pub neptune: Handle<Image>,
}
impl P7Textures {
    fn handles(&self) -> [&Handle<Image>; 13] {
        [
            &self.earth,
            &self.mars,
            &self.ground,
            &self.wall,
            &self.roof,
            &self.crate_map,
            &self.sign,
            &self.mercury,
            &self.venus,
            &self.jupiter,
            &self.saturn,
            &self.uranus,
            &self.neptune,
        ]
    }
}
pub(super) struct TexturePlugin;
impl Plugin for TexturePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractResourcePlugin::<P7Textures>::default())
            .add_systems(PreStartup, load);
        if let Some(render) = app.get_sub_app_mut(RenderApp) {
            render.add_systems(Render, audit_gpu.after(RenderSystems::PrepareAssets));
        }
    }
}
fn payload_bytes(width: u32, height: u32, levels: u32) -> u64 {
    (0..levels)
        .map(|i| u64::from((width >> i).max(1)) * u64::from((height >> i).max(1)) * 4)
        .sum()
}
/// Format étroit P7TX : 16 octets d'en-tête LE puis RGBA8, grand niveau d'abord.
/// Aucune conversion, compression ou génération de niveaux à l'exécution.
fn decode(mut bytes: Vec<u8>, repeat: bool) -> Result<Image, &'static str> {
    if bytes.len() < 16 || &bytes[..4] != b"P7TX" {
        return Err("en-tête P7TX absent");
    }
    let number = |offset| u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
    let (width, height, levels) = (number(4), number(8), number(12));
    if !(1..=2048).contains(&width)
        || !(1..=1024).contains(&height)
        || levels != width.max(height).ilog2() + 1
    {
        return Err("dimensions/niveaux P7 hors budget");
    }
    if bytes.len() as u64 != 16 + payload_bytes(width, height, levels) {
        return Err("taille des mipmaps P7 incorrecte");
    }
    bytes.drain(..16);
    let mut image = Image::new_uninit(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.texture_descriptor.mip_level_count = levels;
    image.data = Some(bytes);
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: if repeat {
            ImageAddressMode::Repeat
        } else {
            ImageAddressMode::ClampToEdge
        },
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        mipmap_filter: ImageFilterMode::Linear,
        anisotropy_clamp: 8,
        ..default()
    });
    Ok(image)
}
fn load(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let started = std::time::Instant::now();
    let mut total = 0;
    let mut load_one = |name: &str, repeat| {
        let path = crate::config::asset_directory()
            .join("textures")
            .join(format!("{name}.p7tex"));
        let length = std::fs::metadata(&path).expect("Texture P7 requise").len();
        assert!(length <= 16 * 1024 * 1024, "Fichier P7 hors budget");
        let image = decode(std::fs::read(&path).expect("Lecture texture P7"), repeat)
            .expect("Texture P7 valide");
        total += image.data.as_ref().unwrap().len() as u64;
        assert!(total <= TEXTURE_BUDGET, "Budget P7 dépassé");
        images.add(image)
    };
    let maps = P7Textures {
        earth: load_one("earth", false),
        mars: load_one("mars", false),
        ground: load_one("regolith", true),
        wall: load_one("wall", true),
        roof: load_one("roof", true),
        crate_map: load_one("crate", false),
        sign: load_one("oxygen-sign", false),
        mercury: load_one("mercury", false),
        venus: load_one("venus", false),
        jupiter: load_one("jupiter", false),
        saturn: load_one("saturn", false),
        uranus: load_one("uranus", false),
        neptune: load_one("neptune", false),
    };
    info!(
        "P7 textures CPU préparées : 13 images, {total} octets, {:.3} ms ; données transférées au renderer, staging/pics pilote exclus",
        started.elapsed().as_secs_f64() * 1000.0
    );
    commands.insert_resource(maps);
}
fn audit_gpu(
    maps: Option<Res<P7Textures>>,
    images: Res<RenderAssets<GpuImage>>,
    mut done: Local<bool>,
) {
    if *done {
        return;
    }
    let Some(maps) = maps else {
        return;
    };
    let mut total = 0;
    for handle in maps.handles() {
        let Some(image) = images.get(handle.id()) else {
            return;
        };
        let d = &image.texture_descriptor;
        assert_eq!(d.format, TextureFormat::Rgba8UnormSrgb);
        total += payload_bytes(d.size.width, d.size.height, d.mip_level_count);
    }
    assert!(total <= TEXTURE_BUDGET, "Budget P7 GPU dépassé");
    info!(
        "P7 GPU résident : 13 textures RGBA8 sRGB avec mipmaps, {total} octets / {TEXTURE_BUDGET} ; allocation pilote totale/staging exclus"
    );
    *done = true;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn packaged_maps_have_complete_mips_and_fit_the_gpu_budget() {
        let mut total = 0;
        for (name, repeat) in [
            ("earth", false),
            ("mars", false),
            ("regolith", true),
            ("wall", true),
            ("roof", true),
            ("crate", false),
            ("oxygen-sign", false),
            ("mercury", false),
            ("venus", false),
            ("jupiter", false),
            ("saturn", false),
            ("uranus", false),
            ("neptune", false),
        ] {
            let bytes = std::fs::read(
                crate::config::asset_directory().join(format!("textures/{name}.p7tex")),
            )
            .unwrap();
            let image = decode(bytes, repeat).unwrap();
            total += image.data.as_ref().unwrap().len() as u64;
            assert_eq!(image.asset_usage, RenderAssetUsages::RENDER_WORLD);
            let ImageSampler::Descriptor(sampler) = image.sampler else {
                panic!()
            };
            assert_eq!(sampler.anisotropy_clamp, 8);
        }
        assert_eq!(total, 42_118_856);
        assert!(total < TEXTURE_BUDGET);
    }
    #[test]
    fn broken_texture_headers_payloads_and_oversized_dimensions_are_refused() {
        let good =
            std::fs::read(crate::config::asset_directory().join("textures/wall.p7tex")).unwrap();
        for size in [0, 3, 15, good.len() - 1] {
            assert!(decode(good[..size].to_vec(), true).is_err());
        }
        for (offset, value) in [(4, 0u32), (4, 4096), (8, 2048), (12, 32)] {
            let mut invalid = good.clone();
            invalid[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
            assert!(decode(invalid, true).is_err());
        }
    }
}
