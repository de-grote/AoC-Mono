module Day08.Day08 where

part1 :: String -> String
part1 = show . foldr (\str acc -> acc + length str - encodedLength str) 0 . lines

encodedLength :: String -> Int
encodedLength ('\\' : 'x' : _ : _ : xs) = 1 + encodedLength xs
encodedLength ('\\' : '"' : xs) = 1 + encodedLength xs
encodedLength ('\\' : '\\' : xs) = 1 + encodedLength xs
encodedLength (_ : xs) = 1 + encodedLength xs
encodedLength [] = -2

encodeLength :: String -> Int
encodeLength =
  foldr
    ( \x acc ->
        acc + case x of
          '"' -> 2
          '\\' -> 2
          _ -> 1
    )
    2

part2 :: String -> String
part2 = show . foldr (\str acc -> acc + encodeLength str - length str) 0 . lines
