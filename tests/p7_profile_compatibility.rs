//! Les anciennes preuves restent intactes ; P10 refuse explicitement les anciens formats.
use solar_survival::persistence::Session;
#[test]
fn old_adventures_are_rejected_without_implicit_terrain_migration() {
    for text in [
        include_str!(
            "../docs/p6-profiles-validation/graphics-responsive/profiles-after-save/Alpha/session.solar"
        ),
        include_str!(
            "../docs/p6-profiles-validation/graphics-responsive/profiles-after-save/Bravo/session.solar"
        ),
        include_str!(
            "../docs/p9-validation/2026-10-15/graphics-initial/profiles/P9 Alpha/session.solar"
        ),
    ] {
        assert!(
            Session::decode(text)
                .err()
                .unwrap()
                .contains("incompatible")
        );
    }
}
