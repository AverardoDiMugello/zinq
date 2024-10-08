use std::collections::HashMap;

pub struct Impdef {
    bools: HashMap<String, bool>,
    ints: HashMap<String, u64>,
}

impl Impdef {
    pub fn bool(&self, key: &str) -> bool {
        self.bools
            .get(key)
            .and_then(|val| Some(val.clone()))
            .expect(format!("Unknown IMPDEF boolean {key}").as_str())
    }

    pub fn int(&self, key: &str) -> u64 {
        self.ints
            .get(key)
            .and_then(|val| Some(val.clone()))
            .expect(format!("Unknown IMPDEF integer {key}").as_str())
    }
}

impl Default for Impdef {
    fn default() -> Self {
        let mut bools = HashMap::new();
        bools.insert(
            String::from("Has Implicit Error Synchronization Barrier before Exception"),
            false,
        );
        bools.insert(
            String::from("BBM level 1 or 2 support nT bit causes Translation Fault"),
            true,
        );
        bools.insert(String::from("Trivial PSTATE.PACM implementation"), true);
        bools.insert(
            String::from("descriptor[15:12] for 64KB granule are OA[51:48]"),
            false,
        );
        bools.insert(
            String::from("BADDR expresses 52 bits for 64KB granule"),
            false,
        );
        bools.insert(
            String::from("Translation fault on misprogrammed contiguous bit"),
            false,
        );
        bools.insert(
            String::from("Permission fault on EL0 IC_IVAU execution"),
            false,
        );
        bools.insert(String::from("SCR_EL3.SIF affects EPAN"), false);
        bools.insert(String::from("SCR_EL3.SIF affects EPAN"), false);
        bools.insert(String::from("Realm EL2&0 regime affects EPAN"), false);
        bools.insert(String::from("Fault on TxSZ value below minimum"), false);
        bools.insert(String::from("Fault on TxSZ value above maximum"), false);
        bools.insert(
            String::from("Apply effective shareability at stage 1"),
            true,
        );
        bools.insert(
            String::from("Generate access flag fault on IC/DC operations"),
            true,
        );
        bools.insert(String::from("Has 4K Translation Granule"), true);
        bools.insert(String::from("Has 16K Translation Granule"), true);
        bools.insert(String::from("Has 64K Translation Granule"), true);
        bools.insert(String::from("Has Stage 2 4K Translation Granule"), true);
        bools.insert(String::from("Has Stage 2 16K Translation Granule"), true);
        bools.insert(String::from("Has Stage 2 64K Translation Granule"), true);
        bools.insert(String::from("Secure-only implementation"), true);
        bools.insert(String::from("Non-secure only implementation"), false);
        bools.insert(String::from("EL3 trap priority when SDD == '1'"), false);
        bools.insert(String::from("HCR_EL2.NV1 is implemented as RAZ"), false);
        bools.insert(
            String::from("Programming HCR_EL2.<NV,NV2> to '10' behaves as '11'"),
            false,
        );

        let mut ints = HashMap::new();
        ints.insert(String::from("Block BBM support level"), 0x2);
        ints.insert(String::from("Maximum Physical Address Size"), 56);
        ints.insert(String::from("TG0 encoded granule size"), 0b00);
        ints.insert(String::from("TG1 encoded granule size"), 0b10);

        Self { bools, ints }
    }
}
