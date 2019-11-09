import socket

HOST, PORT = "", 6969

listen_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
listen_socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
listen_socket.bind((HOST, PORT))
listen_socket.listen(1)
print("Serving HTTP on port {PORT} ...".format(PORT=PORT))
conns = 0

while conns <= 5:
    conns += 1
    print(conns)
    client_connection, client_address = listen_socket.accept()
    request_data = client_connection.recv(1024)
    print(request_data.decode('utf-8'))
    
    http_resp = b"""\
HTTP/1.1 200 OK

Hello, World!
"""
    client_connection.sendall(http_resp)
    client_connection.close()

