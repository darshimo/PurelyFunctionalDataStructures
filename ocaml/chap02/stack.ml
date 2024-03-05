exception EMPTY

type 'a t = Nil | Cons of 'a * 'a t

let empty = Nil
let isEmpty = function
    Nil -> true
  | _ -> false

let cons (x, s) = Cons (x, s)
let head = function
    Nil -> raise EMPTY
  | Cons (x, _) -> x
let tail = function
    Nil -> raise EMPTY
  | Cons (_, s) -> s