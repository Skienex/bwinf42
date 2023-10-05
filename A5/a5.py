def open_tour(num):
    with open(f"tour{num}.txt", "r") as file:
        lines = file.readlines()
    return lines

def calculate(lines):
    locations = []
    for line in lines[1:]:
        location, year, essential, distance = line.strip().split(',')
        locations.append({
            "name": location,
            "year": int(year),
            "essential": essential == 'X',
            "distance": int(distance)
        })

    essential_locations = [loc for loc in locations if loc["essential"]]
    essential_locations.sort(key=lambda loc: loc["year"])

    return essential_locations

for i in range(1, 6):
    lines = open_tour(i)
    essential_locations = calculate(lines)
    print(f"\n\ntour{i}.txt:")
    for loc in essential_locations:
        print(f"{loc['name']} ({loc['year']})")