module Domain.Auth where

data Auth = Auth
    { authEmail    :: Text
    , authPassword :: Text
    } deriving (Show, Eq)


