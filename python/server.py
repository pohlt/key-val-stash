import socket

import message

SERVER_ADDRESS = "0.0.0.0"
SERVER_PORT = 31337
db: dict[bytes, bytes] = {}


def serve(ip: str, port: int):
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.bind((ip, port))
    print("listening on " + ip + ":" + str(port))

    while True:
        value, client_address = sock.recvfrom(10240)
        try:
            command, key, value = message.unpack(value)
        except message.MessageError as e:
            print("Error: " + str(e))
        else:
            response = None
            match command:
                case message.Command.PUT:
                    if len(value) == 0:
                        del db[key]
                    else:
                        db[key] = value
                    response = message.pack(message.Command.OK, key, b"")
                case message.Command.GET:
                    value = db.get(key)
                    if value is None:
                        response = message.pack(message.Command.NOK, key, b"")
                    else:
                        response = message.pack(message.Command.OK, key, db.get(key, b""))
                case _:
                    print("Error: unknown command")

            if response:
                sent = sock.sendto(response, client_address)
                if sent != len(response):
                    print("Error: failed to send response")


if __name__ == "__main__":
    serve(SERVER_ADDRESS, SERVER_PORT)
