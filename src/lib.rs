#[repr(C)]
union BytesToI16 {
    bytes: [u8; 4672082],
    data: [[i16; 2161]; 1081],
}

static U: BytesToI16 = BytesToI16 {
    bytes: *include_bytes!("data.bin"),
};

static DATA: [[i16; 2161]; 1081] = unsafe { U.data };

/// Estimated elevation in meters at a given latitude and longitude. Interpolates between known points.
pub fn elevation(lat: f64, lon: f64) -> f64 {
    assert!(
        (-90.0..=90.0).contains(&lat),
        "Latitude out of bounds [-90.0, 90.0]"
    );
    assert!(
        (-180.0..=180.0).contains(&lon),
        "Longitude out of bounds [-180.0, 180.0]"
    );

    // Grid dimensions and step size
    let lat_min = -90.0;
    let lon_min = -180.0;
    let lat_step = 180.0 / 1080.0; // ≈ 0.1667°
    let lon_step = 360.0 / 2160.0;

    // Position in the grid
    let lat_pos = (lat - lat_min) / lat_step;
    let lon_pos = (lon - lon_min) / lon_step;

    let y = lat_pos.floor() as usize;
    let x = lon_pos.floor() as usize;

    // Clamp to avoid going out of bounds
    let y = y.min(1079); // since we use y+1
    let x = x.min(2159); // since we use x+1

    // Fractional offsets based on the clamped indices
    let dy = lat_pos - y as f64;
    let dx = lon_pos - x as f64;

    let q11 = DATA[y][x] as f64;
    let q12 = DATA[y][x + 1] as f64;
    let q21 = DATA[y + 1][x] as f64;
    let q22 = DATA[y + 1][x + 1] as f64;

    // Bilinear interpolation
q11 * (1.0 - dx) * (1.0 - dy) + q12 * dx * (1.0 - dy) + q21 * (1.0 - dx) * dy + q22 * dx * dy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boundary_values() {
        // Latitude 90°, longitude 0° should correspond exactly to DATA[1080][1080]
        let expected = DATA[1080][1080] as f64;
        assert_eq!(elevation(90.0, 0.0), expected);

        // Latitude 0°, longitude 180° should correspond exactly to DATA[540][2160]
        let expected = DATA[540][2160] as f64;
        assert_eq!(elevation(0.0, 180.0), expected);
    }
}
