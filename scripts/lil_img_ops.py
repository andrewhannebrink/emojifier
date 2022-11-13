import os

def whiteBackgrounds():
    allImageNames = os.listdir("io/lil_imgs/emoji_raw")
    os.system('rm -rf io/lil_imgs/emoji && mkdir io/lil_imgs/emoji')
    for imageName in allImageNames:
        #print(imageName)
        os.system(f'convert io/lil_imgs/emoji_raw/{imageName} -resize 60x60^ -background white -alpha remove -alpha off io/lil_imgs/emoji/{imageName}')

def bigImages():
    allImageNames = os.listdir("io/lil_imgs/emoji_raw")
    os.system('rm -rf io/lil_imgs/emoji_big && mkdir io/lil_imgs/emoji_big')
    for imageName in allImageNames:
        #print(imageName)
        os.system(f'convert io/lil_imgs/emoji_raw/{imageName} -resize 200x200^ -background white -alpha remove -alpha off io/lil_imgs/emoji_big/{imageName}')

def trimWhiteSpace():
    allImageNames = os.listdir("io/lil_imgs/emoji")
    os.system('rm -rf io/lil_imgs/emoji_trim && mkdir io/lil_imgs/emoji_trim')
    for name in allImageNames:
        os.system(f'convert io/lil_imgs/emoji/{name} -trim io/lil_imgs/emoji_trim/{name}')


def centerImagesOnWhiteTile(): 
    allImageNames = os.listdir("io/lil_imgs/emoji_trim")
    os.system('rm -rf io/lil_imgs/emoji_buffered && mkdir io/lil_imgs/emoji_buffered')
    for name in allImageNames:
        os.system(f'convert -size 50x50 xc:white io/lil_imgs/emoji_trim/{name} -gravity center -composite io/lil_imgs/emoji_buffered/{name}')


def main():
    #whiteBackgrounds()
    #bigImages()
    #trimWhiteSpace()
    centerImagesOnWhiteTile()

main()
