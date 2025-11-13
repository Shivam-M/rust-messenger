from socket import *
import select
import json

HOST = "0.0.0.0"
PORT = 4999

server_socket = socket(AF_INET, SOCK_STREAM)
server_socket.setsockopt(SOL_SOCKET, SO_REUSEADDR, 1)
server_socket.bind((HOST, PORT))
server_socket.listen()
print(f"Listening on {HOST}:{PORT}...")

sockets_list = [server_socket]
clients = {}

def broadcast(message):
    for client_socket in clients:
        try:
            client_socket.send(message)
        except:
            client_socket.close()
            sockets_list.remove(client_socket)
            del clients[client_socket]

def get_username(sender_socket):
    return clients.get(sender_socket, "unknown")

def set_username(username, sender_socket):
    if len(username) > 16 or " " in username:
        return False
    clients[sender_socket] = username

while True:
    try:
        read_sockets, _, _ = select.select(sockets_list, [], [])

        for sockfd in list(read_sockets):
            if sockfd == server_socket:
                client_socket, client_address = server_socket.accept()
                sockets_list.append(client_socket)
                clients[client_socket] = "unknown"
                print(f"Client {client_address} connected")
            else:
                try:
                    message = sockfd.recv(1024)
                    if not message:
                        print(f"Client {sockfd} disconnected")
                        sockets_list.remove(sockfd)
                        del clients[sockfd]
                        continue
                    print(f"Received message from {clients[sockfd]}: {message.decode()}")
                    try:
                        json_data = json.loads(message.decode())
                        if json_data["data-type"] == "message":
                            if json_data.get("content"):
                                json_data["username"] = get_username(sockfd)
                                broadcast(json.dumps(json_data).encode())
                        elif json_data["data-type"] == "username":
                            set_username(json_data["username"], sockfd)
                            sockfd.send(message)
                    except Exception as e:
                        print(e)
                except Exception as e:
                    print(e)
                    sockets_list.remove(sockfd)
                    del clients[sockfd]
    except Exception as e:
        print(e)
