module Day09.Day09 where

import Data.List (elemIndex)
import Data.Map qualified as Map
import Data.Maybe (fromJust)
import Data.Set qualified as Set

part1 :: String -> String
part1 = show . findDistance minimum . parse

parse :: String -> Map.Map String (Set.Set (String, Int))
parse = Map.fromListWith Set.union . concatMap parseLine . lines
  where
    splitLine c line = splitAt (fromJust $ elemIndex c line) line
    parseLine line =
      let (a, distance') = splitLine '=' line
          (start, end') = splitLine ' ' a
          end = init $ drop 4 end'
          distance = read $ drop 2 distance'
       in [(start, Set.singleton (end, distance)), (end, Set.singleton (start, distance))]

findDistance :: ([Int] -> Int) -> Map.Map String (Set.Set (String, Int)) -> Int
findDistance m graph = m $ map (\l -> findDistanceR m graph (Set.singleton l) l 0) $ Map.keys graph

findDistanceR :: ([Int] -> Int) -> Map.Map String (Set.Set (String, Int)) -> Set.Set String -> String -> Int -> Int
findDistanceR m graph been loc dist
  | length been == length graph = dist
  | otherwise = m . Set.toList $ Set.map (\(l, d) -> findDistanceR m graph (Set.insert l been) l dist + d) canGo
  where
    here = (Map.!) graph loc
    canGo = Set.filter (not . flip Set.member been . fst) here

part2 :: String -> String
part2 = show . findDistance maximum . parse
