# import tcod
import random
import socket

HOST = "127.0.0.1"
PORT = 9878

id = random.randbytes(8)

def main() -> None:
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((HOST, PORT))
        a = "CONNECT".encode("utf-8") + id
        s.sendall(a)
        data2 = s.recv(16)
    
    print(f"{data2!r}")

        
    # tilesheet = tcod.tileset.load_tilesheet(
    #     "../assets/game/Yayo.png", 16, 16, tcod.tileset.CHARMAP_CP437
    # )
    # console = tcod.console.Console(80, 50, "C")

    # with tcod.context.new(
    #     tileset=tilesheet,
    #     console=console,
    #     title="Laurel",
    # ) as context:
    #     while True:
    #         console.clear()
    #         console.print(1,1, "Hello")
    #         context.present(console=console)
            


if __name__ == "__main__":
    main()
