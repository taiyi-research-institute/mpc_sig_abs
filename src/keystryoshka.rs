use std::{
    collections::{BTreeMap, BTreeSet},
    ops::{Add, Mul},
};

use erreur::*;
use serde::{Deserialize, Serialize};

use crate::MpcAddr;

/// Matryoshka keystore
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct Keystryoshka<ScalarType, PointType>
where
    ScalarType: Clone
        + Default // Zero
        + Add<Output = ScalarType>
        + Mul<Output = ScalarType>
        + Mul<PointType, Output = PointType>,
    PointType: Clone
        + Default // Zero, aka Identity, aka Infinity
        + std::ops::Add<Output = PointType>
        + std::ops::Mul<ScalarType, Output = PointType>,
{
    pub ui_dict: BTreeMap<u16, ScalarType>,
    pub xi_dict: BTreeMap<u16, ScalarType>,
    pub vss_com_grid: BTreeMap<u16, BTreeMap<MpcAddr, Vec<PointType>>>, // (group, member(poly), coef)

    pub ids: BTreeSet<MpcAddr>,
    pub aux: Option<Vec<u8>>,
}

impl<ScalarType, PointType> Keystryoshka<ScalarType, PointType>
where
    ScalarType: Clone
        + Default // Zero
        + Add<Output = ScalarType>
        + Mul<Output = ScalarType>
        + Mul<PointType, Output = PointType>,
    PointType: Clone
        + Default // Zero, aka Identity, aka Infinity
        + std::ops::Add<PointType, Output = PointType>
        + std::ops::Mul<ScalarType, Output = PointType>,
{
    pub fn th(&self, gid: u16) -> Resultat<usize> {
        let vss_com_dict = self
            .vss_com_grid
            .get(&gid)
            .ifnone("NoSuchGroup", gid.to_string())?;

        let vss_com = vss_com_dict
            .values()
            .next()
            .ifnone("EmptyVssCom", format!("at group {}", gid.to_string()))?;

        Ok(vss_com.len())
    }

    pub fn pk(&self) -> Resultat<PointType> {
        assert_throw!(self.vss_com_grid.len() > 0);
        let mut res = PointType::default();
        for vss_com_dict in self.vss_com_grid.values() {
            assert_throw!(vss_com_dict.len() > 0);
            for vss_com in vss_com_dict.values() {
                assert_throw!(vss_com.len() > 0);
                res = res + vss_com[0].clone();
            }
        }
        Ok(res)
    }
}
