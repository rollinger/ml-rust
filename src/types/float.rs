#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fp8(u8);

impl Fp8 {
    const EXP_BITS: u8 = 4;
    const MANT_BITS: u8 = 3;
    const EXP_BIAS: i8 = 7;
    const MAX_EXP: u8 = 0x0F;
    const MAX_MANT: u8 = 0x07;

    /// Create from raw bits (e.g., from storage)
    pub fn from_bits(bits: u8) -> Self {
        Self(bits)
    }

    /// Extract raw bits (e.g., for storage)
    pub fn to_bits(self) -> u8 {
        self.0
    }

    /// Convert f32 → fp8
    pub fn from_f32(x: f32) -> Self {
        if x.is_nan() {
            return Self(0b0111_1111); // Quiet NaN
        }

        let sign = if x.is_sign_negative() { 1 } else { 0 };

        if x.is_infinite() {
            return Self((sign << 7) | (Self::MAX_EXP << 3));
        }

        if x == 0.0 {
            return Self((sign << 7)); // Zero
        }

        let abs = x.abs();
        let exp = abs.log2().floor() as i8;
        let biased_exp = exp + Self::EXP_BIAS;

        if biased_exp <= 0 {
            // Subnormal: treat as zero
            return Self((sign << 7)); // Optional: emulate subnormals
        }

        if biased_exp >= 0x0F {
            // Clamp to max finite value
            let bits = (sign << 7) | (Self::MAX_EXP << 3) | Self::MAX_MANT;
            return Self(bits);
        }

        let mant = abs / 2f32.powi(exp as i32) - 1.0;
        let mant_scaled = (mant * 8.0).round() as u8;

        let bits = (sign << 7) | ((biased_exp as u8) << 3) | (mant_scaled & 0x07);
        Self(bits)
    }

    /// Convert fp8 → f32
    pub fn to_f32(self) -> f32 {
        let bits = self.0;
        let sign = if bits & 0x80 != 0 { -1.0 } else { 1.0 };
        let exp = (bits >> 3) & 0x0F;
        let mant = bits & 0x07;

        match exp {
            0 => {
                // Zero or subnormal
                if mant == 0 {
                    sign * 0.0
                } else {
                    // Subnormal (not handled properly yet — approximated)
                    sign * (mant as f32 / 64.0)
                }
            }
            0x0F => {
                if mant == 0 {
                    sign * f32::INFINITY
                } else {
                    f32::NAN
                }
            }
            _ => {
                let exponent = (exp as i8) - Self::EXP_BIAS;
                let mantissa = 1.0 + (mant as f32 / 8.0);
                sign * mantissa * 2f32.powi(exponent as i32)
            }
        }
    }

    /// Convert f64 → fp8
    pub fn from_f64(x: f64) -> Self {
        Self::from_f32(x as f32)
    }

    /// Convert fp8 → f64
    pub fn to_f64(self) -> f64 {
        self.to_f32() as f64
    }
}


impl From<f32> for Fp8 {
    fn from(x: f32) -> Self {
        Self::from_f32(x)
    }
}

impl From<Fp8> for f32 {
    fn from(fp: Fp8) -> Self {
        fp.to_f32()
    }
}

impl From<f64> for Fp8 {
    fn from(x: f64) -> Self {
        Self::from_f64(x)
    }
}

impl From<Fp8> for f64 {
    fn from(fp: Fp8) -> Self {
        fp.to_f64()
    }
}

