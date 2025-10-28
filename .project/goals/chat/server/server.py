import socket
import threading

clients: list[socket.socket] = []

def handle_client(conn: socket.socket, addr: tuple[str, int]):
    print(f"[NEW CONNECTION] {addr} connected.")
    while True:
        try:
            if not (message := conn.recv(1024)):
                break

            broadcast(message, conn)
        except:
            break
    conn.close()
    clients.remove(conn)
    print(f"[DISCONNECT] {addr} disconnected.")

def broadcast(message: bytes, sender: socket.socket):
    for client in clients:
        if client != sender:
            try:
                client.send(message)
            except:
                pass


def start_server(host: str, port: int):
    server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server.bind((host, port))
    server.listen()
    print(f"[LISTENING] Server is listening on {host}:{port}")
    return server

def connect_client(server: socket.socket):
    conn, addr = server.accept()
    clients.append(conn)
    return conn, addr

def main():
    server = start_server("0.0.0.0", 12345)

    while True:
        conn, addr = connect_client(server)
        thread = threading.Thread(target=handle_client, args=(conn, addr))
        thread.start()


if __name__ == "__main__":
    main()
