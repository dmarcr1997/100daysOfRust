import socket

HOST = "127.0.0.1"
PORT = 3000

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
    sock.connect((HOST, PORT))

    message = "Hello GhostShell!\n"
    sock.sendall(message.encode())
    response = sock.recv(1024)
    print("Received:", response.decode())