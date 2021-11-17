---
title: Rubik Group
author: Daniel Sánchez
---

## Basic Notation

A single letter by itself, refers to a *clockwise* face rotation in
90º (quarter turn). If it's followed by an apostrophe means to turn
that face *counterclockwise* 90º. A number after it, marks repeated
turn.

- Side turns
    - $L$ Left
    - $D$ Down
    - $F$ Front
    - $R$ Right
    - $U$ Upper
    - $B$ Back
- Layer turns
    - $M$ Middle layer, direction as $L$
    - $E$ Equatorial layer, direction as $D$
    - $S$ Standing layer, direction as $F$
- Full turns
    - $X$ All the cube, direction as $R$
    - $Y$ All the cube, direction as $U$
    - $Z$ All the cube, direction as $F$

## Identities

$\Sigma = { L, D, F, R, U, B, M, E, S, X, Y, Z }$

A basic turn is an element of the alphabet, and it could or not be 
followed by an apostrophe. All the valid moves are either basic turns 
or a sequence of basic ones. All movements can be executed as a sum of
one lateral turn and two full turns.

The identity is equal to a neutral move. So, $I = T + T' = 4nT$.
Let $m, n \in \Sigma$ then $m+n \neq n+m$. The sum isn't commutative.
The inverse of a valid move is defined as:

$$inv(m) = \begin{cases}
    m' \quad&, m \,\text{is basic}\\
    (\text{map}\, \text{inv} \ast \text{reverse})\, m
    \quad&, m \,\text{is sequence}\\
\end{cases}$$

`I = foldl (+) I [X, Z, Y, Z']`
$I = [X, Z, Y, Z'] = [X, Y, Z, Y, 2Z]$

## Define all basic moves with X and (Y, D)

- $2T = 2T', 3T = T', 4T = I$
- $I \rightarrow X' = [2Y, X, 2Y]$
- $I_2 \rightarrow L = [Y, X, D] + [X', Y]$
- $I_2 \rightarrow R = [Y', X, D] + [X', Y]$
- $I_2 \rightarrow F = [2Y, X, D] + [X']$
- $I_2 \rightarrow B = [X, D] + [X']$
- $I_2 \rightarrow U = [2X, D] + [2X] = [Y, 2X, Y']$
