{-# LANGUAGE NoImplicitPrelude #-}
module Adapter.InMemory.Authentication where

import           ClassyPrelude
import qualified Domain.Authenticate as Dom

data State = State
    { stateAuth             :: [(Dom.UserId, Dom.Auth)]
    , stateUnverifiedEmails :: Map Dom.VerificationCode Dom.Email
    , stateVerifiedEmails   :: Set Dom.Email
    , stateUserIdCounter    :: Int
    , stateNotifications    :: Map Dom.Email Dom.VerificationCode
    , stateSessions         :: Map Dom.SessionId Dom.UserId
    } deriving (Show, Eq)

initialState :: State
initialState = State
    { stateAuth             = []
    , stateUnverifiedEmails = mempty
    , stateVerifiedEmails   = mempty
    , stateUserIdCounter    = 0
    , stateNotifications    = mempty
    , stateSessions         = mempty
    }
