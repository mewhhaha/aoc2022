module Main where

data Hand = Rock | Paper | Scissors
  deriving (Enum, Eq, Bounded)

data Round = Round
  { opponent :: Hand,
    self :: Hand
  }

data Result = Win | Loss | Draw

main :: IO ()
main = do
  total <- sum . fmap (score . parseRound) . lines <$> getContents
  print total

outcome :: Round -> Result
outcome (Round {opponent, self})
  | opponent `winsAgainst` self = Loss
  | self `winsAgainst` opponent = Win
  | otherwise = Draw

score :: Round -> Int
score r@Round {self} = scoreRound (outcome r) + scoreHand self

scoreRound :: Result -> Int
scoreRound Win = 6
scoreRound Draw = 3
scoreRound Loss = 0

scoreHand :: Hand -> Int
scoreHand Rock = 1
scoreHand Paper = 2
scoreHand Scissors = 3

parseRound :: [Char] -> Round
parseRound [op, _, me] = uncurry Round $ both parseHand (op, me)
parseRound _ = undefined

parseHand :: Char -> Hand
parseHand 'A' = Rock
parseHand 'X' = Rock
parseHand 'B' = Paper
parseHand 'Y' = Paper
parseHand 'Z' = Scissors
parseHand 'C' = Scissors
parseHand _ = undefined

both :: (t -> b) -> (t, t) -> (b, b)
both f (a, b) = (f a, f b)

winsAgainst :: (Eq a, Bounded a, Enum a) => a -> a -> Bool
winsAgainst a b = a == next b

next :: (Eq a, Bounded a, Enum a) => a -> a
next e
  | e == maxBound = minBound
  | otherwise = succ e