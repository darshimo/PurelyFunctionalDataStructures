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


module Make (Ord : OrderedType) : S with type elt = Ord.t