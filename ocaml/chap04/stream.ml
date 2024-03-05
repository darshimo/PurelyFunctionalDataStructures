type 'a stream_cell = Nil | Cons of 'a * 'a t
and 'a t = 'a stream_cell Lazy.t

let rec (++) xs ys =
  lazy (
    match xs with
      lazy Nil -> Lazy.force ys
    | lazy (Cons (x, s)) -> Cons (x, s ++ ys)
  )

let rec take p =
  lazy (
    match p with
      (0, _) -> Nil
    | (_, lazy Nil) -> Nil
    | (n, lazy (Cons (x, s))) -> Cons (x, take (n - 1, s))
  )

let rec drop p =
  lazy (
    match p with
      (0, s) -> Lazy.force s
    | (_, lazy Nil) -> Nil
    | (n, lazy (Cons (_, s))) -> Lazy.force (drop (n - 1, s))
  )

(*
let drop p =
  let rec drop' = function
      (0, s) ->  s
    | (_, lazy Nil) -> lazy Nil
    | (n, lazy (Cons (_, s))) -> drop' (n - 1, s)
  in lazy (Lazy.force (drop' p))
*)

let reverse s =
  let rec reverse' = function
      (lazy Nil, r) -> r
    | (lazy (Cons (x, s)), r) -> reverse' (s, lazy (Cons (x, r)))
  in lazy (Lazy.force (reverse' (s, lazy Nil)))