pub mod decay_mode;
pub mod half_life;

pub use decay_mode::DecayMode;
pub use half_life::HalfLife;

use flagset::FlagSet;
use serde::{Deserialize, Serialize};
use serde_with::DeserializeFromStr;
use std::hash::Hash;

use crate::error::Error;
use crate::regex;

#[rustfmt::skip]
#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Symbol {
    SF, H , He, Li, Be, B, C, N, O, F, Ne, Na, Mg, Al, Si, P, S, Cl, Ar, K, Ca,
    Sc, Ti, V, Cr, Mn, Fe, Co, Ni, Cu, Zn, Ga, Ge, As, Se, Br, Kr, Rb, Sr, Y,
    Zr, Nb, Mo, Tc, Ru, Rh, Pd, Ag, Cd, In, Sn, Sb, Te, I, Xe, Cs, Ba, La, Ce,
    Pr, Nd, Pm, Sm, Eu, Gd, Tb, Dy, Ho, Er, Tm, Yb, Lu, Hf, Ta, W, Re, Os, Ir,
    Pt, Au, Hg, Tl, Pb, Bi, Po, At, Rn, Fr, Ra, Ac, Th, Pa, U, Np, Pu, Am, Cm,
    Bk, Cf, Es, Fm, Md, No, Lr, Rf, Db, Sg, Bh, Hs, Mt, Ds, Rg, Cn, Nh, Fl, Mc,
    Lv, Ts, Og, None,
}

serde_plain::derive_fromstr_from_deserialize!(Symbol);
serde_plain::derive_display_from_serialize!(Symbol);

impl Default for Symbol {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetastableState {
    #[serde(rename = "m")]
    M,
    #[serde(rename = "n")]
    N,
}

serde_plain::derive_fromstr_from_deserialize!(MetastableState);
serde_plain::derive_display_from_serialize!(MetastableState);

#[derive(Debug, Default, Hash, Clone, Copy, PartialEq, Eq, DeserializeFromStr)]
pub struct Nuclide {
    pub symbol: Symbol,
    pub mass_number: Option<u64>,
    pub meta: Option<MetastableState>,
}

impl std::fmt::Display for Nuclide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.symbol {
            Symbol::None => write!(f, "(None)")?,
            _ => {
                write!(f, "{}", self.symbol)?;
                if self.mass_number.is_some() {
                    write!(f, "-{}", self.mass_number.unwrap())?;
                    if self.meta.is_some() {
                        write!(f, "{}", self.meta.unwrap())?;
                    }
                }
            }
        }

        Ok(())
    }
}

impl std::str::FromStr for Nuclide {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex!(r"(?P<symbol>\w+)((?:-)(?P<mass>\d+)(?P<meta>\w)?)?");

        let captures = re
            .captures(s)
            .ok_or_else(|| Error::InvalidNuclide(s.to_string()))?;

        let symbol = captures.name("symbol").unwrap().as_str().parse().unwrap();
        let mass_number = captures.name("mass").map(|a| a.as_str().parse().unwrap());
        let meta = captures.name("meta").map(|m| m.as_str().parse().unwrap());

        Ok(Self {
            symbol,
            mass_number,
            meta,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Progeny {
    pub decay_mode: FlagSet<decay_mode::DecayMode>,
    pub branch_rate: f64,
    pub nuclide: Nuclide,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn nuclide_from_string() {
        let i131: Nuclide = "I-131".parse().unwrap();
        assert_eq!(i131.symbol, Symbol::I);
        assert_eq!(i131.mass_number, Some(131));
        assert_eq!(i131.meta, None);

        let tc99m: Nuclide = "Tc-99m".parse().unwrap();
        assert_eq!(tc99m.symbol, Symbol::Tc);
        assert_eq!(tc99m.mass_number, Some(99));
        assert_eq!(tc99m.meta, Some(MetastableState::M));
    }

    #[test]
    fn nuclide_to_string() {
        let i131: Nuclide = "I-131".parse().unwrap();
        assert_eq!(&i131.to_string(), "I-131");

        let tc99m: Nuclide = "Tc-99m".parse().unwrap();
        assert_eq!(&tc99m.to_string(), "Tc-99m");
    }
}
