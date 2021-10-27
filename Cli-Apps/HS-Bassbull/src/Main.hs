module Main where
import           Data.ByteString.Lazy (ByteString, readFile)
import qualified Data.Vector          as V
-- from cassava
import           Data.Csv

main :: IO ()
main = do
  putStrLn "hello world"
