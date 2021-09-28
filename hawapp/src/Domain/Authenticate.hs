{-# LANGUAGE NoImplicitPrelude #-}
{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE QuasiQuotes       #-}
module Domain.Authenticate where

import           ClassyPrelude
import           Control.Monad.Except
import           Data.Text
import           Domain.Validation
import           Text.Regex.PCRE.Heavy

--  Authentication Data Structure
--      Types definition
data RegistrationError
    = RegistrationErrorEmailTaken
    deriving (Show, Eq)

newtype Email = Email
    { emailRaw :: Text
    } deriving (Show, Eq)

newtype Password = Password
    { passwordRaw :: Text
    } deriving (Show, Eq)

--      Implementation
rawEmail :: Email -> Text
rawEmail = emailRaw

mkEmail :: Text -> Either [Text] Email
mkEmail = validate Email
    [ regexMatches
        [re|^[A-Z0-9a-z._%+-]+@[A-Z0-9a-z.-]+\.[A-Za-z]{2,64}$|]
        "Not a valid email"
    ]

rawPassword :: Password -> Text
rawPassword = passwordRaw

mkPassword :: Text -> Either [Text] Password
mkPassword = validate Password
    [ lengthBetween 8 15        "Should be between 8 and 15"
    , regexMatches [re|\d|]     "Should contain a number"
    , regexMatches [re|[A-Z]|]  "Should contain uppercase letter"
    , regexMatches [re|[a-z]|]  "Should contain lowercase letter"
    ]

--  Registration
--      Types Definition
data Auth = Auth
    { authEmail    :: Email
    , authPassword :: Password
    } deriving (Show, Eq)

data EmailVerificationError =
    EmailVerificationErrorInvalidCode
    deriving (Show, Eq)

type VerificationCode = Text

class Monad m => AuthRepo m where
    addAuth :: Auth -> m (Either RegistrationError VerificationCode)
    setEmailAsVerified :: VerificationCode -> m (Either EmailVerificationError ())
    findUserByAuth :: Auth -> m (Maybe (UserId, Bool))

class Monad m => SessionRepo m where
    newSession :: UserId -> m SessionId
    findUserIdBySessionId :: SessionId -> m (Maybe UserId)

class Monad m => EmailVerificationNotif m where
    notifyEmailVerification :: Email -> VerificationCode -> m ()

--      Implementation
instance AuthRepo IO where
    addAuth (Auth email pass) = do
        putStrLn $ "adding auth: " <> rawEmail email
        return $ Right "01123458"

instance EmailVerificationNotif IO where
    notifyEmailVerification email vcode =
        putStrLn $ "Notify " <> rawEmail email <> " - " <> vcode

register :: (AuthRepo m, EmailVerificationNotif m) =>
    Auth -> m (Either RegistrationError ())
register auth = runExceptT $ do
    vCode <- ExceptT $ addAuth auth
    let email = authEmail auth
    lift $ notifyEmailVerification email vCode

verifyEmail :: AuthRepo m =>
    VerificationCode -> m (Either EmailVerificationError ())
verifyEmail = setEmailAsVerified

-- Login and Resolving Session
--      Types Definition
type UserId = Int

type SessionId = Text

data LoginError = LoginErrorInvalidAuth
    | LoginErrorEmailNotVerified
    deriving (Show, Eq)

resolveSessionId :: SessionRepo m -> SessionId -> m (Maybe UserId)
resolveSessionId = findUserIdBySessionId

--  Email Verification
--      Types definition
