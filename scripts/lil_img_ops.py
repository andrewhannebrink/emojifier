import os
import codecs
import random

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
    allImageNames = os.listdir("io/lil_imgs/emoji_big")
    os.system('rm -rf io/lil_imgs/emoji_trim && mkdir io/lil_imgs/emoji_trim')
    for name in allImageNames:
        os.system(f'convert io/lil_imgs/emoji/{name} -trim io/lil_imgs/emoji_trim/{name}')


def centerImagesOnWhiteTile(inputDir, outputDir): 
    allImageNames = os.listdir(f'io/lil_imgs/{inputDir}')
    os.system(f'rm -rf io/lil_imgs/{outputDir} && mkdir io/lil_imgs/{outputDir}')
    for name in allImageNames:
        os.system(f'convert -size 200x200 xc:white io/lil_imgs/{inputDir}/{name} -gravity center -composite io/lil_imgs/{outputDir}/{name}')

def resize(inputDir, outputDir, newSize): 
    allImageNames = os.listdir(f'io/lil_imgs/{inputDir}')
    os.system(f'rm -rf io/lil_imgs/{outputDir} && mkdir io/lil_imgs/{outputDir}')
    for name in allImageNames:
        os.system(f'convert -resize {newSize}x{newSize} xc:white io/lil_imgs/{inputDir}/{name} -gravity center -composite io/lil_imgs/{outputDir}/{name}')

def convertSvgToJpeg(inputDir, outputDir):
    allImageNames = os.listdir(f'io/lil_imgs/{inputDir}')
    os.system(f'rm -rf io/lil_imgs/{outputDir} && mkdir io/lil_imgs/{outputDir}')
    for name in allImageNames:
        os.system(f'convert io/lil_imgs/{inputDir}/{name} io/lil_imgs/{outputDir}/{name[0:-4]}.jpeg')

def moreColors():
    allImageNames = os.listdir("io/lil_imgs/sdg_raw")
    r = lambda: random.randint(0,255)
    os.system('rm -rf io/lil_imgs/sdg_more_colors && mkdir io/lil_imgs/sdg_more_colors')
    for name in allImageNames:
        os.system(f'cp io/lil_imgs/sdg_raw/{name} io/lil_imgs/sdg_more_colors/{name}')
        with codecs.open(f'io/lil_imgs/sdg_raw/{name}', encoding='utf-8', errors='ignore') as f:
            content = f.read()
            fillIndex = content.find('fill')
            fillLineEndIndex = content[fillIndex:-1].find('\n')
            fillLine = content[fillIndex:(fillIndex + fillLineEndIndex)]
            for i in range(50):
                randomHexColor = '#%02X%02X%02X' % (r(),r(),r())
                indicesToReplace = (fillLine.find("#"), fillLine.find(';'))
                newFillLine = fillLine[0:fillLine.find('#')] + randomHexColor + fillLine[fillLineEndIndex:-1] + ';'
                #print(fillLine)
                #print(newFillLine)
                newContent = content[0:fillIndex] + newFillLine + content[(fillIndex+fillLineEndIndex):-1]
                #print(newContent)
                newFileName = name[:-4] + '_' + str(i) + '.svg'
                with open(f'io/lil_imgs/sdg_more_colors/{newFileName}', 'w') as f:
                    f.write(newContent)
                #print(newFileName)


def main():
    whiteBackgrounds()
    bigImages()
    trimWhiteSpace()
    centerImagesOnWhiteTile("emoji_trim", "emoji_big_buffered")
    #convertSvgToJpeg()
    #resize("sdg_jpeg", "sdg_jpeg_small", 120)
    #centerImagesOnWhiteTile("emoji_trim", "emoji_buffered")
    #moreColors()
    #convertSvgToJpeg("sdg_more_colors", "sdg_more_colors_jpeg")
    #resize("sdg_more_colors_jpeg", "sdg_more_colors_jpeg_small", 120)

main()
