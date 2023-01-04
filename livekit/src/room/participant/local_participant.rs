use super::ConnectionQuality;
use crate::proto::{data_packet, DataPacket, ParticipantInfo, UserPacket};
use crate::room::id::{ParticipantIdentity, ParticipantSid, TrackSid};
use crate::room::participant::{
    impl_participant_trait, ParticipantEvent, ParticipantInternalTrait, ParticipantShared,
    ParticipantTrait,
};
use crate::room::publication::TrackPublication;
use crate::room::RoomError;
use crate::rtc_engine::RTCEngine;
use parking_lot::RwLockReadGuard;
use std::collections::HashMap;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct LocalParticipant {
    shared: ParticipantShared,
    rtc_engine: Arc<RTCEngine>,
}

impl LocalParticipant {
    pub(crate) fn new(
        rtc_engine: Arc<RTCEngine>,
        sid: ParticipantSid,
        identity: ParticipantIdentity,
        name: String,
        metadata: String,
    ) -> Self {
        Self {
            shared: ParticipantShared::new(sid, identity, name, metadata),
            rtc_engine,
        }
    }

    pub async fn publish_data(
        &self,
        data: &[u8],
        kind: data_packet::Kind,
    ) -> Result<(), RoomError> {
        let data = DataPacket {
            kind: kind as i32,
            value: Some(data_packet::Value::User(UserPacket {
                participant_sid: self.sid().to_string(),
                payload: data.to_vec(),
                destination_sids: vec![],
            })),
        };

        self.rtc_engine
            .publish_data(&data, kind)
            .await
            .map_err(Into::into)
    }
}

impl ParticipantInternalTrait for LocalParticipant {
    fn update_info(self: &Arc<Self>, info: ParticipantInfo, _emit_events: bool) {
        self.shared.update_info(info);
    }

    fn set_speaking(&self, speaking: bool) {
        self.shared.set_speaking(speaking);
    }

    fn set_audio_level(&self, level: f32) {
        self.shared.set_audio_level(level);
    }

    fn set_connection_quality(&self, quality: ConnectionQuality) {
        self.shared.set_connection_quality(quality);
    }
}

impl_participant_trait!(LocalParticipant);