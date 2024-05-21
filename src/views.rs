diesel::table! {
    v_ram_ref_laptop (ref_laptop) {
        ref_laptop -> Uuid,
        puissance -> Decimal,
        frequence -> Decimal
    }
}
