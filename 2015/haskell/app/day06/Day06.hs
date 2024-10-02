module Day06.Day06 where

import Data.List (elemIndex, foldl')
import Data.Maybe (fromJust)

data ToggleType = On | Off | ToggleLight
  deriving (Show)

data Toggle = Toggle
  { ttype :: ToggleType,
    start :: (Int, Int),
    end :: (Int, Int)
  }
  deriving (Show)

part1 :: String -> String
part1 = show . length . concatMap (filter id) . applyAll startBoard . map parse . lines
  where
    startBoard = replicate 1000 (replicate 1000 False)

applyAll :: [[Bool]] -> [Toggle] -> [[Bool]]
applyAll = foldl' (flip apply)

parse :: String -> Toggle
parse str
  | Just on <- removePrefix str "turn on " = dup on On
  | Just off <- removePrefix str "turn off " = dup off Off
  | Just toggle <- removePrefix str "toggle " = dup toggle ToggleLight
  | otherwise = error "not a toggle"
  where
    dup x t = uncurry (Toggle t) . fromJust $ parseRest x
    parseRest :: String -> Maybe ((Int, Int), (Int, Int))
    parseRest r = do
      let (a, r1) = split ',' r
      let (b, r2) = split ' ' r1
      r3 <- removePrefix r2 "through "
      let (c, d) = split ',' r3
      return ((read a, read b), (read c, read d))

split :: (Eq a) => a -> [a] -> ([a], [a])
split c list = let (a, b) = splitAt (fromJust $ elemIndex c list) list in (a, tail b)

apply :: Toggle -> [[Bool]] -> [[Bool]]
apply (Toggle ttype (a, b) (c, d)) board = [if i >= a && i <= c then inner b d row else row | (row, i) <- zip board [0 ..]]
  where
    f On = const True
    f Off = const False
    f ToggleLight = not
    inner l h row = [if i >= l && i <= h then f ttype x else x | (x, i) <- zip row [0 ..]]

removePrefix :: (Eq a) => [a] -> [a] -> Maybe [a]
removePrefix a b = if take len a == b then Just $ drop len a else Nothing
  where
    len = length b

part2 :: String -> String
part2 = show . sum . concat . applyAll2 startBoard . map parse . lines
  where
    startBoard = replicate 1000 (replicate 1000 0)

applyAll2 :: [[Int]] -> [Toggle] -> [[Int]]
applyAll2 = foldl' (flip apply2)

apply2 :: Toggle -> [[Int]] -> [[Int]]
apply2 (Toggle ttype (a, b) (c, d)) board = [if i >= a && i <= c then inner b d row else row | (row, i) <- zip board [0 ..]]
  where
    f On = (+ 1)
    f Off = max 0 . flip (-) 1
    f ToggleLight = (+ 2)
    inner l h row = [if i >= l && i <= h then f ttype x else x | (x, i) <- zip row [0 ..]]
