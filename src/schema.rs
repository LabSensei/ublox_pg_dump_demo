// @generated automatically by Diesel CLI.

diesel::table! {
    gps_real_time (id) {
        id -> Int4,
        time -> Timestamptz,
        lat -> Float8,
        lon -> Float8,
        alt -> Float8,
        speed -> Float8,
        heading -> Float8,
        time_gps -> Timestamptz,
    }
}
