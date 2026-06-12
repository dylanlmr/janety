# operations
(+ 1 3)

# if statements
(if (< 1 3) true false)

# definitions
(def toto (< 1 3))
(def tata toto)

# stdout
(print tata)

# (dec foldl (-> (-> b a b) b [a] b))
# (def foldl f acc l
#   (if (empty? l)
#     acc
#     (foldl f (f acc (head l)) (tail l))))


(defn foldl [f acc l]
  (if (empty? l)
    acc
    (foldl f (f acc (first l)) (slice l 1))))

(pp (foldl |(+ $0 $1) 0 [1 2 3]))