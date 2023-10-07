import socket
from http.server import HTTPServer, BaseHTTPRequestHandler
from urllib.parse import urlparse, parse_qs
from PIL import Image
import J1.J1 as wundertuete_cli
import J2.J2 as egano_cli
import A4.A4 as nandu_cli
import A5.A5 as stadtfuehrung_cli

import json

import rust_server_glue as rsg


class HTTPServerV6(HTTPServer):
    address_family = socket.AF_INET6


class NeuralHTTP(BaseHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200)
        self.send_header("Content-type", "application/json; charset=utf-8")
        self.end_headers()

        parsed_url = urlparse(self.path)
        query_components = parse_qs(parsed_url.query)

        program = query_components.get('program', [''])[0]
        size = query_components.get('size', [''])[0]
        file = query_components.get('file', [''])[0]

        match program:
            case "wundertuete":
                self.handle_wundertuete(file)
            case "egano":
                self.handle_egano(file)
            case "nandu":
                self.handle_nandu(file)
            case "stadtfuehrung":
                self.handle_stadtfuehrung(file)
            case "zauberschule":
                self.handle_zauberschule(file)
    
    
    def handle_wundertuete(self, file: str):
        match file:
            case '0':
                wundertueten, max_games, games = wundertuete_cli.get_file("J1/wundertuete0.txt")
                result = wundertuete_cli.verteile_games(wundertueten, max_games, games)
                self.wfile.write(bytes(str(result), "utf-8"))
            case '1':
                wundertueten, max_games, games = wundertuete_cli.get_file("J1/wundertuete1.txt")
                result = wundertuete_cli.verteile_games(wundertueten, max_games, games)
                self.wfile.write(bytes(str(result), "utf-8"))
            case '2':
                wundertueten, max_games, games = wundertuete_cli.get_file("J1/wundertuete2.txt")
                result = wundertuete_cli.verteile_games(wundertueten, max_games, games)
                self.wfile.write(bytes(str(result), "utf-8"))
            case '3':
                wundertueten, max_games, games = wundertuete_cli.get_file("J1/wundertuete3.txt")
                result = wundertuete_cli.verteile_games(wundertueten, max_games, games)
                self.wfile.write(bytes(str(result), "utf-8"))
            case '4':
                wundertueten, max_games, games = wundertuete_cli.get_file("J1/wundertuete4.txt")
                result = wundertuete_cli.verteile_games(wundertueten, max_games, games)
                self.wfile.write(bytes(str(result), "utf-8"))
            case '5':
                wundertueten, max_games, games = wundertuete_cli.get_file("J1/wundertuete5.txt")
                result = wundertuete_cli.verteile_games(wundertueten, max_games, games)
                self.wfile.write(bytes(str(result), "utf-8"))
            case _:
                for i in range(6):
                    wundertueten, max_games, games = wundertuete_cli.get_file(f"J1/wundertuete{i}.txt")
                    result = wundertuete_cli.verteile_games(wundertueten, max_games, games)
                    self.wfile.write(bytes(str(result), "utf-8"))
        
        
    def handle_egano(self, file: str):
        match file:
            case '0':
                image = Image.open("J2/bild01.png").convert('RGB')
                width, height = image.size
                print(egano_cli.extract_keyword(image, width, height))
                self.wfile.write(bytes(egano_cli.extract_keyword(image, width, height), "utf-8"))
            case '1':
                image = Image.open("J2/bild02.png").convert('RGB')
                width, height = image.size
                print(egano_cli.extract_keyword(image, width, height))
                self.wfile.write(bytes(egano_cli.extract_keyword(image, width, height), "utf-8"))
            case '2':
                image = Image.open("J2/bild03.png").convert('RGB')
                width, height = image.size
                print(egano_cli.extract_keyword(image, width, height))
                self.wfile.write(bytes(egano_cli.extract_keyword(image, width, height), "utf-8"))
            case '3':
                image = Image.open("J2/bild04.png").convert('RGB')
                width, height = image.size
                print(egano_cli.extract_keyword(image, width, height))
                self.wfile.write(bytes(egano_cli.extract_keyword(image, width, height), "utf-8"))
            case '4':
                image = Image.open("J2/bild05.png").convert('RGB')
                width, height = image.size
                print(egano_cli.extract_keyword(image, width, height))
                self.wfile.write(bytes(egano_cli.extract_keyword(image, width, height), "utf-8"))
            case '5':
                image = Image.open("J2/bild06.png").convert('RGB')
                width, height = image.size
                print(egano_cli.extract_keyword(image, width, height))
                self.wfile.write(bytes(egano_cli.extract_keyword(image, width, height), "utf-8"))
            case '6':
                image = Image.open("J2/bild07.png").convert('RGB')
                width, height = image.size
                print(egano_cli.extract_keyword(image, width, height))
                self.wfile.write(bytes(egano_cli.extract_keyword(image, width, height), "utf-8"))
            case _:
                for i in range(7):
                    image = Image.open(f"J2/bild0{i + 1}.png").convert('RGB')
                    width, height = image.size
                    print(egano_cli.extract_keyword(image, width, height))
                    self.wfile.write(bytes(egano_cli.extract_keyword(image, width, height), "utf-8"))


    def handle_nandu(self, file: str):
        match file:
            case '0':
                result = nandu_cli.calculate("A4/nandu1.txt")
                self.wfile.write(bytes(str(result), "utf-8"))
            case '1':
                result = nandu_cli.calculate("A4/nandu2.txt")
                self.wfile.write(bytes(str(result), "utf-8"))
            case '2':
                result = nandu_cli.calculate("A4/nandu3.txt")
                self.wfile.write(bytes(str(result), "utf-8"))
            case '3':
                result = nandu_cli.calculate("A4/nandu4.txt")
                self.wfile.write(bytes(str(result), "utf-8"))
            case '4':
                result = nandu_cli.calculate("A4/nandu5.txt")
                self.wfile.write(bytes(str(result), "utf-8"))


    def handle_stadtfuehrung(self, file: str):
        match file:
            case '0':
                lines = stadtfuehrung_cli.open_tour("A5/tour1.txt")
                essential_locations = stadtfuehrung_cli.calculate(lines)
                self.wfile.write(bytes(essential_locations, "utf-8"))
            case '1':
                lines = stadtfuehrung_cli.open_tour("A5/tour2.txt")
                essential_locations = stadtfuehrung_cli.calculate(lines)
                self.wfile.write(bytes(essential_locations, "utf-8"))
            case '2':
                lines = stadtfuehrung_cli.open_tour("A5/tour3.txt")
                essential_locations = stadtfuehrung_cli.calculate(lines)
                self.wfile.write(bytes(essential_locations, "utf-8"))
            case '3':
                lines = stadtfuehrung_cli.open_tour("A5/tour4.txt")
                essential_locations = stadtfuehrung_cli.calculate(lines)
                self.wfile.write(bytes(essential_locations, "utf-8"))
            case '4':
                lines = stadtfuehrung_cli.open_tour("A5/tour5.txt")
                essential_locations = stadtfuehrung_cli.calculate(lines)
                self.wfile.write(bytes(essential_locations, "utf-8"))


    def handle_zauberschule(self, file: str):
        result = None
        match file:
            case '0':
                result = rsg.solve_maze(read_file("mazes/zauberschule0.txt"))
            case '1':
                result = rsg.solve_maze(read_file("mazes/zauberschule1.txt"))
            case '2':
                result = rsg.solve_maze(read_file("mazes/zauberschule2.txt"))
            case '3':
                result = rsg.solve_maze(read_file("mazes/zauberschule3.txt"))
            case '4':
                result = rsg.solve_maze(read_file("mazes/zauberschule4.txt"))
            case '5':
                result = rsg.solve_maze(read_file("mazes/zauberschule5.txt"))
        if result is None:
            print("None-result!!!")
            self.wfile.write(bytes("[]", "utf-8"))
            return
        # self.wfile.write(bytes(f"[{json.dumps(result.maze.cells)}, {json.dumps(result.path)}]", "utf-8"))
        self.wfile.write(bytes(json.dumps({"cells": result.maze.cells, "path": result.path}), "utf-8"))


def read_file(file: str) -> str:
    with open(file, "r") as f:
        return f.read()


HOST = "::"
PORT = 8888
server = HTTPServerV6((HOST, PORT), NeuralHTTP)

print("Server started on port %s" % PORT)
server.serve_forever()
server.server_close()
print("Server stopped.")
