use std::{collections::BTreeSet, ops::{Add, Mul}};

use erreur::*;

use crate::MpcAddr;

pub fn eval_poly<ScalarType>(x: ScalarType, coef_vec: &[ScalarType]) -> ScalarType
where
    ScalarType: Clone + Default + Add<Output = ScalarType> + Mul<Output = ScalarType>,
{
    let mut y = ScalarType::default();
    for coef in coef_vec.iter().rev() {
        y = y * x.clone() + coef.clone();
    }
    y
}

pub fn eval_polycom<ScalarType, PointType>(x: ScalarType, coef_com_vec: &[PointType]) -> PointType
where
    ScalarType: Clone + Default + Add<Output = ScalarType> + Mul<PointType, Output = PointType>,
    PointType:
        Clone + Default + Add<PointType, Output = PointType> + Mul<ScalarType, Output = PointType>,
{
    let mut y = PointType::default();
    for coef_com in coef_com_vec.iter().rev() {
        y = (y * x.clone()) + coef_com.clone();
    }
    y
}

pub fn lagrange_lambda(
    id: MpcAddr,
    signers: BTreeSet<MpcAddr>,
) -> Resultat<(i64, i64)>
{
    let mut num: i64 = 1;
    let mut den: i64 = 1;
    let id = id.member_id() as i64;
    for j in signers.iter() {
        let j = j.member_id() as i64;
        if id == j {
            continue;
        }
        num *= j;
        den *= j - id;
    }
    assert_throw!(den != 0);

    Ok((num, den))
}
