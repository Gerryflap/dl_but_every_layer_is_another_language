import Data.List.Split
import Text.Printf

weights :: [[Float]]
weights = [[ 0.48936352, -0.05662829,  0.01805103,  0.27240247, -0.15629888], [-0.53693306, -0.63743633, -0.68035221,  0.94708794, -1.73457336], [ 0.7747519 , -0.49496982, -0.76751226,  0.66909695,  0.47870636], [-0.7018404 , -0.37255695, -0.69004083,  0.9095974 ,  1.14389133], [-0.28958786,  0.69477117, -0.05298883,  0.24670994, -0.49097171]]

biases :: [Float]
biases = [0.45154285,  0.0 , -0.05499396, -0.01513017, -0.14601986]

transpose :: [[Float]] -> [[Float]]
transpose ([]:_) = []
transpose mat = (map head mat): transpose (map tail mat)

relu :: Float -> Float
relu x = max 0 x

forward :: [Float] -> [Float]
forward vec = relu <$> (zipWith (+) biases $ map sum $ map (zipWith (*) vec) w)
    where w = transpose weights

parseIn :: [Char] -> [Float]
parseIn s = map (\x -> read x :: Float) $ splitOn " " s

printOut :: [Float] -> [Char]
printOut (f:[]) = show f
printOut (f:fs) = show f ++ " " ++ printOut fs

main :: IO ()
main = do
    str <- getLine
    putStrLn $ printOut $ forward $ parseIn str