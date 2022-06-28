module StateMonad where

import           Control.Monad.State
import qualified Data.Array          as Array
import qualified Data.Ix             as Ix
import           System.Random       (StdGen, newStdGen, randomR)

-- ##########################
-- ## defining data #########
data Player = XPlayer | OPlayer

type TileIndex = (Int, Int)

data TileState = Empty | X | O
    deriving (Eq)
instance (Show TileState) where
    show Empty = " · "
    show O     = " O "
    show X     = " X "

data GameState = GameState
    {   board         :: Array.Array TileIndex TileState
      , currentPlayer :: Player
      , generator     :: StdGen
    }

boardIndex :: [TileIndex]
boardIndex = Ix.range ((0, 0), (2, 2))

initialGameState :: StdGen -> GameState
initialGameState =
    GameState (Array.array (head boardIndex, last boardIndex)
        [(i, Empty) | i <- boardIndex])
    XPlayer

nextPlayer :: Player -> Player
nextPlayer OPlayer = XPlayer
nextPlayer XPlayer = OPlayer

tileForPlayer :: Player -> TileState
tileForPlayer XPlayer = X
tileForPlayer OPlayer = O

chooseRandomMove :: State GameState TileIndex
chooseRandomMove = do
    game <- get
    let openSpots = [ fst pair
                    | pair <- Array.assocs (board game)
                    , snd pair == Empty]
    let gen = generator game
    let (i, gen') = randomR (0, length openSpots - 1) gen
    put $ game { generator = gen' }
    return $ openSpots !! i

applyMove :: TileIndex -> State GameState ()
applyMove i = do
    game <- get
    let p = currentPlayer game
    let newBoard = board game Array.// [(i, tileForPlayer p)]
    put $ game { currentPlayer = nextPlayer p
                , board = newBoard }

isGameDone :: State GameState Bool
isGameDone = do
    game <- get
    let openSpots = [ fst pair
                    | pair <- Array.assocs (board game)
                    , snd pair == Empty ]
    return $ null openSpots

resolveTurn :: State GameState Bool
resolveTurn = do
    i <- chooseRandomMove
    applyMove i
    isGameDone
