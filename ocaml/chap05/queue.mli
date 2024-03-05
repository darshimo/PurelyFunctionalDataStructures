type 'a t

val empty : 'a t
val isEmpty : 'a t -> bool

val snoc : 'a t * 'a -> 'a t
val head : 'a t -> 'a
val tail : 'a t -> 'a t