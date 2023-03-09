use crate::error::Error;
use crate::host::ISMPHost;
use crate::messaging::RequestMessage;
use crate::prelude::Vec;
use codec::{Decode, Encode};
use core::time::Duration;

pub type StateMachineId = u64;
pub type ConsensusClientId = u64;
pub const ETHEREUM_CONSENSUS_CLIENT_ID: ConsensusClientId = 100;
pub const GNOSIS_CONSENSUS_CLIENT_ID: ConsensusClientId = 200;

#[derive(Debug, Clone, Encode, Decode)]
pub struct StateCommitment {
    /// Timestamp in nanoseconds
    pub timestamp: u64,
    pub commitment_root: Vec<u8>,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct IntermediateState {
    pub height: StateMachineHeight,
    pub commitment: StateCommitment,
}

#[derive(Debug, Clone, Copy, Encode, Decode)]
pub struct StateMachineHeight {
    pub id: StateMachineId,
    pub height: u64,
}

pub trait ConsensusClient {
    /// Should decode the scale encoded trusted consensus state and new consensus proof, verifying that:
    /// - the client isn't frozen yet
    /// - that the client hasn't elapsed it's unbonding period
    /// - check for byzantine behaviour
    /// - verify the consensus proofs
    /// - finally return the new consensusState and state commitments.
    fn verify(
        &self,
        host: &dyn ISMPHost,
        trusted_consensus_state: Vec<u8>,
        proof: Vec<u8>,
    ) -> Result<(Vec<u8>, Vec<IntermediateState>), Error>;

    /// Check if the client has expired since the last update
    fn is_expired(&self, host: &dyn ISMPHost) -> Result<bool, Error> {
        let host_timestamp = host.host_timestamp();
        let unbonding_period = self.unbonding_period();
        let last_update = host.consensus_update_time(self.consensus_id())?;
        Ok(host_timestamp.saturating_sub(last_update) > unbonding_period)
    }

    /// Return the Consensus Client Id
    fn consensus_id(&self) -> ConsensusClientId;

    /// Return unbonding period
    fn unbonding_period(&self) -> Duration;

    /// Verify membership proof of request
    fn verify_request(&self, msg: RequestMessage) -> Result<(), Error>;

    /// Verify membership proof of response
    fn verify_response(&self, msg: RequestMessage) -> Result<(), Error>;
}
