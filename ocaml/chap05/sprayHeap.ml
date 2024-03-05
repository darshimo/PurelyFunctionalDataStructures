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

  val sort : Ord.t list -> Ord.t list
end


exception EMPTY


module Make (Order : OrderedType) =
struct
  module Ord = Order

  type t = E | T of t * Ord.t * t

  let empty = E

  let isEmpty = function
      E -> true
    | _ -> false

  let rec bigger = function
      (_, E) -> E
    | (pivot, T (a, x, b)) ->
      if Ord.leq x  pivot then bigger (pivot, b)
      else match a with
          E -> T (E, x, b)
        | T (a1, y, a2) ->
          if Ord.leq y pivot then T (bigger (pivot, a2), x, b)
          else T (bigger (pivot, a1), y, T (a2, x, b))

  let rec smaller = function
      (_, E) -> E
    | (pivot,T (a, x, b)) ->
      if Ord.lt pivot x then smaller (pivot, a)
      else match b with
          E -> T (a, x, E)
        | T (b1, y, b2) ->
          if Ord.lt pivot y then T (a, x, smaller (pivot, b1))
          else T (T (a, x, b1), y, smaller (pivot, b2))

  let rec partition = function
      (_, E) -> (E, E)
    | (pivot, (T (a, x, b) as t)) ->
      if Ord.leq x pivot then
        match b with
          E -> (t, E)
        | T (b1, y, b2) ->
          if Ord.leq y pivot then
            let (small, big) = partition (pivot, b2)
            in (T (T (a, x, b1), y, small), big)
          else
            let (small, big) = partition (pivot, b1)
            in (T (a, x, small), T (big, y, b2))
      else
        match a with
          E -> (E, t)
        | T (a1, y, a2) ->
          if Ord.leq y pivot then
            let (small, big) = partition (pivot, a2)
            in (T (a1, y, small), T (big, x, b))
          else
            let (small, big) = partition (pivot, a1)
            in (small, T (big, y, T (a2, x, b)))

  let insert (x, t) =
    let (small, big) = partition (x, t)
    in T (small, x, big)

  let rec merge = function
      (E, t) -> t
    | (T (a, x, b), t) ->
      let (small, big) = partition (x, t)
      in T (merge (a, small), x, merge (b, big))

  let rec findMin = function
      E -> raise EMPTY
    | T (E, x, _) -> x
    | T (a, _, _) -> findMin a

  let rec deleteMin = function
      E -> raise EMPTY
    | T (E, _, b) -> b
    | T (T (E, x, b), y, c) -> T (b, y, c)
    | T (T (a, x, b), y, c) -> T (deleteMin a, x, T (b, y, c))

  let rec l2t = function
      [] -> E
    | x :: xs -> insert (x, l2t xs)

  let t2l =
    let rec inner l = function
        E -> l
      | T (a, x, b) -> inner (x :: inner l b) a
    in inner []

  let sort l = t2l (l2t l)
end