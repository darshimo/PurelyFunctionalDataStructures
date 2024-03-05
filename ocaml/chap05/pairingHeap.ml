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

  type t = E | T of Ord.t * t list

  let empty = E

  let isEmpty = function
      E -> true
    | _ -> false

  let merge = function
      (h, E) -> h
    | (E, h) -> h
    | ((T (x, hs1) as h1), (T (y, hs2) as h2)) ->
      if Ord.leq x y then T (x, h2 :: hs1) else T (y, h1 :: hs2)

  let inesrt = function
      (x, h) -> merge (T (x, []), h)

  let rec mergePairs = function
      [] -> E
    | [h] -> h
    | h1 :: h2 :: hs -> merge (merge (h1, h2), mergePairs hs)

  let deleteMin = function
      E -> raise EMPTY
    | T (_, hs) -> mergePairs hs

  let findMin = function
      E -> raise EMPTY
    | T (t, _) -> t
end