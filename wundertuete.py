import requests


def get_url(url):
    file = requests.get(url).text
    lines = file.splitlines()
    without_space = [line for line in lines if ' ' not in line]
    wundertueten = int(without_space[0])
    max_games = int(without_space[1])
    games = without_space[2:]
    games = [int(game) for game in games if game != '']
    return wundertueten, max_games, games


def verteile_games(url):
    wundertueten = []
    wundertueten_num, max_games, games = get_url(url)
    for i in range(int(wundertueten_num)):
        wundertueten.append([])
        for j in range(int(max_games)):
            wundertueten[i].append(0)

    index = 0
    for i in range(len(games)):
        while games[i] > 0:
            wundertueten[index][i] += 1
            games[i] -= 1
            index += 1
            if index >= wundertueten_num:
                index = 0

    return wundertueten


def print_wundertueten(result):
    for i in range(len(result)):
        print(f"Wundertüte {i + 1}:")
        for j in range(len(result[i])):
            print(f"Spiel {j}: {result[i][j]}")
        print("----------------------------")


url = 'https://bwinf.de/fileadmin/user_upload/wundertuete'

if __name__ == '__main__':
    for i in range(6):
        if i == 0:
            modified_url = "https://bwinf.de/fileadmin/bundeswettbewerb/42/wundertuete0.txt"
            result = verteile_games(modified_url)
        else:
            modified_url = f"{url}{i}.txt"
            result = verteile_games(modified_url)
        print(f"\n\nDATEI NR. {i + 1} ({modified_url}):")
        print_wundertueten(result)