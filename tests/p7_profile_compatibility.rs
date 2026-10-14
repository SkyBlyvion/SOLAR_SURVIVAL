//! Fichiers réellement livrés par P6, conservés comme preuves historiques.
use bevy::prelude::*;
use solar_survival::{
    buildings::Buildings, config::GameConfig, construction::Construction, persistence::Session,
    planets::terrain::MarsTerrain, survival::Oxygen, universe::catalog::SolarSystem,
};
#[test]
fn delivered_p6_adventures_restore_without_migration_and_keep_shelter_services() {
    for (text, count, stock) in [
        (
            include_str!(
                "../docs/p6-profiles-validation/graphics-responsive/profiles-after-save/Alpha/session.solar"
            ),
            2,
            0,
        ),
        (
            include_str!(
                "../docs/p6-profiles-validation/graphics-responsive/profiles-after-save/Bravo/session.solar"
            ),
            1,
            6,
        ),
    ] {
        let mut world = World::new();
        world.init_resource::<SolarSystem>();
        world.init_resource::<GameConfig>();
        world.init_resource::<MarsTerrain>();
        world.init_resource::<Oxygen>();
        let session = Session::decode(text).unwrap();
        assert_eq!(session.encode(), text);
        session.restore(&mut world).unwrap();
        let terrain = world.resource::<MarsTerrain>();
        let buildings = world.resource::<Buildings>();
        assert_eq!(buildings.shelters.len(), count);
        assert_eq!(terrain.building_solids.len(), count * 6);
        assert_eq!(world.resource::<Construction>().alloy, stock);
        for b in &buildings.shelters {
            assert_eq!(
                buildings.shelter_at(b.position_m, b.orientation),
                Some(b.id)
            );
        }
    }
    assert!(
        Session::decode(include_str!(
            "../docs/p6-profiles-validation/graphics-responsive/profiles/Broken/session.solar"
        ))
        .is_err()
    );
}
