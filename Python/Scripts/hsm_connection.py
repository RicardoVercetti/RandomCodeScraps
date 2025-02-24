import socket
import time
from datetime import datetime

def current_time():
    # Get the current date and time
    now = datetime.now()

    # Format the time as a string
    formatted_time = now.strftime("%Y-%m-%d %H:%M:%S")
    return formatted_time

class FutureX:
    def __init__(self, host='192.168.100.63', port=9000):
        self.host = host
        self.port = port
        self.sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.log_all = False
        try:
            self.sock.connect((self.host, self.port))
            print(f"Connected to server at {self.host}:{self.port}")
        except ConnectionRefusedError:
            print("Connection failed. Ensure the server is running and reachable.")

    def send(self, message: str):
        """Sends a message to the connected server."""
        encapsulated = self.encapsulate(message)
        try:
            self.sock.sendall(encapsulated.encode('utf-8'))
            print("sent : " + encapsulated) if self.log_all else None
        except Exception as e:
            print(f"Error sending message: {e}")

    def receive(self, buffer_size=1024) -> str:
        """Receives data from the connected server."""
        try:
            data = self.sock.recv(buffer_size)
            if data:
                received = data.decode('utf-8')
                print("Received : ", received) if self.log_all else None
                return received
            else:
                return ""
        except Exception as e:
            print(f"Error receiving message: {e}")
            return ""

    def close(self):
        """Closes the socket connection."""
        self.sock.close()
        print("Connection closed.")

    def encapsulate(self, str):
        return f"[{str}]"

if __name__ == "__main__":
    print(f"started at : {current_time()}")
    # Instantiate the client
    client = FutureX()

    for i in range(3):
        # Send a message
        client.send("[Hello, this is my custom string!]")
        
        # Optionally, receive a response
        response = client.receive()
        if response:
            print("Received:", response)
    
    # Close the connection
    client.close()
