#!/usr/bin/env python3

import socket
import sys
import threading
import json
import getpass


def send(sock, msg):
	length = len(msg)
	header = length.to_bytes(4, byteorder="big")
	totalmsg = header + msg
	sock.sendall(totalmsg)

def receive(sock):
	header = recvall(sock, 4) #sock.recv(4)
	length = int.from_bytes(header, byteorder="big")
	return recvall(sock, length)

def recvall(sock, length):
	chunks = []
	bytes_recd = 0
	while bytes_recd < length:
		chunk = sock.recv(min(length - bytes_recd, 4096))
		if chunk == b'':
			break
			#raise RuntimeError("socket connection broken")
		chunks.append(chunk)
		bytes_recd = bytes_recd + len(chunk)
	return b''.join(chunks)
	


sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
sock.connect("\0rustifarm")#("localhost", 1234))

def listen():
	while True:
		d = receive(sock)
		if len(d) == 0:
			print("Connection closed by server", file=sys.stdout)
			return
		print(str(d, "utf-8"))

threading.Thread(target=listen, daemon=True).start()

if len(sys.argv) >= 2:
	name = sys.argv[1]
else:
	name = "~" + getpass.getuser()

send(sock, bytes(json.dumps(["name", name]), "utf-8"))

for line in sys.stdin:
	send(sock, bytes(json.dumps(["chat", line.strip()]), "utf-8"))
