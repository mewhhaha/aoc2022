module Main where

data Hand = Rock | Paper | Scissors

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
outcome (Round Rock Paper) = Win
outcome (Round Rock Scissors) = Loss
outcome (Round Paper Scissors) = Win
outcome (Round Paper Rock) = Loss
outcome (Round Scissors Rock) = Win
outcome (Round Scissors Paper) = Loss
outcome _ = Draw

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
