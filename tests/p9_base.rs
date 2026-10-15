use bevy::{
    math::{DQuat, DVec3},
    prelude::*,
};
use solar_survival::{
    base::*,
    buildings::Buildings,
    construction::Construction,
    inventory::*,
    planets::terrain::MarsTerrain,
    player::surface::SurfaceState,
    universe::{SimulationClock, catalog::SolarSystem},
};
fn terrain() -> MarsTerrain {
    let mut world = World::new();
    world.init_resource::<SolarSystem>();
    let mut t = MarsTerrain::from_world(&mut world);
    t.globe_enabled = true;
    t.set_site(DVec3::Y);
    t
}
#[test]
fn bounded_stacks_atomic_exact_partial_maximum_repetition_and_organization() {
    let mut a = Inventory::new(2);
    let mut b = Inventory::new(1);
    a.add(Item::Alloy, 37).unwrap();
    b.add(Item::Alloy, 18).unwrap();
    let before = (a.clone(), b.clone());
    assert!(transfer(&mut a, &mut b, 0, Amount::Exact(3)).is_err());
    assert_eq!((a.clone(), b.clone()), before);
    assert_eq!(transfer(&mut a, &mut b, 0, Amount::Maximum), Ok(2));
    let before = (a.clone(), b.clone());
    for _ in 0..3 {
        assert!(transfer(&mut a, &mut b, 0, Amount::Maximum).is_err());
    }
    assert_eq!((a.clone(), b.clone()), before);
    assert_eq!(a.count(Item::Alloy) + b.count(Item::Alloy), 55);
    a.swap(0, 1).unwrap();
    a.organize();
    assert_eq!(a.slots[0].as_ref().unwrap().quantity, 20);
    a.pay(&[(Item::Alloy, 5), (Item::Circuit, 1)]).unwrap_err();
    assert_eq!(a.count(Item::Alloy), 35);
    a.pay(&[(Item::Alloy, 20), (Item::Alloy, 16)]).unwrap_err();
    assert_eq!(a.count(Item::Alloy), 35);
    a.pay(&[(Item::Alloy, 5)]).unwrap();
    assert_eq!(a.count(Item::Alloy), 30);
    assert!(a.valid(2));
    assert!(a.add(Item::Circuit, 21).is_err());
}
#[test]
fn renewable_harvest_full_refusal_recovery_and_wall_access() {
    let mut t = terrain();
    let mut b = Buildings::default();
    b.install_first(&t);
    b.publish_colliders(&mut t);
    let mut base = Base::default();
    base.seed_site(&t);
    let p = base.sources[1].position_m;
    let eye = p + base.sources[1].orientation * DVec3::Y * 1.65;
    let mut inventory = Inventory::new(1);
    inventory.add(Item::Alloy, 20).unwrap();
    assert!(base.harvest(1, &mut inventory, p, eye, &t).is_err());
    assert_eq!(base.sources[1].remaining, 6);
    inventory = Inventory::new(1);
    assert_eq!(base.harvest(1, &mut inventory, p, eye, &t), Ok(6));
    assert!(base.harvest(1, &mut inventory, p, eye, &t).is_err());
    base.advance(29.0, &b, 0.0);
    assert_eq!(base.sources[1].remaining, 0);
    base.advance(1.0, &b, 0.0);
    assert_eq!(base.sources[1].remaining, 6);
    assert!(
        base.harvest(1, &mut inventory, p + DVec3::X * 3.0, eye, &t)
            .is_err()
    );
    assert_eq!(base.sources[1].remaining, 6);
    let wall = solar_survival::physics::surface::BodyBox {
        center: p + DVec3::Y,
        orientation: DQuat::IDENTITY,
        size: DVec3::new(4.0, 0.2, 4.0),
    };
    t.building_solids.push(wall);
    assert!(base.harvest(1, &mut inventory, p, eye, &t).is_err());
    assert_eq!(base.sources[1].remaining, 6);
}
fn install(
    c: &mut Construction,
    t: &mut MarsTerrain,
    b: &mut Buildings,
    base: &Base,
    kind: BuildKind,
) -> solar_survival::buildings::BuildingId {
    c.kind = kind;
    c.active = true;
    let mut state = SurfaceState::default();
    state.feet.y = t.support_height(state.feet, 0.3) + 0.002;
    for distance in [7.0, 11.0, 15.0, 19.0, 23.0] {
        for lateral in [-16.0, -12.0, -4.0, 4.0, 12.0, 16.0] {
            c.distance_m = distance;
            c.lateral_m = lateral;
            if c.candidate(t, &state, b, base).verdict.is_ok() {
                return c.confirm(t, &state, b, base).unwrap();
            }
        }
    }
    panic!("Aucune pose admise pour {:?}", kind);
}
#[test]
fn fresh_bootstrap_harvest_carry_store_retrieve_build_produce_refill_and_rebase() {
    let mut t = terrain();
    let mut buildings = Buildings::default();
    buildings.install_first(&t);
    buildings.publish_colliders(&mut t);
    let mut base = Base::default();
    base.seed_site(&t);
    let mut c = Construction::default();
    c.seed_site(&t);
    // Récoltes réelles des sources, renouvelées en temps actif, aucun matériau injecté.
    for _ in 0..7 {
        for index in [0, 1] {
            let p = base.sources[index].position_m;
            let eye = p + base.sources[index].orientation * DVec3::Y * 1.65;
            base.harvest(index, &mut c.inventory, p, eye, &t).unwrap();
        }
        base.advance(30.0, &buildings, 0.0);
    }
    let chest = install(&mut c, &mut t, &mut buildings, &base, BuildKind::Chest);
    let f = buildings
        .facilities
        .iter_mut()
        .find(|f| f.id == chest)
        .unwrap();
    assert_eq!(
        transfer(&mut c.inventory, &mut f.inventory, 0, Amount::Exact(5)),
        Ok(5)
    );
    assert_eq!(
        transfer(&mut f.inventory, &mut c.inventory, 0, Amount::Stack),
        Ok(5)
    );
    for kind in [
        BuildKind::Panel,
        BuildKind::Battery,
        BuildKind::Generator,
        BuildKind::Tank,
    ] {
        install(&mut c, &mut t, &mut buildings, &base, kind);
    }
    assert_eq!(c.inventory.count(Item::Alloy), 2);
    assert_eq!(c.inventory.count(Item::Circuit), 28);
    assert_eq!(t.building_solids.len(), 11);
    let energy = Base::energy_capacity(&buildings);
    assert_eq!(energy, BATTERY_J);
    base.oxygen_s = 0.0;
    base.advance(60.0, &buildings, 120.0);
    assert_eq!(base.oxygen_s, 240.0);
    assert_eq!(base.energy_j, 4800.0);
    assert_eq!(base.refill(180.0), 180.0);
    assert_eq!(base.oxygen_s, 60.0);
    let before = buildings
        .facilities
        .iter()
        .map(|f| (f.id, f.position_m, f.inventory.clone()))
        .collect::<Vec<_>>();
    let mut state = SurfaceState {
        feet: DVec3::new(64.1, 10.0, 0.0),
        ..default()
    };
    state.migrate(&mut t);
    assert_eq!(
        before,
        buildings
            .facilities
            .iter()
            .map(|f| (f.id, f.position_m, f.inventory.clone()))
            .collect::<Vec<_>>()
    );
    assert!(!chest_access(
        &buildings,
        chest,
        t.body_relative(state.feet),
        t.body_relative(state.eye(&t)),
        &t
    ));
}
#[test]
fn day_night_storm_blackout_restart_full_tank_and_large_delta_remain_bounded() {
    let mut b = Buildings::default();
    let q = DQuat::IDENTITY;
    let p = DVec3::Y * 3389500.0;
    for kind in [
        BuildKind::Panel,
        BuildKind::Battery,
        BuildKind::Generator,
        BuildKind::Tank,
    ] {
        b.install_facility(kind, p, q);
    }
    let mut base = Base {
        oxygen_s: 0.0,
        ..default()
    };
    let clear = solar_survival::rendering::atmosphere::Weather::default();
    let storm = solar_survival::rendering::atmosphere::Weather {
        dust: 2.6,
        storm: 1.0,
        ..clear
    };
    assert_eq!(panel_power(-0.5, clear), 0.0);
    assert!(panel_power(0.7, storm) < panel_power(0.7, clear));
    base.advance(20.0, &b, 0.0);
    assert_eq!(base.oxygen_s, 0.0);
    assert_eq!(base.load_w, 0.0);
    base.advance(100.0, &b, 100.0);
    assert_eq!(base.oxygen_s, 400.0);
    assert_eq!(base.energy_j, 6000.0);
    base.advance(200.0, &b, 0.0);
    assert_eq!(base.oxygen_s, 1000.0);
    assert_eq!(base.energy_j, 0.0);
    base.advance(100.0, &b, 0.0);
    assert_eq!(base.oxygen_s, 1000.0);
    base.advance(10.0, &b, 80.0);
    assert_eq!(base.oxygen_s, 1040.0);
    base.advance(1e9, &b, 100.0);
    assert_eq!(base.oxygen_s, Base::oxygen_capacity(&b));
    assert_eq!(base.energy_j, BATTERY_J);
    base.advance(20.0, &b, 0.0);
    assert_eq!(base.load_w, 0.0);
    assert_eq!(base.energy_j, BATTERY_J);
    base.advance(f64::NAN, &b, 0.0);
    assert_eq!(base.energy_j, BATTERY_J);
}
#[test]
fn solar_power_uses_fixed_body_pose_and_changes_over_rotation_without_rebase_drift() {
    let mut t = terrain();
    // Site équatorial : le pôle Y conserve son incidence pendant la rotation.
    t.set_site(DVec3::X);
    let solar = SolarSystem::default();
    let mut b = Buildings::default();
    b.install_facility(
        BuildKind::Panel,
        t.body_relative(DVec3::ZERO),
        t.orientation,
    );
    let mut clock = SimulationClock {
        jd: 2451545.0,
        speed_index: 0,
        paused: false,
    };
    let mut min = f64::MAX;
    let mut max = 0.0_f64;
    for i in 0..24 {
        clock.jd = 2451545.0 + i as f64 / 24.0;
        let w = Base::solar_power(&b, &t, &solar, &clock);
        min = min.min(w);
        max = max.max(w);
    }
    assert_eq!(min, 0.0);
    assert!(max > 50.0);
    let before = Base::solar_power(&b, &t, &solar, &clock);
    let mut state = SurfaceState {
        feet: DVec3::new(64.1, 10.0, 0.0),
        ..default()
    };
    state.migrate(&mut t);
    assert_eq!(Base::solar_power(&b, &t, &solar, &clock), before);
}

#[test]
fn finite_oxygen_service_debits_only_received_units_and_consumes_after_mid_frame_depletion() {
    use solar_survival::{buildings::BuildingId, survival::Oxygen};
    let mut suit = Oxygen {
        remaining_seconds: 100.0,
        ..default()
    };
    let mut reserve = 3.0;
    suit.advance(12.0, true, false, Some(BuildingId(1)), &mut reserve);
    assert_eq!(reserve, 0.0);
    assert!((suit.remaining_seconds - 91.1).abs() < 1e-12);
    suit.advance(200.0, true, false, Some(BuildingId(1)), &mut reserve);
    assert!(suit.failed);
    reserve = 720.0;
    suit.advance(20.0, true, false, Some(BuildingId(1)), &mut reserve);
    assert_eq!(reserve, 720.0);
    assert_eq!(suit.remaining_seconds, 0.0);
}
