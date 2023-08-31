# Protocol

| Name | Offset | Size |
| ---- | ------ | ---- |
| Magic number | 0x00 | 4 |
| Packet type | 0x04 | 4 |
| Payload | 0x08 | <1016 |

## Packet header:
The packet header consists of 3 fields:
- Magic number: Used to identify packets, it's value is `0xCAFEBABE`
- Packet type: Type of the packet
    0. Connect: Someone joined the chat
    1. ConnectResponse: Confirmation to the client about a `Connect` request
    1. SendMessage: Client sent a chat message
    2. Message: Server notifies clients about a new message
- Packet size: Number of all bytes in the packet including both the header and payload

## Packet payload:
The packet payload's content depends on the `Packet type` field

### Connect

Unconnected clients use the `Connect` packet to tell the server it wishes to connect to it.
Servers use the `Connect` packet to notify all connected clients that a new client has joined.

Username must be longer than 0 bytes and does not need to be null terminated.

| Name | Payload offset | Size |
| ---- | -------------- | ---- |
| Username length | 0x00 | 1 |
| Username | 0x01 | `Username length` |

### ConnectResponse

Servers use the `ConnectResponse` packet to tell the client whether it successfully connected or not.

1 means successful, 0 means failed

| Name | Payload offset | Size |
| ---- | -------------- | ---- |
| Success | 0x00 | 1 |

### SendMessage

Connected clients use the `SendMessage` packet to send a message.

Message must be longer than 0 bytes, shorter than 1001 bytes and does not need to be null terminated.

| Name | Payload offset | Size |
| ---- | -------------- | ---- |
| Message length | 0x00 | 2 |
| Message | 0x1 | `Message length` |

### Message

Servers use the `Message` packet to notify clients about a new message that was sent by a client.

| Name | Payload offset | Size |
| ---- | -------------- | ---- |
| Username length | 0x00 | 1 |
| Message length | 0x01 | 2 |
| Username | 0x03 | `Username length` |
| Message | 0x03 + `Username length` | `Message length` |
