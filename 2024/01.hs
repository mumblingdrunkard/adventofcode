module Day01 where

import System.IO
import Data.List
import Data.Function
import Control.Category

getInput = do
  isClosed <- isEOF
  if isClosed
    then return []
    else do
      l <- getLine
      r <- getInput
      return (l : r)

lineToListOfNumbers =
  words >>> map (\l -> read l :: Integer)

listOfNumbersToTuple [first, second] = (first, second)

preprocessInput = map (lineToListOfNumbers >>> listOfNumbersToTuple)

part1 tuples =
  let (as', bs') = unzip tuples
      as = sort as'
      bs = sort bs'
      absDiff a b = abs (a - b)
  in zipWith absDiff as bs & sum

part2 tuples =
  let (as, bs) = unzip tuples
      count p xs = filter p xs & length & toInteger
      weight a = a * count (==a) bs
  in map weight as & sum

main = do
  input' <- getInput
  let input = preprocessInput input'
  input & part1 & print
  input & part2 & print
