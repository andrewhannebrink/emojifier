import os
import random

def concatVideos(concatFile, outputVideo):
    os.system(f'ffmpeg -f concat -safe 0 -i {concatFile} -c copy io/input/vid/{outputVideo}')

def getImgNamesAndWipeOutput(inputDir, outputDir):
    allImageNames = os.listdir(f'io/input/vid/{inputDir}')
    os.system(f'rm -rf io/input/vid/{outputDir} && mkdir io/input/vid/{outputDir}')
    return allImageNames

def cutVideos(inputDir, outputDir, seconds): 
    allImageNames = getImgNamesAndWipeOutput(inputDir, outputDir)
    for i, name in enumerate(allImageNames):
        os.system(f'ffmpeg -i io/input/vid/{inputDir}/{name} -ss 00:00:00 -t 00:00:{seconds} -async 1 io/input/vid/{outputDir}/{name}')

def writeConcatFile(tenDir, twoDir, concatFile):
    f = open(f'{concatFile}', "w")
    for i in range(18):
        f.write(f'file io/input/vid/{tenDir}/{i}.mp4\n')
        fiveRandomVids = list(map(lambda x: f'io/input/vid/{twoDir}/{x}.mp4', random.sample(range(18), 5)))
        print(fiveRandomVids)
        for filePath in fiveRandomVids:
            f.write(f'file {filePath}\n')
    f.close()
    

def renameVideos(inputDir, outputDir):
    allImageNames = getImgNamesAndWipeOutput(inputDir, outputDir)
    for i, name in enumerate(allImageNames):
        print(name, i)
        adjustedName = name.replace(' ', '\\ ')
        print(f'cp io/input/vid/{inputDir}/{adjustedName} io/input/vid/{outputDir}/')
        os.system(f'cp io/input/vid/{inputDir}/{adjustedName} io/input/vid/{outputDir}/')
        os.system(f'mv io/input/vid/{outputDir}/{adjustedName} io/input/vid/{outputDir}/{i}.mp4')

def writeFramesToDir(inputVideoNumber, inputDir):
    os.system(f'mkdir io/input/vid/cut_frames/{inputDir}')
    os.system(f'ffmpeg -ss 0 -i "io/input/vid/{inputDir}/{inputVideoNumber}.mp4" -r 30.0 "io/input/vid/cut_frames/{inputDir}/{inputVideoNumber}/%5d.jpeg"')
    
def writeCutFramesToB():
    os.system('rm -rf io/input/b && mkdir io/input/b')
    os.system('cp -r io/input/vid/cut_frames/winners_cut_exact_twos/0/ io/input/b')

def makeCutFrames():
    os.system('rm -rf io/input/vid/cut_frames')
    os.system('mkdir io/input/vid/cut_frames')
    for inputDir in ["winners_cut_exact_tens", "winners_cut_exact_twos"]:
        os.system(f'mkdir io/input/vid/cut_frames/{inputDir}')
        for i in range(2):
            os.system(f'mkdir io/input/vid/cut_frames/{inputDir}/{i}')
            writeFramesToDir(i, inputDir)


    
    

def main():
    #cutVideos("winners_cut",  "winners_cut_exact_tens", "10")
    #cutVideos("winners_cut",  "winners_cut_exact_twos", "02")

    #makeCutFrames()
    writeCutFramesToB()

    #writeConcatFile("winners_cut_exact", "winners_cut_exact_twos", "concat.txt")
    #concatVideos("concat.txt", "winners-long.mp4")

main()
