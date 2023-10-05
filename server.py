from http.server import HTTPServer, BaseHTTPRequestHandler
from urllib.parse import urlparse, parse_qs
from PIL import Image
import J1.J1 as wundertuete_cli
import J2.J2 as egano_cli
import A4.A4 as nandu_cli
import A5.A5 as stadtfuehrung_cli

HOST = "192.168.178.36"
PORT = 8888


class NeuralHTTP(BaseHTTPRequestHandler):

    def do_GET(self):
        self.send_response(200)
        self.send_header("Content-type", "text/html; charset=utf-8")
        self.end_headers()

        parsed_url = urlparse(self.path)
        query_components = parse_qs(parsed_url.query)

        programm = query_components.get('programm', [''])[0]
        size = query_components.get('size', [''])[0]
        file = query_components.get('file', [''])[0]

        if programm == 'wundertuete':
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

        if programm == 'egano':
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

        if programm == 'nandu':
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

        if programm == 'stadtfuehrung':
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

server = HTTPServer((HOST, PORT), NeuralHTTP)
print("Server started on port %s" % PORT)
server.serve_forever()
server.server_close()
print("Server stopped.")
