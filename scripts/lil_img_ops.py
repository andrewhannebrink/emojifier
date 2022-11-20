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


def centerImagesOnWhiteTile(inputDir, outputDir): 
    allImageNames = os.listdir(f'io/lil_imgs/{inputDir}')
    os.system(f'rm -rf io/lil_imgs/{outputDir} && mkdir io/lil_imgs/{outputDir}')
    for name in allImageNames:
        os.system(f'convert -size 50x50 xc:white io/lil_imgs/{inputDir}/{name} -gravity center -composite io/lil_imgs/{outputDir}/{name}')

def resize(inputDir, outputDir, newSize): 
    allImageNames = os.listdir(f'io/lil_imgs/{inputDir}')
    os.system(f'rm -rf io/lil_imgs/{outputDir} && mkdir io/lil_imgs/{outputDir}')
    for name in allImageNames:
        os.system(f'convert -resize {newSize}x{newSize} xc:white io/lil_imgs/{inputDir}/{name} -gravity center -composite io/lil_imgs/{outputDir}/{name}')

def convertSvgToJpeg():
    allImageNames = os.listdir("io/lil_imgs/sdg_raw")
    os.system('rm -rf io/lil_imgs/sdg_jpeg && mkdir io/lil_imgs/sdg_jpeg')
    for name in allImageNames:
        os.system(f'convert io/lil_imgs/sdg_raw/{name} io/lil_imgs/sdg_jpeg/{name[0:-4]}.jpeg')

def main():
    #whiteBackgrounds()
    #bigImages()
    #trimWhiteSpace()
    #convertSvgToJpeg()
    resize("sdg_jpeg", "sdg_jpeg_small", 120)
    #centerImagesOnWhiteTile("emoji_trim", "emoji_buffered")

main()
