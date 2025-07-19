#!/usr/bin/env runhaskell
-- 上面这行是 Shebang，让脚本可以直接执行

-- 导入必要的模块
import System.Directory (listDirectory, doesFileExist, doesDirectoryExist)
import System.FilePath ((</>), takeBaseName)
import Control.Monad (filterM, forM)
import System.IO (withFile, IOMode (WriteMode))

concatMap'' :: Monad m => (a->m[b]) -> [a] -> m[b]
concatMap'' f xs = concat <$> mapM f xs

getRecursiveContents :: FilePath -> IO [FilePath]
getRecursiveContents topPath = do
  contents <- listDirectory topPath

  let fullPaths = map (topPath </>) contents

  firstDirs <- filterM doesDirectoryExist fullPaths
  concatMap'' getSubFileName firstDirs


-- getSubDirName :: FilePath -> IO [FilePath]
-- getSubDirName path = do 
--   names <- listDirectory path
--   let fullPaths = map (path </>) names
--   filterM doesDirectoryExist fullPaths
  
getSubFileName ::FilePath -> IO [FilePath]
getSubFileName path = do 
  names <- listDirectory path
  let fullPaths = map (path </>) names
  filterM doesFileExist fullPaths

genrateTestFile :: FilePath -> IO ()
genrateTestFile path = do
  let strPath = takeBaseName path
  let testPath = strPath ++ "_test.rs"
  exists <- doesFileExist testPath
  if exists
    then putStrLn ("File " ++ testPath ++ " is existed, Skipping...")
    else withFile testPath WriteMode $ \handle -> do
      putStrLn ("File " ++ testPath ++ " created succeed.")

main :: IO ()
main = do
  putStrLn "Obtaining path name.."
  allFiles <- getRecursiveContents "../src/use_case/"
  mapM_ putStrLn allFiles
  mapM_ genrateTestFile allFiles
