use std::collections::BTreeMap;
use std::ops::{Add, Mul};

use erreur::*;
use serde::{Deserialize, Serialize};

use crate::MpcAddr;

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct Keystore<ScalarType, PointType>
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
    /// Party key
    pub u_i: ScalarType,
    /// signing key
    pub x_i: ScalarType,
    /// Commitment to the polynomial coefficients for each `member_id` within the same `group_id`.
    pub vss_com_dict: BTreeMap<MpcAddr, Vec<PointType>>,
    /// `(group_id, member_id)` of current shard.
    pub id: MpcAddr,
    /// Auxiliary data for the shard.
    pub aux: Option<Vec<u8>>,
}

impl<ScalarType, PointType> Keystore<ScalarType, PointType>
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
    pub fn th(&self) -> Resultat<usize> {
        let my_vss_com = self
            .vss_com_dict
            .get(&self.id)
            .ifnone("EmptyVssCom", format!("At shard_id={}", self.id))?;
        Ok(my_vss_com.len())
    }

    pub fn pk(&self) -> Resultat<PointType> {
        assert_throw!(self.vss_com_dict.len() > 0);
        let mut res = PointType::default();
        for vss_com in self.vss_com_dict.values() {
            assert_throw!(vss_com.len() > 0);
            res = res + vss_com[0].clone();
        }
        Ok(res)
    }
}
