(def identifier
  ~{:reserved (+ "boolean" "number" "string" "->" "::")
    :char (if-not (set " \t\r\n()[]\"") 1)
    :main (/ (<- (* (if-not :reserved 1)
                    (any :char)))
             ,|[:identifier $])})

(def typee
  ~{:ws (any (set " \t\r\n"))

    :boolean (/ "boolean" ,:boolean)
    :number  (/ "number"  ,:number)
    :string  (/ "string"  ,:string)

    :function (/ 
               (* "(" :ws "->" :ws (some (* :main :ws)) ")") 
               ,(fn [& types] 
                  (def return-type (last types)) 
                  (def arg-types (slice types 0 (- (length types) 1))) 
                  [:function [:args arg-types] [:return return-type]]))

    :main (+ :function :boolean :number :string)})

(def signature
  ~{:ws (any (set " \t\r\n"))
    :identifier ,identifier
    :type ,typee

    :main (/ (* "(" :ws "::" :ws
                :identifier :ws
                :type :ws
                ")" :ws
                -1)
             ,(fn [identifier type]
                [identifier type]))})

(defn pp-match [str]
  (pp str)
  (pp (peg/match signature str))
  (print))

(print)

(pp-match "(:: add (-> (-> string number) number number))")
(pp-match "(:: three number)")
