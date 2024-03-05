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

  val fromList : Ord.t list -> t
end


exception EMPTY


module Make (Order : OrderedType) =
struct
  module Ord = Order

  type t = E | T of int * Ord.t * t * t

  let rank = function
      E -> 0
    | T (r, _, _, _) -> r

  let makeT (x, a, b) =
    if rank a >= rank b then T (rank b + 1, x, a, b)
    else T (rank a + 1, x, b, a)

  let empty = E

  let isEmpty = function
      E -> true
    | _ -> false

  let rec merge = function
      (h1, E) -> h1
    | (E, h2) -> h2
    | ((T (_, x, a1, b1) as h1) , (T(_, y, a2, b2) as h2)) ->
      if Ord.leq x y then makeT (x, a1, merge (b1,h2)) else makeT (y, a2, merge(h1,b2))

  let rec insert (x, h) = merge (T(1, x, E, E), h)

  let findMin = function
      E -> raise EMPTY
    | T (_, x, _, _) -> x

  let deleteMin = function
      E -> raise EMPTY
    | T (_, x, a, b) -> merge (a, b)

  let rec list2list = function
      [] -> []
    | x :: xs -> T (1, x, E, E) :: list2list xs

  let rec listMerge = function
      x :: y :: s -> merge (x, y) :: listMerge s
    | l -> l

  let rec fromList el =
    let rec sub = function
        [] -> raise EMPTY
      | x :: [] -> x
      | l -> sub (listMerge l)
    in sub (list2list el)
end