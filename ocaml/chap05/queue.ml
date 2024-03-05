exception EMPTY


type 'a t = 'a list * 'a list

let empty = ([], [])

let isEmpty = function
    ([], _) -> true
  | _ -> false

let checkf = function
    ([], r) -> (List.rev r, [])
  | q -> q

let snoc ((f, r), x) = checkf (f, x :: r)

let head = function
    ([], _) -> raise EMPTY
  | (x :: f, r) -> x

let tail = function
    ([], _) -> raise EMPTY
  | (x :: f, r) -> checkf (f, r)