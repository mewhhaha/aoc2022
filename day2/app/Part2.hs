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
fixHand Rock Win = Paper
fixHand Rock Loss = Scissors
fixHand Paper Win = Scissors
fixHand Paper Loss = Rock
fixHand Scissors Win = Rock
fixHand Scissors Loss = Paper
fixHand same _ = same
