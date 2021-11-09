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
