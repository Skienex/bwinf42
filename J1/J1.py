def get_file(path):
    with open(path, 'r') as file:
        lines = file.read().splitlines()
        without_space = [line for line in lines if ' ' not in line]
        wundertueten = int(without_space[0])
        max_games = int(without_space[1])
        games = without_space[2:]
        games = [int(game) for game in games if game != '']
        return wundertueten, max_games, games


def verteile_games(wundertueten_num, max_games, games):
    wundertueten = []
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


if __name__ == '__main__':
    for i in range(6):
        wundertueten, max_games, games = get_file(f"wundertuete{i}.txt")
        result = verteile_games(wundertueten, max_games, games)
        print_wundertueten(result)