module Day05.Day05 where

part1 :: String -> String
part1 = show . length . filter nice . lines

nice :: String -> Bool
nice str = vowels && hasDouble str && not (evilStr str)
  where
    vowels = length (filter (`elem` "aeiou") str) >= 3
    hasDouble (a : b : bs) = a == b || hasDouble (b : bs)
    hasDouble _ = False
    evilStr (a : b : bs) = elem [a, b] ["ab", "cd", "pq", "xy"] || evilStr (b : bs)
    evilStr _ = False

part2 :: String -> String
part2 = show . length . filter nice2 . lines

nice2 :: String -> Bool
nice2 str = pairs str && aba str
  where
    pairs (a : b : bs) = substr a b bs || pairs (b:bs)
    pairs _ = False
    substr p q (a:b:bs) = (p == a && q == b) || substr p q (b:bs)
    substr _ _ _ = False
    aba (a:b:c:cs) = a == c || aba (b:c:cs)
    aba _ = False
