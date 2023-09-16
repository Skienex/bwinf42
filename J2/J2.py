from PIL import Image

def rgb_of_pixel(x, y):
    r, g, b = image.getpixel((x, y))
    w, h = image.size
    pixel = {
        'x': x,
        'y': y,
        'r': r,
        'g': g,
        'b': b
    }
    return pixel

def next_pixel(current_pixel):
    next_x = current_pixel.get('x') + current_pixel.get('g')
    next_y = current_pixel.get('y') + current_pixel.get('b')

    next_x = next_x % width
    next_y = next_y % height

    return next_x, next_y

def extract_keyword():
    word = ""
    pixel = rgb_of_pixel(0, 0)
    word += chr(pixel.get('r'))

    while True:
        next_x, next_y = next_pixel(pixel)
        if next_x == pixel.get('x') and next_y == pixel.get('y'):
            break

        pixel = rgb_of_pixel(next_x, next_y)
        word += chr(pixel.get('r'))

    return word

for i in range(7):
    img_path = f"assets/bild0{i+1}.png"
    image = Image.open(img_path).convert('RGB')
    width, height = image.size
    
    print(f"Wort für Bild {i + 1} gefunden! Es ist: \n'{extract_keyword()}'")
    print("------------------------------------------")
    print("------------------------------------------")