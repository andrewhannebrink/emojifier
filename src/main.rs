mod transpose;
mod mosaic;
mod quadrants;
mod instruct;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let instructions = instruct::get_instructions();
    transpose::transpose_every_frame(&instructions);
    //quadrants::frames_into_quadrants();
    let elapsed_time = now.elapsed();
    println!("main() took {} seconds.", elapsed_time.as_secs());
}


//  def readFile(instructionsFile, movDir, outputName, colorMap = {}, mp4Bool = False, secondsRange = (0, 60)):
//      movieMaker.wipeDir('unique/')
//      frameMap = {}
//      specMap = {}	# { (PIVOTCOLORS, LEVELSPERPIVOTCOLOR, LILIMGDIR) : SPECTRUMDIR }
//  #	colorMap = {} 	# { (COLOR TUPLE, LILIMGDIR) : LILIMGPATH }	
//      definedSequences = {}
//      lilImgMap = {}
//      gifMap = {}
//      seqOrder = []
//      dbFrame = 1
//      f = open(instructionsFile)
//      frame = 1
//      lines = f.readlines()
//      curLine = 0
//      #EVENTUALLY CHANGE THIS TO ROLLING I VALUE
//      while curLine < len(lines):
//          lineWords = getWords(lines[curLine])
//          if lineWords is None:
//              curLine += 1
//              continue
//          if '#' in lineWords[0]:
//              curLine += 1
//              continue
//          if lineWords[0] == 'Sequence':
//              seqLines = []
//              seqName = lineWords[1]
//              seqType = lineWords[2][1:-1]
//              seq = Sequence(seqName)
//              #SPECTRUM MODE LINE PARSING
//              if seqType == 'spec':
//                  lilImgDir = ''
//                  baseDir = lineWords[3]
//                  loopFrame = int(lineWords[4])
//                  whiteSquare = False
//                  if baseDir[-1] == 'w':
//                      baseDir = baseDir[:-1]
//                      whiteSquare = True
//                  anims = []
//                  j = curLine + 1
//                  while getWords(lines[j])[0] != 'endSeq':
//                      seqLines.append(lines[j])
//                      if '#' in lines[j]:
//                          curLine += 1
//                          continue
//                      j += 1
//                  seqLines.append(lines[j])
//                  curSeqLine = 0
//                  while curSeqLine < len(seqLines):
//                      s = re.search('^\t{1}\S', seqLines[curSeqLine])
//                      try:
//                          match = s.group(0)
//                      except:
//                          s = re.search('^\t{2}\S', seqLines[curSeqLine])
//                          try:
//                              match = s.group(0)
//                          except:
//                              s = re.search('^\t{3}\S', seqLines[curSeqLine])
//                              try:
//                                  match = s.group(0)
//                                  seqLineWords = getWords(seqLines[curSeqLine])
//                                  if len(seqLineWords) is 4:
//                                      anim = [gifName, lilImgDir, seqLineWords[0], seqLineWords[1], seqLineWords[2], seqLineWords[3]]
//                                      anims.append(anim)
//                                  if len(seqLineWords) is 3:
//                                      anim = [gifName, lilImgDir, seqLineWords[0], seqLineWords[1], seqLineWords[2]]
//                                      anims.append(anim)
//                              except:
//                                  curSeqLine += 1
//                                  continue
//                              curSeqLine += 1
//                              continue 
//                          gifName = getWords(seqLines[curSeqLine])[0]
//                          curSeqLine += 1
//                          continue
//                      spectrumNums = getNums(seqLines[curSeqLine])
//                      anims.append(spectrumNums)
//                      
//                      curSeqLine += 1
//                  print 'anims:', anims
//                  (anims, specDirOrder) = modifySpecAnims(anims, colorMap, whiteSquare, baseDir)
//                  [dbFrame, loopFrame] = getMosFrames(anims, colorMap, gifMap, frameMap, movDir, outputName, seq, dbFrame, loopFrame, specDirOrder, secondsRange = secondsRange)


 //             #MOSAIC MODE LINE PARSING
 //             if seqType == 'mos':
 //                 #ANIMS DESCRIBE SEGMENTS OF THE SEQUENCE
 //                 loopFrame = int(lineWords[3])
 //                 anims = []
 //                 j = curLine + 1
 //                 while getWords(lines[j])[0] != 'endSeq':
 //                     seqLines.append(lines[j])
 //                     if '#' in lines[j]:
 //                         curLine += 1
 //                         continue
 //                     j += 1
 //                 seqLines.append(lines[j]) 
 //                 curSeqLine = 0
 //                 while curSeqLine < len(seqLines):
 //                     #MATCH SINGLE TABBED LINE
 //                     s = re.search('^\t{1}\S', seqLines[curSeqLine])
 //                     try:
 //                         match = s.group(0)
 //                     except:
 //                         #MATCH DOUBLE TABBED LINE
 //                         s = re.search('^\t{2}\S', seqLines[curSeqLine])
 //                         try:
 //                             match = s.group(0)
 //                             seqLineWords = getWords(seqLines[curSeqLine])
 //                             if len(seqLineWords) is 4:
 //                                 anim = (gifName, lilImgDir, seqLineWords[0], seqLineWords[1], seqLineWords[2], seqLineWords[3])
 //                                 anims.append(anim)
 //                             if len(seqLineWords) is 3:
 //                                 anim = (gifName, lilImgDir, seqLineWords[0], seqLineWords[1], seqLineWords[2])
 //                                 anims.append(anim)
 //                         except:
 //                             curSeqLine += 1
 //                             continue
 //                         curSeqLine += 1
 //                         continue
 //                     seqLineWords = getWords(seqLines[curSeqLine])
 //                     (gifName, lilImgDir) = (seqLineWords[0], seqLineWords[1])
 //                     print gifName, lilImgDir
 //                     curSeqLine += 1
 //                 [dbFrame, loopFrame] = getMosFrames(anims, colorMap, gifMap, frameMap, movDir, outputName, seq, dbFrame, loopFrame, secondsRange = secondsRange)
 //             definedSequences[seq.name] = seq
 //         if lineWords[0] == 'makeAnim':
 //             j = curLine + 1
 //             while getWords(lines[j])[0] != 'endAnim':	
 //                 if '#' in lines[j]:
 //                     curLine += 1
 //                     continue
 //                 seqOrder.append(getWords(lines[j])[0])
 //                 j += 1
 //             frame = 1
 //             pool = Pool(processes = 4)
 //             inputFrameNames = []
 //             print 'making mosaics for ' + str(len(frameMap)) + ' unique frames...'
 //             for key in frameMap:
 //                 (gifName, lilImgDir, loopFrame, curRes) = key
 //                 if gifName not in gifMap:
 //                     gifInfo = GifInfo(gifName, mp4Bool, secondsRange)
 //                     print 'new gifInfo class'
 //                     gifMap[gifName] = gifInfo
 //                 if lilImgDir not in lilImgMap:
 //                     littleImgs = remoji.getLittleImgs(lilImgDir)
 //                     lilImgMap[lilImgDir] = littleImgs
 //                 else:
 //                     littleImgs = lilImgMap[lilImgDir]
 //             for key in frameMap:
 //                 (gifName, lilImgDir, loopFrame, curRes) = key
 //                 inputFrameNames = os.listdir(gifMap[gifName].framesDir)
 //                 inputFrameNames.sort()
 //                 depthPix = curRes + 6
 //                 pool.apply_async(remoji.makeMosaic, [gifMap[gifName].framesDir + inputFrameNames[loopFrame], 'autoScale', depthPix, lilImgMap[lilImgDir], frameMap[key], colorMap, lilImgDir])
 //             pool.close()
 //             pool.join()

  //            movieMaker.wipeDir(movDir)
  //            for seqName in seqOrder:
  //                seq = definedSequences[seqName]
  //                for framePath in seq.framePaths:
  //                    frameStr = movieMaker.getFrameStr(frame, 4)
  //                    os.system('cp ' + framePath + ' ' + movDir + outputName + frameStr + '.png')
  //                    frame += 1
  //            break
  //        curLine += 1
