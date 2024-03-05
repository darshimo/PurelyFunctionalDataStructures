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


module Make (H : S) =
struct
  module Ord = H.Ord

  type t = E | NE of Ord.t * H.t

  let empty = E

  let isEmpty = function
      E -> true
    | _ -> false

  let rec insert (x, h) =
    match h with
      E -> NE (x, H.insert (x, H.empty))
    | NE (y, t) -> if Ord.leq x y then NE (x, H.insert (x, t)) else NE (y, H.insert (x, t))

  let rec merge = function
      (h1, E) -> h1
    | (E, h2) -> h2
    | (NE (x, t1), NE (y, t2)) -> if Ord.leq x y then NE (x, H.merge (t1, t2)) else NE (y, H.merge (t1, t2))

  let findMin = function
      E -> raise EMPTY
    | NE (x, _) -> x

  let deleteMin = function
      E -> raise EMPTY
    | NE (_, h) -> let t = H.deleteMin h in
      if H.isEmpty t then E else NE (H.findMin t, t)
end