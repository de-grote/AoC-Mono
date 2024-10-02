module Main where

import Day01.Day01 as Day01
import Day02.Day02 as Day02
import Day03.Day03 as Day03
import Day04.Day04 as Day04
import Day05.Day05 as Day05
import Day06.Day06 as Day06
import System.Environment (getArgs)

main :: IO ()
main = do
  args <- getArgs
  let (day, part) = dayAndPart args
  input <- readFile $ "app/day" ++ numToString day ++ "/input.txt"
  let output = solve day part input
  putStrLn output

defaultDay :: (Int, Int)
defaultDay = (6, 1)

dayAndPart :: [String] -> (Int, Int)
dayAndPart [day, part] = (read day, read part)
dayAndPart _ = defaultDay

solve :: Int -> Int -> (String -> String)
solve 1 1 = Day01.part1
solve 1 2 = Day01.part2
solve 2 1 = Day02.part1
solve 2 2 = Day02.part2
solve 3 1 = Day03.part1
solve 3 2 = Day03.part2
solve 4 1 = Day04.part1
solve 4 2 = Day04.part2
solve 5 1 = Day05.part1
solve 5 2 = Day05.part2
solve 6 1 = Day06.part1
solve 6 2 = Day06.part2
solve _ _ = uncurry solve defaultDay

numToString :: Int -> String
numToString n = let s = show n in if length s == 1 then '0' : s else s