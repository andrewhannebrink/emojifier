import os

def whiteBackgrounds():
    allImageNames = os.listdir("io/lil_imgs/emoji_raw")
    os.system('rm -rf io/lil_imgs/emoji && mkdir io/lil_imgs/emoji')
    for imageName in allImageNames:
        #print(imageName)
        os.system(f'convert io/lil_imgs/emoji_raw/{imageName} -resize 60x60^ -background white -alpha remove -alpha off io/lil_imgs/emoji/{imageName}')

def main():
    whiteBackgrounds()

main()
