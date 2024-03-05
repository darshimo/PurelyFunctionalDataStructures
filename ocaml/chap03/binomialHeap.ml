module type OrderedType =
sig
  type t

  val eq : t -> t -> bool
  val lt : t -> t -> bool
  val leq : t -> t -> bool
end


module type S =
sig
  module Ord : OrderedType

  type t

  val empty : t
  val isEmpty : t -> bool

  val insert : Ord.t * t -> t
  val merge : t * t -> t

  val findMin : t -> Ord.t
  val deleteMin : t -> t
end


exception EMPTY


module Make (Order : OrderedType) =
struct
  module Ord = Order

  type tree = N of Ord.t * tree list

  type t = (int * tree) list

  let empty = []

  let isEmpty = function
      [] -> true
    | _ -> false

  let root (_, N (x, _)) = x

  let rank (r, _) = r

  let link ((r, (N (x1, c1) as t1)), (_, (N (x2, c2) as t2))) =
    if Ord.leq x1 x2 then (r + 1, N (x1, t2 :: c1))
    else (r + 1, N (x2, t1 :: c2))

  let rec insTree (t, ts) =
    match ts with
      [] -> [t]
    | t' :: ts' -> if rank t < rank t' then t :: ts else insTree (link (t, t'), ts')

  let rec insert (x, ts) = insTree ((0, N (x, [])), ts)

  let rec merge = function
      (ts1, []) -> ts1
    | ([], ts2) -> ts2
    | ((t1 :: ts1' as ts1), (t2 :: ts2' as ts2)) ->
      if rank t1 < rank t2 then t1 :: merge (ts1', ts2)
      else if rank t2 < rank t1 then t2 :: merge (ts1, ts2')
      else insTree (link (t1, t2), merge (ts1', ts2'))

  let rec removeMinTree = function
      [] -> raise EMPTY
    | [t] -> (t, [])
    | t :: ts -> let (t', ts') = removeMinTree ts in
      if Ord.lt (root t) (root t') then (t, ts) else (t' , t :: ts')

  let findMin ts = let (t, _) = removeMinTree ts in root t

  let ranking =
    let rec inner c = function
        [] -> []
      | x :: xs -> (c, x) :: inner (c + 1) xs
    in inner 0

  let deleteMin ts = let ((r, N (_, ts1)), ts2) = removeMinTree ts in merge (ranking (List.rev ts1), ts2)
end