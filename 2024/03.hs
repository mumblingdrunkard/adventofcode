module Day03 where

import System.IO
import Data.List
import Data.Function
import Data.Char
import Control.Category ((>>>))

getInput = do
  isClosed <- isEOF
  if isClosed
    then return []
    else do
      l <- getLine
      r <- getInput
      return (l : r)

preprocessInput = concat

search p (c:cs) =
  if p c
    then c:cs
    else search p cs

-- tries to parse the "(x,y)"
-- On success returns (Just (x, y), rest) where rest is the rest of s after "(x,y)"
-- On fail, returns (Nothing, s)
parseBody s =
  let (leftParen, s') = splitAt 1 s
      (x, s'') = span isDigit s'
      (comma, s''') = splitAt 1 s''
      (y, s'''') = span isDigit s'''
      rightParen = take 1 s''''
  in if leftParen == "(" 
        && x /= "" 
        && comma == "," 
        && y /= ""
        && rightParen == ")" 
      then ([(read x :: Integer, read y :: Integer)], drop 1 s'''')
      else ([], s)

-- searches for "mul" and returns the rest
searchMul ('m':'u':'l':rest) = rest
searchMul [] = ""
searchMul (c:cs) = searchMul cs

gatherMuls s =
  let s' = searchMul s
      (ns, s'') = parseBody s'
  in if s'' /= ""
    then ns ++ gatherMuls s''
    else ns

lenUntilEndOfDo ('d':'o':'(':')':rest) = 4
lenUntilEndOfDo [] = 0
lenUntilEndOfDo (c:cs) = 1 + lenUntilEndOfDo cs

lenUntilEndOfDont ('d':'o':'n':'\'':'t':'(':')':rest) = 7
lenUntilEndOfDont [] = 0
lenUntilEndOfDont (c:cs) = 1 + lenUntilEndOfDont cs

part1 =
  gatherMuls 
  >>> map (uncurry (*)) 
  >>> sum

-- strips out parts of input between "don't()" and "do()" before passing to gatherMuls
gatherMulsWithCommands input =
  let nActive = lenUntilEndOfDont input
      -- active part of string
      active = take nActive input
      rest = drop nActive input
      -- inactive part
      nInactive = lenUntilEndOfDo rest
      -- rest' is handled recursively by 
      rest' = drop nInactive rest
  in if rest' /= ""
    then gatherMuls active ++ gatherMulsWithCommands rest'
    else gatherMuls active

part2 =
  gatherMulsWithCommands >>> map (uncurry (*)) >>> sum

main = do
  input' <- getInput
  let input = preprocessInput input'
  input & part1 & print
  input & part2 & print
