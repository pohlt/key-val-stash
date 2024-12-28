#!/usr/bin/env python3
import socket
from secrets import token_bytes

import message

server_address = "127.0.0.1"
server_port = 28536

client_socket = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
client_socket.connect((server_address, server_port))

msg = message.pack(message.Command.GET, b"kXy_" * 8, b"")
client_socket.send(msg)
response = client_socket.recv(message.MAX_MESSAGE_LENGTH)
print(message.unpack(response))

msg = message.pack(message.Command.GET, b"key_" * 8, b"")
client_socket.send(msg)
response = client_socket.recv(message.MAX_MESSAGE_LENGTH)
print(message.unpack(response))

msg = message.pack(message.Command.PUT, b"key_" * 8, b"msg" + message.unpack(response)[2])
client_socket.send(msg)
response = client_socket.recv(message.MAX_MESSAGE_LENGTH)
print(message.unpack(response))

for i in range(1, 80):
    msg = message.pack(message.Command.PUT, token_bytes(32), b"msg" * i)
    client_socket.send(msg)
    response = client_socket.recv(message.MAX_MESSAGE_LENGTH)
    print(i, message.unpack(response))


client_socket.close()
