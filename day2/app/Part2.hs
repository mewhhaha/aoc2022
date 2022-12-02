module Main where

data Hand = Rock | Paper | Scissors
  deriving (Eq, Enum, Bounded)

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

-- >>> score <$> [Round Rock Paper, Round Paper Rock, Round Scissors Scissors]
-- [8,1,6]
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
parseRound [op, _, me] =
  let opponent = parseHand op
   in Round opponent (fixHand opponent . parseResult $ me)
parseRound _ = undefined

parseHand :: Char -> Hand
parseHand 'A' = Rock
parseHand 'B' = Paper
parseHand 'C' = Scissors
parseHand _ = undefined

parseResult :: Char -> Result
parseResult 'X' = Loss
parseResult 'Y' = Draw
parseResult 'Z' = Win
parseResult _ = undefined

fixHand :: Hand -> Result -> Hand
fixHand h Win = next h
fixHand h Loss = prev h
fixHand h _ = h

winsAgainst :: (Eq a, Bounded a, Enum a) => a -> a -> Bool
winsAgainst a b = a == next b

next :: (Eq a, Bounded a, Enum a) => a -> a
next e
  | e == maxBound = minBound
  | otherwise = succ e

prev :: (Eq a, Bounded a, Enum a) => a -> a
prev e
  | e == minBound = maxBound
  | otherwise = pred e