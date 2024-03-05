type 'a stream_cell = Nil | Cons of 'a * 'a t
and 'a t = 'a stream_cell Lazy.t

val (++) : 'a t -> 'a t -> 'a t
val take : int * 'a t -> 'a t
val drop : int * 'a t -> 'a t
val reverse : 'a t -> 'a t
