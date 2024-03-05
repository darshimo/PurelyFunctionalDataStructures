module type OrderedType =
sig
  type t

  val eq : t -> t -> bool
  val lt : t -> t -> bool
  val leq : t -> t -> bool
end


module type S =
sig
  type key
  type 'a t

  val empty : 'a t
  val bind : key * 'a * 'a t -> 'a t
  val lookup : key * 'a t -> 'a
end


exception NOTFOUND


module Make (Ord : OrderedType) =
struct
  type key = Ord.t
  type 'a t = E | T of 'a t * key * 'a * 'a t

  let empty = E

  let rec bind (k, v, m) =
    match m with
      E -> T (E, k, v, E)
    | T (a, k', v', b) when Ord.lt k k' -> T (bind (k, v, a), k', v', b)
    | T (a, k', v', b) when Ord.lt k' k -> T (a, k', v', bind (k, v, b))
    | T (a, _, _, b) -> T (a, k, v, b)

  let rec lookup (k, m) =
    match m with
      E -> raise NOTFOUND
    | T (a, k', _, _) when Ord.lt k k' -> lookup (k, a)
    | T (_, k', _, b) when Ord.lt k' k -> lookup (k, b)
    | T (_, _, v, _) -> v
end