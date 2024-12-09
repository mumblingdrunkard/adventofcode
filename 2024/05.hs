module Day04 where

import System.IO
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


splitOnceWhen' p acc (c:cs) =
  if p c
    then (acc, cs)
    else splitOnceWhen' p (acc ++ [c]) cs
splitOnceWhen' _ acc _ = (acc, [])
splitOnceWhen p = splitOnceWhen' p []

splitWhen' p l acc (c:cs) =
  if p c
    then l ++ [acc] ++ splitWhen' p l [] cs
    else splitWhen' p l (acc ++ [c]) cs
splitWhen' _ l acc _ = l ++ [acc]
splitWhen p = splitWhen' p [] []

parseRule s =
  let (x', y') = splitOnceWhen (=='|') s
  in (read x' :: Integer, read y' :: Integer)

parseUpdate =
  splitWhen (==',')
  >>> map (\x -> read x :: Integer)

-- splits input into sections of rules and updates
preprocessInput :: [String] -> ([(Integer, Integer)], [[Integer]])
preprocessInput ss =
  let (rules', updates') = splitOnceWhen (=="") ss
      rules = map parseRule rules'
      updates = map parseUpdate updates'
  in (rules, updates)

-- checks whether a given update follows a given rule by verifying that no f comes after an s
followsRule (f, s) =
  dropWhile (/=s)
  >>> dropWhile (/=f)
  >>> null

followsRules rules update =
  all (`followsRule` update) rules

getFirstViolation rules update =
  rules & filter (not . (`followsRule` update)) & head

middleValue s =
  s !! (length s `div` 2)

part1 (rules, updates) =
  updates
  & filter (followsRules rules)
  & map middleValue
  & sum

fixRule' (f, s) acc (c:rest)
  | c == f = acc ++ [s] ++ fixRule' (f, s) acc rest
  | c == s = acc ++ [f] ++ fixRule' (f, s) acc rest
  | otherwise = acc ++ [c] ++ fixRule' (f, s) acc rest
fixRule' _ acc _ = acc
fixRule rule = fixRule' rule []

fixRules rules update =
  if followsRules rules update
    then update
    else fixRules rules (fixRule (getFirstViolation rules update) update)

part2 (rules, updates) =
  updates
  & filter (not . followsRules rules)
  & map (fixRules rules >>> middleValue)
  & sum

main :: IO ()
main = do
  input' <- getInput
  let input = preprocessInput input'
  input & part1 & print
  input & part2 & print
