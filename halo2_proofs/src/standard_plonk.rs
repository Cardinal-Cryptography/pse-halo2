//! Simple standard Plonk circuit configuration. Helpful for deserializing verification keys.

use ff::Field;

use crate::{
    circuit::{Layouter, SimpleFloorPlanner},
    plonk::{
        Circuit,
        ConstraintSystem, Error,
    },
    poly::Rotation,
};

fn configure_circuit<Fr: Field>(meta: &mut ConstraintSystem<Fr>) {
    let [a, b, c] = [(); 3].map(|_| meta.advice_column());
    let [q_a, q_b, q_c, q_ab, constant] = [(); 5].map(|_| meta.fixed_column());
    let instance = meta.instance_column();

    [a, b, c].map(|column| meta.enable_equality(column));

    meta.create_gate(
        "q_a·a + q_b·b + q_c·c + q_ab·a·b + constant + instance = 0",
        |meta| {
            let [a, b, c] = [a, b, c].map(|column| meta.query_advice(column, Rotation::cur()));
            let [q_a, q_b, q_c, q_ab, constant] = [q_a, q_b, q_c, q_ab, constant]
                .map(|column| meta.query_fixed(column, Rotation::cur()));
            let instance = meta.query_instance(instance, Rotation::cur());
            Some(
                q_a * a.clone()
                    + q_b * b.clone()
                    + q_c * c
                    + q_ab * a * b
                    + constant
                    + instance,
            )
        },
    );
}

/// Standard Plonk circuit. Warning: usable only for a configuration phase!
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct StandardPlonk;

impl<Fr: Field> Circuit<Fr> for StandardPlonk {
    type Config = ();
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<Fr>) -> Self::Config {
        configure_circuit(meta)
    }

    fn synthesize(
        &self,
        _: Self::Config,
        _: impl Layouter<Fr>,
    ) -> Result<(), Error> {
        unreachable!("`StandardPlonk` is intended only for a configuration purposes")
    }
}
