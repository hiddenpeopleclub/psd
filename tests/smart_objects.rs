use psd::{Psd, PsdGroup};

const GREEN_PIXEL: [u8; 4] = [0, 255, 0, 255];

/// cargo test --test layer_and_mask_information_section layer_with_smart_object -- --exact
#[test]
fn layer_with_smart_object() {
    let psd = include_bytes!("fixtures/smart-object.psd");
    let psd = Psd::from_bytes(psd).unwrap();
    psd.layer_by_name("RedGrass_01").unwrap();
    assert_eq!(1, 2);
    
}
