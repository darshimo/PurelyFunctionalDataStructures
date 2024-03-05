module type OrderedType =
sig
  type t

  val eq : t -> t -> bool
  val lt : t -> t -> bool
  val leq : t -> t -> bool
end


module type S =
sig
  type elt
  type t

  val empty : t
  val insert : elt * t -> t
  val member : elt * t -> bool

  val fromOrdList : elt list -> t
end


exception Hoge


module Make (Ord : OrderedType) =
struct
  type elt = Ord.t

  type c = R | B

  type t = E | T of c * t * elt * t

  let empty = E

  let rec member = function
      (_, E) -> false
    | (x, T (_, a, y, b)) ->
      if Ord.lt x y then member (x, a)
      else if Ord.lt y x then member (x, b)
      else true

  let llbalance = function
      (B, T (R, T (R, a, x, b), y, c), z, d)-> T (R, T (B, a, x, b), y, T (B, c, z, d))
    | (c, a, x, b) -> T (c, a, x, b)

  let lrbalance = function
      (B, T (R, a, x, T (R, b, y, c)), z, d) -> T (R, T (B, a, x, b), y, T (B, c, z, d))
    | (c, a, x, b) -> T (c, a, x, b)

  let rlbalance = function
      (B, a, x, T (R, T (R, b, y, c), z, d)) -> T (R, T (B, a, x, b), y, T (B, c, z, d))
    | (c, a, x, b) -> T (c, a, x, b)

  let rrbalance = function
      (B, a, x, T (R, b, y, T (R, c, z, d))) -> T (R, T (B, a, x, b), y, T (B, c, z, d))
    | (c, a, x, b) -> T (c, a, x, b)

  let rec insert (x, s) =
    let rec ins = function
        E -> T (R, E, x, E)
      | T (R, _, _, _) -> raise Hoge
      | T (B, a, y, b) as s ->
        if Ord.lt x y then
          match a with
            T (R, c, z, d) ->
            if Ord.lt x z then llbalance (B, T (R, ins c, z, d), y, b)
            else if Ord.lt z x then lrbalance (B, T (R, c, z, ins d), y, b)
            else s
          | _ -> T (B, ins a, y, b)
        else if Ord.lt y x then
          match b with
            T (R, c, z, d) ->
            if Ord.lt x z then rlbalance (B, a, y, T (R, ins c, z, d))
            else if Ord.lt z x then rrbalance (B, a, y, T (R, c, z, ins d))
            else s
          | _ -> T (B, a, y, ins b)
        else s
    in match ins s with
      E -> raise Hoge
    | T(_, a, y, b) -> T (B, a, y, b)

  let rec take = function
      (0, xs) -> ([], xs)
    | (_, []) -> ([], [])
    | (c, x :: xs) -> let (a, b) = take (c - 1, xs) in (x :: a, b)

  let rec fromOrdList = function
      [] -> E
    | [x] -> T (R, E, x, E)
    | xs -> let (a, b) = take ((List.length xs) / 2, xs) in
      T (B, fromOrdList a, List.hd b, fromOrdList (List.tl b))
end