module Patient where

data Sex = Male | Female deriving (Show)
sexInitial :: Sex -> Char
sexInitial = head . show

-- Blood Type
data RhType = Pos | Neg
data ABOType = A | B | AB | O deriving (Show)
data BloodType = BloodType ABOType RhType

instance (Show BloodType) where
    show (BloodType abo rh) = show abo ++ showRh rh
        where showRh Pos = "+"
              showRh Neg = "-"

canDonateTo :: BloodType -> BloodType -> Bool
canDonateTo (BloodType O _) _               = True
canDonateTo _ (BloodType AB _)              = True
canDonateTo (BloodType A _) (BloodType A _) = True
canDonateTo (BloodType B _) (BloodType B _) = True
canDonateTo _ _                             = False

type FirstName = String
type LastName = String
type MiddleName = String
data Name
    = Name FirstName LastName
    | NameWMiddle FirstName MiddleName LastName
instance (Show Name) where
    show (Name f l)          = f ++ " " ++ l
    show (NameWMiddle f m l) = f ++ " " ++ m ++ " " ++ l

-- Record Syntax
-- data Patient = Patient Name Sex Age Height Weight BloodType
data Patient =
    Patient { name   :: Name
            , sex    :: Sex
            , age    :: Int
            , height :: Float
            , weight :: Float
            , blood  :: BloodType
            }
instance (Show Patient) where
    show (Patient n s a h w b) =
        let msgVal m v = m ++ show v ++ "\n"
            unitVal m v u = m ++ show v ++ u ++ "\n"
         in concat  [ msgVal "Patient Name:\t" n
                    , msgVal "Sex:\t\t" s
                    , msgVal "Age:\t\t" a
                    , unitVal "Height:\t\t" h "m"
                    , unitVal "Weight:\t\t" w "kg"
                    , msgVal "Blood Type:\t" b
                    , replicate 25 '*'
                    ]

