module Day03.Day03 where

import Data.Foldable (foldl')
import Data.Set as Set (Set, insert, singleton, union)

part1 :: String -> String
part1 = show . length . walk (0, 0)

walk_ :: (Int, Int) -> String -> Set (Int, Int)
walk_ pos [] = Set.singleton pos
walk_ pos (d : ds) = Set.insert pos $ walk_ next ds
  where
    next = step pos d

walk :: (Int, Int) -> String -> Set (Int, Int)
walk pos = snd . foldl' (\(p, s) d -> let n = step p d in (n, Set.insert p s)) (pos, Set.singleton pos)

step :: (Int, Int) -> Char -> (Int, Int)
step (x, y) direction = case direction of
  '^' -> (x, y + 1)
  'v' -> (x, y - 1)
  '>' -> (x + 1, y)
  '<' -> (x - 1, y)
  a -> error (a : " is not a direction last time I checked")

part2 :: String -> String
part2 input = show . length $ santa `union` robot
  where
    (santa, robot) = walk2 True (0, 0) (0, 0) input

walk2 :: Bool -> (Int, Int) -> (Int, Int) -> String -> (Set (Int, Int), Set (Int, Int))
walk2 _ pos rPos [] = (Set.singleton pos, Set.singleton rPos)
walk2 True pos rPos (d : ds) = (Set.insert pos santa, robot)
  where
    next = step pos d
    (santa, robot) = walk2 False next rPos ds
walk2 False pos rPos (d : ds) = (santa, Set.insert rPos robot)
  where
    next = step rPos d
    (santa, robot) = walk2 True pos next ds