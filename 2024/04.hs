module Day04 where

import System.IO
import Data.List
import Data.Function
import Control.Category ((>>>))

getInput :: IO [String]
getInput = do
  isClosed <- isEOF
  if isClosed
    then return []
    else do
      l <- getLine
      r <- getInput
      return (l : r)

isXMAS :: String -> Integer
isXMAS "XMAS" = 1
isXMAS "SAMX" = 1
isXMAS _ = 0

countHorizontal :: String -> Integer
countHorizontal [] = 0
countHorizontal cs = isXMAS (take 4 cs) + countHorizontal (drop 1 cs)

countVertical4 :: String -> String -> String -> String -> Integer
countVertical4 (c1:cs1)
               (c2:cs2)
               (c3:cs3)
               (c4:cs4) = 
  isXMAS [c1,c2,c3,c4] + countVertical4 cs1 cs2 cs3 cs4
countVertical4 _ _ _ _ = 0

countDiagonal4 :: String -> String -> String -> String -> Integer
countDiagonal4 (c11:c12:c13:c14:cs1)
               ( _ :c22:c23:c24:cs2)
               ( _ :c32:c33:c34:cs3)
               (c41:c42:c43:c44:cs4) =
  isXMAS [c11,c22,c33,c44]
  + isXMAS [c41,c32,c23,c14]
  + countDiagonal4 (c12:c13:c14:cs1) (c22:c23:c24:cs2) (c32:c33:c34:cs3) (c42:c43:c44:cs4)
countDiagonal4 _ _ _ _ = 0

count4s :: [String] -> Integer
count4s (cs1:cs2:cs3:cs4:rest) =
  countVertical4 cs1 cs2 cs3 cs4
  + countDiagonal4 cs1 cs2 cs3 cs4
  + count4s (cs2:cs3:cs4:rest)
count4s _ = 0

part1 :: [String] -> Integer
part1 lines =
  count4s lines + (map countHorizontal lines & sum)

isMAS :: String -> Integer
isMAS "MAS" = 1
isMAS "SAM" = 1
isMAS _ = 0

countCrossedMAS3 :: String -> String -> String -> Integer
countCrossedMAS3 (c11:c12:c13:cs1)
                 ( _ :c22:c23:cs2)
                 (c31:c32:c33:cs3) =
  isMAS [c11,c22,c33] * isMAS [c31,c22,c13]
  + countCrossedMAS3 (c12:c13:cs1) (c22:c23:cs2) (c32:c33:cs3)
countCrossedMAS3 _ _ _ = 0

countCrossedMASs :: [String] -> Integer
countCrossedMASs (cs1:cs2:cs3:rest) = 
  countCrossedMAS3 cs1 cs2 cs3 + countCrossedMASs (cs2:cs3:rest)
countCrossedMASs _ = 0

part2 :: [String] -> Integer
part2 = countCrossedMASs
          
main :: IO ()
main = do
  input <- getInput
  input & part1 & print
  input & part2 & print
