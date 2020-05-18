/// This enum holds the packets used by the server and client.
///
/// It is not sorted alphabetically, rather it is sorted according to the
/// Packet ID, which is 8 bits long, of Minecraft server protocol version 14
/// (the one used in Minecraft Beta 1.7.3).
///
/// More information on the protocol can be found on this helpful [wiki page]
/// (https://wiki.vg/Protocol&oldid=510).
#[derive(Clone, Debug)]
pub enum PacketId {
    /* 0x00 */ KeepAlive,
    /* 0x01 */ LoginRequest,
    /* 0x02 */ Handshake,
    /* 0x03 */ ChatMessage,
    /* 0x04 */ TimeUpdate,
    /* 0x05 */ EntityEquipment,
    /* 0x06 */ SpawnPosition,
    /* 0x07 */ UseEntity,
    /* 0x08 */ UpdateHealth,
    /* 0x09 */ Respawn,
    /* 0x0a */ PlayerFlying,
    /* 0x0b */ PlayerPosition,
    /* 0x0c */ PlayerLook,
    /* 0x0d */ PlayerPositionAndLook,
    /* 0x0e */ PlayerDigging,
    /* 0x0f */ PlayerBlockPlacement,
    /* 0x10 */ HoldingChange,
    /* 0x11 */ UseBed,
    /* 0x12 */ UseAnimation,
    /* 0x13 */ EntityAction,
    /* 0x14 */ NamedEntitySpawn,
    /* 0x15 */ PickupSpawn,
    /* 0x16 */ CollectItem,
    /* 0x17 */ AddObject,
    /* 0x18 */ MobSpawn,
    /* 0x19 */ AddPainting,
    // UNUSED (0x1a)
    /* 0x1b */ StanceUpdate,
    /* 0x1c */ EntityVelocity,
    /* 0x1d */ DestroyEntity,
    /* 0x1e */ EntityUnchanged,
    /* 0x1f */ EntityRelativeMove,
    /* 0x20 */ EntityLook,
    /* 0x21 */ EntityLookAndRelativeMove,
    /* 0x22 */ EntityTeleport,
    // UNUSED (0x23 - 0x25 inclusve)
    /* 0x26 */ EntityStatus,
    /* 0x27 */ AttachEntity,
    /* 0x28 */ EntityMetadata,
    // UNUSED (0x29 - 0x31 inclusive)
    /* 0x32 */ PreChunk,
    /* 0x33 */ MapChunk,
    /* 0x34 */ MultiBlockChange,
    /* 0x35 */ BlockChange,
    /* 0x36 */ BlockAction,
    // UNUSED (0x36 - 0x3B inclusive)
    /* 0x3c */ Explosion,
    /* 0x3d */ SoundEffect,
    // UNUSED (0x3E - 0x45 inclusive)
    /* 0x46 */ NewState,
    /* 0x47 */ Thunderbolt,
    // UNUSED (0x48 - 0x63 inclusive)
    /* 0x64 */ OpenWindow,
    /* 0x65 */ CloseWindow,
    /* 0x66 */ WindowClick,
    /* 0x67 */ SetSlot,
    /* 0x68 */ WindowItems,
    /* 0x69 */ UpdateProgressBar,
    /* 0x6a */ Transaction,
    // UNUSED (0x6B - 0x81 inclusive)
    /* 0x82 */ UpdateSign,
    /* 0x83 */ ItemData,
    /* 0x84 */ IncrementStatistic,
    // UNUSED (0x85 - 0xFE inclusive)
    /* 0xFF */ DisconnectOrKick,
}

impl PacketId {
    pub fn packet_id(&self) -> u8 {
        match self {
            Self::KeepAlive => 0x00,
            Self::LoginRequest => 0x01,
            Self::Handshake => 0x02,
            Self::ChatMessage => 0x03,
            Self::TimeUpdate => 0x04,
            Self::EntityEquipment => 0x05,
            Self::SpawnPosition => 0x06,
            Self::UseEntity => 0x07,
            Self::UpdateHealth => 0x08,
            Self::Respawn => 0x09,
            Self::PlayerFlying => 0x0a,
            Self::PlayerPosition => 0x0b,
            Self::PlayerLook => 0x0c,
            Self::PlayerPositionAndLook => 0x0d,
            Self::PlayerDigging => 0x0e,
            Self::PlayerBlockPlacement => 0x0f,
            Self::HoldingChange => 0x10,
            Self::UseBed => 0x11,
            Self::UseAnimation => 0x12,
            Self::EntityAction => 0x13,
            Self::NamedEntitySpawn => 0x14,
            Self::PickupSpawn => 0x15,
            Self::CollectItem => 0x16,
            Self::AddObject => 0x17,
            Self::MobSpawn => 0x18,
            Self::AddPainting => 0x19,
            Self::StanceUpdate => 0x1b,
            Self::EntityVelocity => 0x1c,
            Self::DestroyEntity => 0x1d,
            Self::EntityUnchanged => 0x1e,
            Self::EntityRelativeMove => 0x1f,
            Self::EntityLook => 0x20,
            Self::EntityLookAndRelativeMove => 0x21,
            Self::EntityTeleport => 0x22,
            Self::EntityStatus => 0x26,
            Self::AttachEntity => 0x27,
            Self::EntityMetadata => 0x28,
            Self::PreChunk => 0x32,
            Self::MapChunk => 0x33,
            Self::MultiBlockChange => 0x34,
            Self::BlockChange => 0x35,
            Self::BlockAction => 0x36,
            Self::Explosion => 0x3c,
            Self::SoundEffect => 0x3d,
            Self::NewState => 0x46,
            Self::Thunderbolt => 0x47,
            Self::OpenWindow => 0x64,
            Self::CloseWindow => 0x65,
            Self::WindowClick => 0x66,
            Self::SetSlot => 0x67,
            Self::WindowItems => 0x68,
            Self::UpdateProgressBar => 0x69,
            Self::Transaction => 0x6a,
            Self::UpdateSign => 0x82,
            Self::ItemData => 0x83,
            Self::IncrementStatistic => 0x84,
            Self::DisconnectOrKick => 0xFF,
        }
    }

    pub fn from_packet_id(id: u8) -> Option<Self> {
        match id {
            0x00 => Some(Self::KeepAlive),
            0x01 => Some(Self::LoginRequest),
            0x02 => Some(Self::Handshake),
            0x03 => Some(Self::ChatMessage),
            0x04 => Some(Self::TimeUpdate),
            0x05 => Some(Self::EntityEquipment),
            0x06 => Some(Self::SpawnPosition),
            0x07 => Some(Self::UseEntity),
            0x08 => Some(Self::UpdateHealth),
            0x09 => Some(Self::Respawn),
            0x0a => Some(Self::PlayerFlying),
            0x0b => Some(Self::PlayerPosition),
            0x0c => Some(Self::PlayerLook),
            0x0d => Some(Self::PlayerPositionAndLook),
            0x0e => Some(Self::PlayerDigging),
            0x0f => Some(Self::PlayerBlockPlacement),
            0x10 => Some(Self::HoldingChange),
            0x11 => Some(Self::UseBed),
            0x12 => Some(Self::UseAnimation),
            0x13 => Some(Self::EntityAction),
            0x14 => Some(Self::NamedEntitySpawn),
            0x15 => Some(Self::PickupSpawn),
            0x16 => Some(Self::CollectItem),
            0x17 => Some(Self::AddObject),
            0x18 => Some(Self::MobSpawn),
            0x19 => Some(Self::AddPainting),
            0x1b => Some(Self::StanceUpdate),
            0x1c => Some(Self::EntityVelocity),
            0x1d => Some(Self::DestroyEntity),
            0x1e => Some(Self::EntityUnchanged),
            0x1f => Some(Self::EntityRelativeMove),
            0x20 => Some(Self::EntityLook),
            0x21 => Some(Self::EntityLookAndRelativeMove),
            0x22 => Some(Self::EntityTeleport),
            0x26 => Some(Self::EntityStatus),
            0x27 => Some(Self::AttachEntity),
            0x28 => Some(Self::EntityMetadata),
            0x32 => Some(Self::PreChunk),
            0x33 => Some(Self::MapChunk),
            0x34 => Some(Self::MultiBlockChange),
            0x35 => Some(Self::BlockChange),
            0x36 => Some(Self::BlockAction),
            0x3c => Some(Self::Explosion),
            0x3d => Some(Self::SoundEffect),
            0x46 => Some(Self::NewState),
            0x47 => Some(Self::Thunderbolt),
            0x64 => Some(Self::OpenWindow),
            0x65 => Some(Self::CloseWindow),
            0x66 => Some(Self::WindowClick),
            0x67 => Some(Self::SetSlot),
            0x68 => Some(Self::WindowItems),
            0x69 => Some(Self::UpdateProgressBar),
            0x6a => Some(Self::Transaction),
            0x82 => Some(Self::UpdateSign),
            0x83 => Some(Self::ItemData),
            0x84 => Some(Self::IncrementStatistic),
            0xFF => Some(Self::DisconnectOrKick),
            _ => None,
        }
    }
}
