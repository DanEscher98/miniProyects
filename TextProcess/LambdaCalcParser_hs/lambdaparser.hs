module LambdaParser where

data Term
    = Var Int
    | Hol Term
    | Lam (Term -> Term)
    | App Term Term

toString :: Term -> String
toString = aux 0 where
    aux level term = case term of
        Var index   -> "x" ++ show index
        Lam body    -> "λx" ++ show level ++ ". " ++
            aux (succ level) (body (Hol (Var level)))
        Hol hol     -> aux level hol
        App fun arg -> "(" ++ aux level fun ++ " " ++
            aux level arg ++ ")"

reduce :: Term -> Term
reduce (Hol hol)    = hol
reduce (Var index)   = Var index
reduce (Lam body)   = Lam (reduce . body)
reduce (App fun arg)= case reduce fun of
    Hol fhol    -> 2
