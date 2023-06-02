# Valid expressions

## `exp -> number`
- number:                       integer | float
- normal arithmetic (nari):     nexp + nexp | nexp - nexp | nexp * nexp | nexp / nexp
- number expression (nexp):     number | (nari)

## `exp -> amount`
- currency (curr):              `USD` | `MXN` | `ARS` | `EUR` | `BTC` | `ETH` | `DOT`
- amount:                       number ~ currency
    example: `3MXN` | `4.6USD` | `3435.2351BTC`
- currency arithmetic (cari):   cexp + cexp | cexp - cexp | cexp * nexp | nexp * cexp | cexp / nexp | (cexp * cexp) / cexp
    example: `(3USD * 4MXN) / 5USD` | `20MXN + 3.5USD` | `(25USD - 2EUR) * (3 / 4)` 
- currency expression (cexp):   amount | (cari) | eval_c
- eval currency (eval_c):       cexp -> currency
    example: `(4 + 2.3 - 7) ARS -> USD -> MXN` | `(45USD - 23ARS) -> ARS`

## `exp -> bool`
- comparission (cmp):           cexp > cexp | cexp < cexp | cexp = cexp
- eval comparission (eval_cmp): cmp -> currency
    example: `(3USD -> EUR -> MXN) > (3USD -> ARS -> MXN)`
