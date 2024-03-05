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


module Make (Ord : OrderedType) : S with type elt = Ord.t