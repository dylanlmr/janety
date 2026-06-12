

(defmacro add
  [x y]
  (with-syms [$x $y]
    ~(let [,$x ,x
           ,$y ,y] 
       (if-not (number? ,$x) ,(error (string "number required, but got " (type 'x))))
       (if-not (number? ,$y) ,(error (string "number required, but got " (type 'y))))
       (+ ,$x ,$y))))

(pp (macex1 '(add "tset" 3)))
(pp (add 3 3))