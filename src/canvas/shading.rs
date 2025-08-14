// Centralized shading lookup table and saturating mapping from [0,1] to ASCII chars.

// Keep this sorted from darkest to brightest for consistency with shading values.
pub const GRAYSCALE_LUT: &[char] = &[
    '.', ':', '-', '"', '+', '=', 'c', 'o', '*', '%', '#', 'M', '@',
];

/// Maps a shading value in [0.0, 1.0] to a character using the LUT.
/// Values outside the range are clamped to [0.0, 1.0].
#[inline]
pub fn shade_to_char(value: f32) -> char {
    if GRAYSCALE_LUT.is_empty() {
        return ' ';
    }

    // Clamp value to [0, 1]
    let v = value.max(0.0).min(1.0);

    // Map to [0, len-1], biased so 1.0 goes to the last index
    let len = GRAYSCALE_LUT.len() as f32;
    let idx = (v * (len - 1.0)).round() as usize;
    GRAYSCALE_LUT[idx]
}
