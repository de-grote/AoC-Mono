{-# LANGUAGE UndecidableInstances #-}

module Day07.Day07 where

import Control.Monad.State
import Data.Bits (Bits (complement, shiftL, shiftR, (.&.), (.|.)))
import Data.Char (isDigit)
import Data.Map (Map, fromList, insert, (!), (!?))
import Data.Tuple (swap)
import Data.Word (Word16)

data Variable = Register String | Literal Word16
  deriving (Show)

data Instruction
  = Store Variable
  | Not Variable
  | And Variable Variable
  | Or Variable Variable
  | RShift Variable Int
  | LShift Variable Int
  deriving (Show)

type InstructionCache = Map String Word16

part1 :: String -> String
part1 = show . flip evalState mempty . getValue "a" . fromList . map (swap . parseInstruction) . lines

getValue :: String -> Map String Instruction -> State InstructionCache Word16
getValue register maps = flip evalInstruction maps $ maps ! register

evalVariable :: Variable -> Map String Instruction -> State InstructionCache Word16
evalVariable (Literal x) _ = return x
evalVariable (Register s) maps = do
  cache <- get
  maybe
    ( do
        let (res, nstate) = runState (getValue s maps) cache
        put $ insert s res nstate
        return res
    )
    return
    (cache !? s)

evalInstruction :: Instruction -> Map String Instruction -> State InstructionCache Word16
evalInstruction (Store v) maps = evalVariable v maps
evalInstruction (Not v) maps = complement <$> evalVariable v maps
evalInstruction (And v w) maps = (.&.) <$> evalVariable v maps <*> evalVariable w maps
evalInstruction (Or v w) maps = (.|.) <$> evalVariable v maps <*> evalVariable w maps
evalInstruction (LShift v w) maps = flip shiftL w <$> evalVariable v maps
evalInstruction (RShift v w) maps = flip shiftR w <$> evalVariable v maps

parseVariable :: String -> Variable
parseVariable str
  | all isDigit str = Literal $ read str
  | otherwise = Register str

parseInstruction :: String -> (Instruction, String)
parseInstruction str = case words str of
  [v, "->", end] -> (Store $ parseVariable v, end)
  ["NOT", v, "->", end] -> (Not $ parseVariable v, end)
  [v, "AND", w, "->", end] -> (And (parseVariable v) (parseVariable w), end)
  [v, "OR", w, "->", end] -> (Or (parseVariable v) (parseVariable w), end)
  [v, "LSHIFT", w, "->", end] -> (LShift (parseVariable v) (read w), end)
  [v, "RSHIFT", w, "->", end] -> (RShift (parseVariable v) (read w), end)
  _ -> undefined

part2 :: String -> String
part2 input = show . flip evalState mempty . getValue "a" $ insert "b" (Store (Literal result1)) instructions
  where
    instructions = fromList . map (swap . parseInstruction) . lines $ input
    result1 = flip evalState mempty . getValue "a" $ instructions