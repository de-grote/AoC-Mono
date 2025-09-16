module Day01.Day01 where

part1 :: String -> String
part1 input = show $ countParans input 0

countParans :: String -> Int -> Int
countParans [] c = c
countParans (x:xs) c 
    | x == '(' = countParans xs $ c + 1
    | otherwise = countParans xs $ c - 1
    

part2 :: String -> String
part2 input = show $ countParans2 input 0 0

countParans2 :: String -> Int -> Int -> Int
countParans2 [] pos _ = pos
countParans2 _ pos (-1) = pos
countParans2 (x:xs) pos elv
    | x == '(' = countParans2 xs (pos + 1) $ elv + 1
    | otherwise = countParans2 xs (pos + 1) $ elv - 1