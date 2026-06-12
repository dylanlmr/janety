#(:: add (-> num num num))
#(defn add [a b] (+ a b))

(defn sub [a b] (- a b))
(defn + [a b] (sub a b))

(pp (+ 2 3))