exception EMPTY


type 'a t = 'a list * 'a list

let empty = ([], [])

let isEmpty = function
    ([], _) -> true
  | _ -> false

let splitf f =
  let rec inner = function
      (0, xs) -> ([], List.rev xs)
    | (_, []) -> raise Not_found
    | (i, x :: xs) -> let (l1, l2) = inner (i-1,xs) in (x :: l1, l2)
  in inner ((List.length f + 1) / 2, f)

let splitr r =
  let rec inner = function
      (0, xs) -> (List.rev xs, [])
    | (_, []) -> raise Not_found
    | (i, x :: xs) -> let (l1, l2) = inner (i-1,xs) in (l1, x :: l2)
  in inner ((List.length r) / 2, r)

let checkf = function
    ([], r) -> splitr r
  | (f, []) -> splitf f
  | q -> q

let cons (x, (f, r)) = checkf (x :: f, r)

let head = function
    ([], _) -> raise EMPTY
  | (x :: f, _) -> x

let tail = function
    ([], _) -> raise EMPTY
  | (x :: f, r) -> checkf (f, r)

let snoc ((f, r), x) = checkf (f, x :: r)

let last = function
  | (_, y :: _) -> y
  | (f, []) -> match splitr f with
      ([], _) -> raise EMPTY
    | (x :: _, _) -> x

let init = function
  | (f, y :: r) -> checkf (f, r)
  | (f, []) -> match splitr f with
      ([], _) -> raise EMPTY
    | (_ :: l1, l2) -> (l2, l1)