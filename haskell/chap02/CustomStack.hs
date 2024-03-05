module CustomStack (CustomStack) where
    import Prelude hiding (head, tail)
    import Stack

    data CustomStack a = Nil | Cons a (CustomStack a)

    instance Stack CustomStack where
        empty = Nil
        isEmpty Nil = True
        isEmpty _ = False

        cons (x, s) = Cons x s

        head Nil = error "Empty"
        head (Cons x _) = x

        tail Nil = error "Empty"
        tail (Cons _ xs) = xs

        fringe Nil = []
        fringe (Cons x xs) = x : fringe xs