import socket
import threading

def receive_messages(sock: socket.socket):
    while True:
        try:
            if message := sock.recv(1024).decode('utf-8'):
                print(f"\n{message}")
            else:
                break
        except:
            break

def connect(host: str, port: int):
    client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    client.connect((host, port))
    return client

def get_username():
    return input("Enter your name: ")

def main():        
    client = connect("server", 12345)
    name = get_username()

    threading.Thread(
        target=receive_messages, 
        args=(client,), 
        daemon=True
    ).start()

    while True:
        if (message := input()).lower() == "exit":
            break

        line = f"{name}: {message}"
        client.send(line.encode('utf-8'))

    client.close()
    print("Disconnected from server.")

if __name__ == "__main__":
    main()
