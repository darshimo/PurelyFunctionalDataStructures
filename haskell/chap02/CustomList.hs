module CustomList (CustomList) where
    import Prelude hiding (head, tail)
    import Stack

    data CustomList a = L [a]

    instance Stack CustomList where
        empty = L []
        isEmpty (L []) = True
        isEmpty _ = False

        cons (x, L xs) = L (x : xs)

        head (L []) = error "Empty"
        head (L (x : _)) = x

        tail (L []) = error "Empty"
        tail (L (_ : xs)) = L xs

        fringe (L s) = s