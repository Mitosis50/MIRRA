//! Versioned, directly stable-memory-backed state.

use std::borrow::Cow;

use candid::{Decode, Encode, Principal};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    storable::Bound,
    DefaultMemoryImpl, StableBTreeMap, StableCell, Storable,
};

use crate::types::{
    ConfigRecord, CorrectionRecord, ExpertRecord, GovernanceEvent, StoredReceipt, SCHEMA_VERSION,
};

pub const CONFIG_MEMORY_ID: u8 = 0;
pub const SEQUENCE_MEMORY_ID: u8 = 1;
pub const EXPERTS_MEMORY_ID: u8 = 2;
pub const PENALTIES_MEMORY_ID: u8 = 3;
pub const CORRECTIONS_MEMORY_ID: u8 = 4;
pub const RECEIPTS_MEMORY_ID: u8 = 5;
pub const WRITERS_MEMORY_ID: u8 = 6;
pub const GOVERNANCE_EVENTS_MEMORY_ID: u8 = 7;
pub const GOVERNANCE_SEQUENCE_MEMORY_ID: u8 = 8;

type Memory = VirtualMemory<DefaultMemoryImpl>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Key32(pub [u8; 32]);

impl Key32 {
    pub fn from_slice(bytes: &[u8], field: &str) -> Result<Self, String> {
        let value: [u8; 32] = bytes
            .try_into()
            .map_err(|_| format!("{field} must contain exactly 32 bytes"))?;
        Ok(Self(value))
    }

    pub fn to_vec(self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl Storable for Key32 {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(&self.0)
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Self(
            bytes
                .as_ref()
                .try_into()
                .expect("corrupt 32-byte stable key"),
        )
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 32,
        is_fixed_size: true,
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PrincipalKey(pub [u8; 30]);

impl PrincipalKey {
    pub fn new(principal: &Principal) -> Self {
        let bytes = principal.as_slice();
        assert!(bytes.len() <= 29, "ICP principal exceeds 29 bytes");
        let mut output = [0; 30];
        output[0] = bytes.len() as u8;
        output[1..1 + bytes.len()].copy_from_slice(bytes);
        Self(output)
    }
}

impl Storable for PrincipalKey {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(&self.0)
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Self(
            bytes
                .as_ref()
                .try_into()
                .expect("corrupt principal stable key"),
        )
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 30,
        is_fixed_size: true,
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Penalty(pub i128);

impl Storable for Penalty {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(self.0.to_be_bytes().to_vec())
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        let raw: [u8; 16] = bytes
            .as_ref()
            .try_into()
            .expect("corrupt Q64.64 stable penalty");
        Self(i128::from_be_bytes(raw))
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 16,
        is_fixed_size: true,
    };
}

macro_rules! candid_storable {
    ($ty:ty, $max_size:expr) => {
        impl Storable for $ty {
            fn to_bytes(&self) -> Cow<'_, [u8]> {
                Cow::Owned(Encode!(self).expect("stable Candid encoding failed"))
            }

            fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
                Decode!(bytes.as_ref(), $ty).expect("stable Candid decoding failed")
            }

            const BOUND: Bound = Bound::Bounded {
                max_size: $max_size,
                is_fixed_size: false,
            };
        }
    };
}

candid_storable!(ConfigRecord, 512);
candid_storable!(ExpertRecord, 1_024);
candid_storable!(CorrectionRecord, 8_192);
candid_storable!(StoredReceipt, 2_048);
candid_storable!(GovernanceEvent, 1_024);

pub struct Storage {
    config: StableCell<ConfigRecord, Memory>,
    sequence: StableCell<u64, Memory>,
    experts: StableBTreeMap<Key32, ExpertRecord, Memory>,
    penalties: StableBTreeMap<Key32, Penalty, Memory>,
    corrections: StableBTreeMap<u64, CorrectionRecord, Memory>,
    receipts: StableBTreeMap<Key32, StoredReceipt, Memory>,
    writers: StableBTreeMap<PrincipalKey, u8, Memory>,
    governance_events: StableBTreeMap<u64, GovernanceEvent, Memory>,
    governance_sequence: StableCell<u64, Memory>,
}

impl Storage {
    pub fn init(memory: DefaultMemoryImpl) -> Self {
        let manager = MemoryManager::init(memory);
        let config = StableCell::init(
            manager.get(MemoryId::new(CONFIG_MEMORY_ID)),
            ConfigRecord::default(),
        )
        .expect("failed to initialize config stable cell");
        assert_eq!(
            config.get().schema_version,
            SCHEMA_VERSION,
            "unsupported MIRRA stable schema"
        );
        Self {
            config,
            sequence: StableCell::init(manager.get(MemoryId::new(SEQUENCE_MEMORY_ID)), 0)
                .expect("failed to initialize sequence stable cell"),
            experts: StableBTreeMap::init(manager.get(MemoryId::new(EXPERTS_MEMORY_ID))),
            penalties: StableBTreeMap::init(manager.get(MemoryId::new(PENALTIES_MEMORY_ID))),
            corrections: StableBTreeMap::init(manager.get(MemoryId::new(CORRECTIONS_MEMORY_ID))),
            receipts: StableBTreeMap::init(manager.get(MemoryId::new(RECEIPTS_MEMORY_ID))),
            writers: StableBTreeMap::init(manager.get(MemoryId::new(WRITERS_MEMORY_ID))),
            governance_events: StableBTreeMap::init(
                manager.get(MemoryId::new(GOVERNANCE_EVENTS_MEMORY_ID)),
            ),
            governance_sequence: StableCell::init(
                manager.get(MemoryId::new(GOVERNANCE_SEQUENCE_MEMORY_ID)),
                0,
            )
            .expect("failed to initialize governance sequence stable cell"),
        }
    }

    pub fn config(&self) -> ConfigRecord {
        self.config.get().clone()
    }

    pub fn replace_config(&mut self, config: ConfigRecord) {
        self.config
            .set(config)
            .expect("config write exceeded the frozen stable bound");
    }

    pub fn sequence(&self) -> u64 {
        *self.sequence.get()
    }

    pub fn expert(&self, key: &Key32) -> Option<ExpertRecord> {
        self.experts.get(key)
    }

    pub fn insert_expert(&mut self, key: Key32, record: ExpertRecord) {
        self.experts.insert(key, record);
    }

    pub fn experts(&self) -> Vec<(Key32, ExpertRecord)> {
        self.experts.iter().collect()
    }

    pub fn expert_page(&self, start_after: Option<Key32>, limit: usize) -> Vec<ExpertRecord> {
        self.experts
            .iter()
            .filter(|(key, _)| start_after.is_none_or(|start| *key > start))
            .take(limit)
            .map(|(_, record)| record)
            .collect()
    }

    pub fn active_experts(&self) -> Vec<(Key32, ExpertRecord)> {
        self.experts
            .iter()
            .filter(|(_, record)| record.active)
            .collect()
    }

    pub fn penalty(&self, key: &Key32) -> i128 {
        self.penalties.get(key).map(|value| value.0).unwrap_or(0)
    }

    pub fn set_penalty(&mut self, key: Key32, penalty: i128) {
        self.penalties.insert(key, Penalty(penalty));
    }

    pub fn receipt(&self, event_key: &Key32) -> Option<StoredReceipt> {
        self.receipts.get(event_key)
    }

    pub fn correction(&self, sequence: u64) -> Option<CorrectionRecord> {
        self.corrections.get(&sequence)
    }

    pub fn governance_event(&self, sequence: u64) -> Option<GovernanceEvent> {
        self.governance_events.get(&sequence)
    }

    pub fn writer_roles(&self, principal: &Principal) -> u8 {
        self.writers.get(&PrincipalKey::new(principal)).unwrap_or(0)
    }

    pub fn set_writer_roles(&mut self, principal: &Principal, roles: u8) {
        let key = PrincipalKey::new(principal);
        if roles == 0 {
            self.writers.remove(&key);
        } else {
            self.writers.insert(key, roles);
        }
    }

    pub fn append_governance_event(
        &mut self,
        actor: Principal,
        action: &str,
        subject: Option<Principal>,
        roles: Option<u8>,
        now_ns: u64,
    ) -> GovernanceEvent {
        let sequence = self
            .governance_sequence
            .get()
            .checked_add(1)
            .expect("governance event sequence overflow");
        let event = GovernanceEvent {
            seq: sequence,
            actor,
            action: action.into(),
            subject,
            roles,
            recorded_at_ns: now_ns,
        };
        self.governance_events.insert(sequence, event.clone());
        self.governance_sequence
            .set(sequence)
            .expect("governance event sequence write failed");
        event
    }

    pub fn commit_correction(
        &mut self,
        sequence: u64,
        penalties: &[(Key32, i128)],
        correction: CorrectionRecord,
        event_key: Key32,
        receipt: StoredReceipt,
        config: ConfigRecord,
    ) {
        for (key, penalty) in penalties {
            self.penalties.insert(*key, Penalty(*penalty));
        }
        self.corrections.insert(sequence, correction);
        self.receipts.insert(event_key, receipt);
        self.sequence
            .set(sequence)
            .expect("sequence stable-cell write failed");
        self.replace_config(config);
    }

    pub fn expert_count(&self) -> u64 {
        self.experts.len()
    }

    pub fn active_expert_count(&self) -> u64 {
        self.experts
            .iter()
            .filter(|(_, record)| record.active)
            .count() as u64
    }

    pub fn correction_count(&self) -> u64 {
        self.corrections.len()
    }

    pub fn receipt_count(&self) -> u64 {
        self.receipts.len()
    }

    pub fn writer_count(&self) -> u64 {
        self.writers.len()
    }

    pub fn governance_event_count(&self) -> u64 {
        self.governance_events.len()
    }
}
