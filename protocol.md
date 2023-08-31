# Protocol

| Name | Offset | Size |
| ---- | ------ | ---- |
| Magic number | 0x00 | 4 |
| Packet type | 0x04 | 4 |
| Packet size | 0x08 | 4 |
| Payload | 0x0B | Max 1012 |

## Packet header:
The packet header consists of 3 fields:
- Magic number: Used to identify packets, it's value is `0xCAFEBABE`
- Packet type: Type of the packet
    - Connect: Someone joined the chat
    - Message: Someone sent a chat message
    - Leave: Someone left the chat
- Packet size: Number of all bytes in the packet including both the header and payload

## Packet payload
The packet payload's content depends on the `Packet type` field

### Connect

Clients use the `Connect` packet to tell the server it wishes to connect to it.
Servers use the `Connect` packet to tell the all connected clients that a new client joined.

| Name | Payload offset | Size |
| ---- | -------------- | ---- |
| Username length | 0x00 | 1 |
| Username | 0x01 | `Username length` |

### Message

| Name | Payload offset | Size |
| ---- | -------------- | ---- |
| Message length | 0
| Username length | 0x00 | 1 |
| Username | 0x01 | `Username length` |