//! A popup's wl_surface shown again. GTK hides a popover by unmapping its wl_surface and
//! destroying the xdg_popup and xdg_surface, and shows it again with new ones on the same
//! wl_surface: the same role taken again, which wl_surface allows. Ghostty was disconnected
//! with xdg_wm_base `role` the second time a menu opened.

use super::xdg_shell::{map_toplevel, settle};
use casement_core::geometry::PhysicalSize;
use casement_core::policy::ConnectionRole;
use casement_testclient::surface::popup::{Popup, PopupSpec};
use casement_testclient::surface::toplevel::WmBase;
use casement_testclient::{Pattern, shm_buffer};
use casement_testkit::{Fixture, FixtureSpec};

#[test]
fn a_popover_shown_again_on_its_surface_keeps_the_client() {
    let mut fixture = Fixture::start(FixtureSpec::desktop()).expect("fixture");
    let id = fixture.connect(ConnectionRole::Public);
    fixture.pump();
    let base = WmBase::bind(fixture.client(id), 7);
    let parent = map_toplevel(&mut fixture, id, &base, (400, 300));
    settle(&mut fixture);
    let surface = fixture.client(id).create_surface().expect("a surface");
    let spec = PopupSpec::menu_at((20, 20));

    let first = Popup::of(fixture.client(id), &base, &parent, surface.clone(), spec);
    surface.commit();
    fixture.pump();
    assert!(
        first.ack_latest().is_some(),
        "the first popup is configured"
    );
    let size = PhysicalSize {
        w: u32::try_from(spec.size.0).unwrap_or(1),
        h: u32::try_from(spec.size.1).unwrap_or(1),
    };
    let buffer = shm_buffer(fixture.client(id), size, Pattern::Solid(0xff20_2020));
    first.show(&buffer);
    fixture.pump();

    first.popup.destroy();
    first.xdg_surface.destroy();
    surface.attach(None, 0, 0);
    surface.commit();
    fixture.pump();

    let second = Popup::of(fixture.client(id), &base, &parent, surface.clone(), spec);
    surface.commit();
    fixture.pump();
    assert_eq!(fixture.client(id).protocol_error(), None);
    assert!(
        second.ack_latest().is_some(),
        "the second popup is configured"
    );
    second.show(&buffer);
    fixture.pump();
    assert_eq!(fixture.client(id).protocol_error(), None);
}
