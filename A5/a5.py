with open("tour.txt", "r") as file:
    lines = file.readlines()

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

for loc in essential_locations:
    print(f"{loc['name']} ({loc['year']})")