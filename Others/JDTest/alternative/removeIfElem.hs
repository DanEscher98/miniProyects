module JohnDeereChallenges where

import           Control.Monad.State

intersection :: (Eq a) => [a] -> [a] -> [a]
intersection _ [] = []
intersection [] _ = []
intersection (x:xs) ys
    | state     = x : intersection xs ys'
    | otherwise = intersection xs ys'
    where (state, ys') = ifElemRemove (False, x) ys

removeClon :: (Eq a) => [a] -> [a]
removeClon []     = []
removeClon (x:xs)
    | state     = removeClon xs'
    | otherwise = x : removeClon xs'
    where (xs', state) = ifElemRemove' True x xs

-----------------------------------------------------------
-- Auxiliar Functions -------------------------------------
ifElemRemove' :: (Eq a) => Bool -> a -> [a] -> ([a], Bool)
ifElemRemove' all e xs = runState (loop e xs) False where
    loop _ [] = return []
    loop e (x:xs) = do
        xs' <- loop e xs
        if e /= x then return (x:xs')
                  else do
                      put True
                      if all then loop e xs
                             else return xs

ifElemRemove :: (Eq a) => (Bool, a) -> [a] -> (Bool, [a])
ifElemRemove (c, e) = auxLoop (False, e) where
    auxLoop (state, _) [] = (state, [])
    auxLoop (state, e) (x:xs)
        | e /= x    = (state', x:xs')
        | otherwise = if c then auxLoop (True, e) xs
                           else (True, xs)
        where (state', xs') = auxLoop (state, e) xs
