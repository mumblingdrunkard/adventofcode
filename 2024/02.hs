module Day02 where

import System.IO
import Control.Category ((>>>))
import Data.Function ((&))

getInput :: IO [String]
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

preprocessInput = 
  map lineToListOfNumbers

isSafe xs =
  -- all consecutive elements differ by at most 3
  strictly (\a b -> abs (a - b) <=3) xs 
  -- and the list is strictly decreasing or strictly increasing
  && (strictly (>) xs || strictly (<) xs)
  where
    strictly (?) [x, x'] = x ? x'
    strictly (?) (x:xs)  = x ? head xs && strictly (?) xs

part1 :: [[Integer]] -> Integer
part1 =
  map isSafe
  >>> filter id
  >>> length
  >>> toInteger

part2 :: [[Integer]] -> Integer
part2 =
  let
    removeN xs n = l ++ r
      where (l, _:r) = splitAt n xs
    isSafeWithProblemDampener xs =
      isSafe xs
      ||  ([0..length xs - 1] & any (removeN xs >>> isSafe))
  in
    map isSafeWithProblemDampener
    >>> filter id
    >>> length
    >>> toInteger


main :: IO ()
main = do
  input' <- getInput
  let input = preprocessInput input'
  input & part1 & print
  input & part2 & print
