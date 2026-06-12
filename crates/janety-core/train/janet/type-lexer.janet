(def symbols "()[]")

(def typee
  ~{:boolean (/ "boolean" ,|[:type :boolean])
    :number  (/ "number" ,|[:type :number])
    :string  (/ "string" ,|[:type :string])
    :main (+ :boolean :number :string)})

(def identifier 
  ~{:reserved (+ :type :keyword)
    :char (if-not (set " \t\r\n()[]\"") 1)
    :main (/ (<- (* (if-not :reserved 1) (any :char)))
         ,|[:identifier $])})

(def keywordd 
  ~{:type-def (/ "::" ,|[:type-def "::"])
    :type-arrow (/ "->" ,|[:type-arrow "->"])
    :main (+ :type-def :type-arrow)})

(def tokens
  ~{:spaces :s*
    :keyword ,keywordd
    :type ,typee
    :identifier ,identifier
    :symbol (/ (<- (set ,symbols)) ,|[:symbol $])
    :error (/ (<- 1) ,|[:error $])
    :token (+ :symbol :keyword :type :identifier :error)
    :main (* (any (* :spaces :token)) :spaces -1)})

(defn pp-match [str]
  (pp str)
  (pp (peg/match tokens str))
  (print))

(defn pp-match-all [arr]
  (each v arr (pp-match v)))
