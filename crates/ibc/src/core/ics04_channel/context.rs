//! ICS4 (channel) context.

use core::time::Duration;

use super::packet::Sequence;
use crate::core::events::IbcEvent;
use crate::core::ics02_client::client_state::ClientState;
use crate::core::ics02_client::consensus_state::ConsensusState;
use crate::core::ics02_client::{ClientExecutionContext, ClientValidationContext};
use crate::core::ics03_connection::connection::ConnectionEnd;
use crate::core::ics04_channel::channel::ChannelEnd;
use crate::core::ics04_channel::commitment::PacketCommitment;
use crate::core::ics24_host::identifier::{ClientId, ConnectionId};
use crate::core::ics24_host::path::{
    ChannelEndPath, ClientConsensusStatePath, CommitmentPath, SeqSendPath,
};
use crate::core::{ContextError, ExecutionContext, ValidationContext};
use crate::prelude::*;

/// Methods required in send packet validation, to be implemented by the host
pub trait SendPacketValidationContext {
    type V: ClientValidationContext;
    type E: ClientExecutionContext;
    type AnyConsensusState: ConsensusState;
    type AnyClientState: ClientState<Self::V, Self::E>;

    /// Retrieve the context that implements all clients' `ValidationContext`.
    fn get_client_validation_context(&self) -> &Self::V;

    /// Returns the ChannelEnd for the given `port_id` and `chan_id`.
    fn channel_end(&self, channel_end_path: &ChannelEndPath) -> Result<ChannelEnd, ContextError>;

    /// Returns the ConnectionState for the given identifier `connection_id`.
    fn connection_end(&self, connection_id: &ConnectionId) -> Result<ConnectionEnd, ContextError>;

    /// Returns the ClientState for the given identifier `client_id`. Necessary dependency towards
    /// proof verification.
    fn client_state(&self, client_id: &ClientId) -> Result<Self::AnyClientState, ContextError>;

    fn client_consensus_state(
        &self,
        client_cons_state_path: &ClientConsensusStatePath,
    ) -> Result<Self::AnyConsensusState, ContextError>;

    fn get_next_sequence_send(&self, seq_send_path: &SeqSendPath)
        -> Result<Sequence, ContextError>;
}

impl<T> SendPacketValidationContext for T
where
    T: ValidationContext,
{
    type V = T::V;
    type E = T::E;
    type AnyConsensusState = T::AnyConsensusState;
    type AnyClientState = T::AnyClientState;

    fn get_client_validation_context(&self) -> &Self::V {
        self.get_client_validation_context()
    }

    fn channel_end(&self, channel_end_path: &ChannelEndPath) -> Result<ChannelEnd, ContextError> {
        self.channel_end(channel_end_path)
    }

    fn connection_end(&self, connection_id: &ConnectionId) -> Result<ConnectionEnd, ContextError> {
        self.connection_end(connection_id)
    }

    fn client_state(&self, client_id: &ClientId) -> Result<T::AnyClientState, ContextError> {
        self.client_state(client_id)
    }

    fn client_consensus_state(
        &self,
        client_cons_state_path: &ClientConsensusStatePath,
    ) -> Result<T::AnyConsensusState, ContextError> {
        self.consensus_state(client_cons_state_path)
    }

    fn get_next_sequence_send(
        &self,
        seq_send_path: &SeqSendPath,
    ) -> Result<Sequence, ContextError> {
        self.get_next_sequence_send(seq_send_path)
    }
}

/// Methods required in send packet execution, to be implemented by the host
pub trait SendPacketExecutionContext: SendPacketValidationContext {
    fn store_next_sequence_send(
        &mut self,
        seq_send_path: &SeqSendPath,
        seq: Sequence,
    ) -> Result<(), ContextError>;

    fn store_packet_commitment(
        &mut self,
        commitment_path: &CommitmentPath,
        commitment: PacketCommitment,
    ) -> Result<(), ContextError>;

    /// Ibc events
    fn emit_ibc_event(&mut self, event: IbcEvent) -> Result<(), ContextError>;

    /// Logging facility
    fn log_message(&mut self, message: String) -> Result<(), ContextError>;
}

impl<T> SendPacketExecutionContext for T
where
    T: ExecutionContext,
{
    fn store_next_sequence_send(
        &mut self,
        seq_send_path: &SeqSendPath,
        seq: Sequence,
    ) -> Result<(), ContextError> {
        self.store_next_sequence_send(seq_send_path, seq)
    }

    fn store_packet_commitment(
        &mut self,
        commitment_path: &CommitmentPath,
        commitment: PacketCommitment,
    ) -> Result<(), ContextError> {
        self.store_packet_commitment(commitment_path, commitment)
    }

    fn emit_ibc_event(&mut self, event: IbcEvent) -> Result<(), ContextError> {
        self.emit_ibc_event(event)
    }

    fn log_message(&mut self, message: String) -> Result<(), ContextError> {
        self.log_message(message)
    }
}

pub(crate) fn calculate_block_delay(
    delay_period_time: &Duration,
    max_expected_time_per_block: &Duration,
) -> u64 {
    let delay_period_time = delay_period_time.as_secs();
    let max_expected_time_per_block = max_expected_time_per_block.as_secs();
    if max_expected_time_per_block == 0 {
        return 0;
    }
    if delay_period_time % max_expected_time_per_block == 0 {
        return delay_period_time / max_expected_time_per_block;
    }
    (delay_period_time / max_expected_time_per_block) + 1
}

#[cfg(test)]
mod tests {

    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::remainder_zero(10, 2, 5)]
    #[case::remainder_not_zero(10, 3, 4)]
    #[case::max_expected_zero(10, 0, 0)]
    #[case::delay_period_zero(0, 2, 0)]
    #[case::both_zero(0, 0, 0)]
    #[case::delay_less_than_max(10, 11, 1)]
    fn test_calculate_block_delay_zero(
        #[case] delay_period_time: u64,
        #[case] max_expected_time_per_block: u64,
        #[case] expected: u64,
    ) {
        assert_eq!(
            calculate_block_delay(
                &Duration::from_secs(delay_period_time),
                &Duration::from_secs(max_expected_time_per_block)
            ),
            expected
        );
    }
}
