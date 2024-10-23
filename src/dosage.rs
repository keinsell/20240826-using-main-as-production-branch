use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
use uom::si::f64::Mass;
use uom::si::mass::{gram, kilogram, milligram};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dosage {
    #[serde(with = "mass_serde")]
    value: Mass,
}

impl Dosage {
    pub fn new(value: f64, unit: &str) -> Result<Self, String> {
        let mass = match unit.to_lowercase().as_str() {
            "mg" | "milligram" | "milligrams" => Mass::new::<milligram>(value),
            "g" | "gram" | "grams" => Mass::new::<gram>(value),
            "kg" | "kilogram" | "kilograms" => Mass::new::<kilogram>(value),
            _ => return Err(format!("Unsupported unit: {}", unit)),
        };
        Ok(Dosage { value: mass })
    }

    pub fn to_milligrams(&self) -> f64 {
        self.value.get::<milligram>()
    }

    pub fn to_grams(&self) -> f64 {
        self.value.get::<gram>()
    }

    pub fn to_kilograms(&self) -> f64 {
        self.value.get::<kilogram>()
    }
}

impl fmt::Display for Dosage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (value, unit) = if self.value.get::<kilogram>() >= 1.0 {
            (self.to_kilograms(), "kg")
        } else if self.value.get::<gram>() >= 1.0 {
            (self.to_grams(), "g")
        } else {
            (self.to_milligrams(), "mg")
        };
        write!(f, "{:.2} {}", value, unit)
    }
}

impl Add for Dosage {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Dosage { value: self.value + other.value }
    }
}

impl Sub for Dosage {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Dosage { value: self.value - other.value }
    }
}

impl Mul<f64> for Dosage {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Dosage { value: self.value * scalar }
    }
}

impl Div<f64> for Dosage {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Dosage { value: self.value / scalar }
    }
}

mod mass_serde {
    use serde::{self, Deserialize, Serializer, Deserializer};
    use uom::si::f64::Mass;
    use uom::si::mass::kilogram;

    pub fn serialize<S>(mass: &Mass, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f64(mass.get::<kilogram>())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Mass, D::Error>
    where
        D: Deserializer<'de>,
    {
        let kg: f64 = Deserialize::deserialize(deserializer)?;
        Ok(Mass::new::<kilogram>(kg))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dosage_creation() {
        let dosage = Dosage::new(500.0, "mg").unwrap();
        assert_eq!(dosage.to_milligrams(), 500.0);
    }

    #[test]
    fn test_dosage_display() {
        let dosage = Dosage::new(1.5, "g").unwrap();
        assert_eq!(format!("{}", dosage), "1.50 g");
    }

    #[test]
    fn test_dosage_arithmetic() {
        let dosage1 = Dosage::new(500.0, "mg").unwrap();
        let dosage2 = Dosage::new(0.5, "g").unwrap();
        let sum = dosage1 + dosage2;
        assert_eq!(sum.to_milligrams(), 1000.0);
    }

    #[test]
    fn test_dosage_serialization() {
        let dosage = Dosage::new(1.5, "g").unwrap();
        let serialized = serde_json::to_string(&dosage).unwrap();
        let deserialized: Dosage = serde_json::from_str(&serialized).unwrap();
        assert_eq!(dosage, deserialized);
    }
}
