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


module Make (Ord : OrderedType) : S with type key = Ord.t