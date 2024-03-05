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
  val complete : elt * int -> t
  val semicomplete : elt * int -> t

  val fringe : t -> elt list
end


exception EXISTS;;


module Make (Ord : OrderedType) =
struct
  type elt = Ord.t
  type t = E | T of t * elt * t

  let empty = E

  let rec member (x, t, b) =
    let rec inner (x, t, c, o) =
      match t with
        E -> (
          match o with
            Some (y) when x = y -> if b then (print_int c; print_newline ()) else (); true
          |   _ -> if b then (print_int c; print_newline ()) else (); false
        )
      |   T (a, y, b) -> if Ord.leq y x then inner (x, b, c + 1, Some(y)) else inner (x, a, c + 1, o)
    in inner (x, t, 0, None)

  let rec insert (x, t, b) =
    let rec inner (x, t, c, o) =
      match t with
        E -> (
          match o with
            Some (y) when x = y -> raise EXISTS
          |   _ -> if b then (print_int c; print_newline ()) else (); T (E, x, E)
        )
      |   T (a, y, b) -> if Ord.leq y x then T (a, y, inner (x, b, c + 1, Some(y))) else T (inner (x, a, c + 1, o), y, b)
    in inner (x, t, 0, None)

  let rec complete (x, d) =
    if d <= 0 then E else let tmp = complete(x, d - 1) in T (tmp, x , tmp)

  let rec create2 (x, m) =
    if m == 0 then (E,T(E,x,E))
    else let (t1, t2) = create2(x,(m-1)/2) in
      if m mod 2 == 0 then (T(t2,x,t1),T(t2,x,t2))
      else (T(t1,x,t1),T(t2,x,t1))

  let rec semicomplete (x, n) = let (t, _) = create2(x,n) in t

  let rec fringe = function
      E -> []
    |   T (a, x, b) -> List.append (fringe a) (x :: fringe b)
end