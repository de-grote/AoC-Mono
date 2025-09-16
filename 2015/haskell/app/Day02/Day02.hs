module Day02.Day02 where

import Data.List (elemIndex)
import Data.Maybe (fromJust)

part1 :: String -> String
part1 = show . sum . map ((\(x, y, z) -> let sides = [x * y, y * z, x * z] in 2 * sum sides + minimum sides) . parseLine) . lines

parseLine :: String -> (Int, Int, Int)
parseLine str = (read x, read y, read z)
  where
    split s = splitAt (fromJust $ elemIndex 'x' s) s
    (x, rest) = split str
    (y, z') = split $ tail rest
    z = tail z'

part2 :: String -> String
part2 = show . sum . map ((\(x, y, z) -> let sides = [x, y, z] in 2 * (sum sides - maximum sides) + product sides) . parseLine) . lines
