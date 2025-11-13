from socket import *
import select

HOST = "0.0.0.0"
PORT = 4999

server_socket = socket(AF_INET, SOCK_STREAM)
server_socket.setsockopt(SOL_SOCKET, SO_REUSEADDR, 1)
server_socket.bind((HOST, PORT))
server_socket.listen()
print(f"Listening on {HOST}:{PORT}...")

sockets_list = [server_socket]
clients = {}

def broadcast(message, sender_socket):
    for client_socket in clients:
        if client_socket != sender_socket:
            try:
                client_socket.send(message)
            except:
                client_socket.close()
                sockets_list.remove(client_socket)
                del clients[client_socket]

while True:
    try:
        read_sockets, _, _ = select.select(sockets_list, [], [])

        for sockfd in read_sockets:
            if sockfd == server_socket:
                client_socket, client_address = server_socket.accept()
                sockets_list.append(client_socket)
                clients[client_socket] = client_address
                print("Client {0}:{1} connected".format(*clients[client_socket]))
            else:
                try:
                    message = sockfd.recv(1024)
                    if not message:
                        print("Client {0}:{1} disconnected".format(*clients[sockfd]))
                        sockets_list.remove(sockfd)
                        del clients[sockfd]
                        continue
                    print(f"Received message from {clients[sockfd]}: {message.decode()}")
                    broadcast(message, sockfd)
                except:
                    sockets_list.remove(sockfd)
                    del clients[sockfd]
    except Exception as e:
        print(e)
        continue
