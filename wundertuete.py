import requests

def get_url(url):
    file = requests.get(url).text
    lines = file.splitlines()
    without_space = [line for line in lines if ' ' not in line]
    wundertueten = without_space[0]
    game_number = without_space[1]
    games = without_space[2:]
    return wundertueten, game_number, games

wundertueten, game_number, games = get_url("https://bwinf.de/fileadmin/bundeswettbewerb/42/wundertuete0.txt")
result = []

for i in range(game_number): # Jedes spiel durchgehen
