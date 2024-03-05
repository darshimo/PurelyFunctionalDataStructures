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
  val insert : elt * t * bool -> t
  val member : elt * t * bool -> bool

  val fringe : t -> elt list
end


module Make (Ord : OrderedType) =
struct
  type elt = Ord.t
  type t = E | T of t * elt * t

  let empty = E

  let rec member (x, t, b) =
    let rec inner (x, t, c) =
      match t with
        E -> if b then (print_int c; print_newline ()) else (); false
      |   T (a, y, _) when Ord.lt x y ->  inner (x, a, c + 1)
      |   T (_, y, b) when Ord.lt y x ->  inner (x, b, c + 2)
      |   _ -> if b then (print_int (c + 2); print_newline ()) else (); true
    in inner (x, t, 0)

  let rec insert (x, t, b) =
    let rec inner (x, t, c) =
      match t with
        E -> if b then (print_int c; print_newline ()) else (); T (E, x, E)
      |   T (a, y, b) when Ord.lt x y -> T (inner (x, a, c + 1), y, b)
      |   T (a, y, b) when Ord.lt y x -> T (a, y, inner (x, b, c + 2))
      |   t -> if b then (print_int c; print_newline ()) else (); t
    in inner (x, t, 0)

  let rec fringe = function
      E -> []
    |   T (a, x, b) -> List.append (fringe a) (x :: fringe b)
end

