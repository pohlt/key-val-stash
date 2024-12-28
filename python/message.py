import struct
from enum import Enum

KEY_LENGTH = 32
MAX_VALUE_LENGTH = 4096
MIN_MESSAGE_LENGTH = 3 + KEY_LENGTH
MAX_MESSAGE_LENGTH = MIN_MESSAGE_LENGTH + MAX_VALUE_LENGTH


class Command(Enum):
    PUT = ord("P")
    GET = ord("G")
    OK = ord("O")
    NOK = ord("N")


class MessageError(Exception):
    pass


def pack(command: Command, key: bytes, msg: bytes) -> bytes:
    if len(key) != KEY_LENGTH:
        raise MessageError("key length mismatch")
    if len(msg) > MAX_VALUE_LENGTH:
        raise MessageError("message too long")
    if command not in {c for c in Command}:
        raise MessageError("unknown command")

    length = 3 + KEY_LENGTH + len(msg)
    return struct.pack("!HB32s", length, command.value, key) + msg


def unpack(data: bytes):
    if len(data) > MAX_MESSAGE_LENGTH:
        raise MessageError("message too long")
    if len(data) < 3 + KEY_LENGTH:
        raise MessageError("message too short")

    length, command, key = struct.unpack("HB32s", data[: 3 + KEY_LENGTH])
    if length != len(data):
        raise MessageError(f"length mismatch: {length} != {len(data)}")
    if command not in {c.value for c in Command}:
        raise MessageError("unknown command")

    msg = data[35:]
    return Command(command), key, msg
