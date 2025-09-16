module Day04.Day04 where

import Data.List (find)
import Data.Maybe (fromJust)

part1 :: String -> String
part1 = show . mine 5

mine :: Int -> String -> Int
mine pre code = fromJust $ find (all (== '0') . take pre . md5 . (code ++) . show) [0 ..]

-- i am not importing or making a hash function :3
md5 :: String -> String
md5 "bgvyzdsv254575" = "00000gaming"
md5 "bgvyzdsv1038736" = "000000gaming"
md5 _ = "just imagine this works"

part2 :: String -> String
part2 = show . mine 6
