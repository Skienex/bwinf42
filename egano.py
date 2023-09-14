from PIL import Image

def rgb_of_pixel(x, y):
    r, g, b = image.getpixel((x, y))
    w, h = image.size
    pixel = {
        'x': x,
        'y': y,
        'r': r,
        'g': g,
        'b': b,
        'width': w,
        'height': h
    }
    return pixel

def next_pixel(pixel):
    next_x = pixel.get('x') + pixel.get('g')
    next_y = pixel.get('y') + pixel.get('b')

    next_x = next_x % pixel.get('width')
    next_y = next_y % pixel.get('height')

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

for i in range(6):
    img_path = f"assets/bild0{i+1}.png"
    image = Image.open(img_path).convert('RGB')
    print(f"Wort für Bild {i + 1} gefunden! Es ist: \n'{extract_keyword()}'")
    print("------------------------------------------")
    print("------------------------------------------")