(def symbols "<>*{()}=+-~[]_!;:?")

(def litterals
  ~{:bool (/ (* (<- (+ "true" "false")) -1) ,|[:bool $])
    :int (/ (<- :d+) ,|[:int $])
    :string (/ (* "\"" (<- (any (if-not "\"" 1))) "\"") ,|[:string $])
    :main (+ :bool :int :string)})

(def tokens
  ~{:spaces :s*
    :valid-ident (if-not (+ :d (set " ()[]\"") :litteral) 1)
    :identifier (/ (<- (* :valid-ident (any :valid-ident))) ,|[:identifier $])
    :litteral ,litterals
    :keyword (/ (<- (+ "if" "let")) ,|[:keyword $])
    :symbol (/ (<- (set ,symbols)) ,|[:symbol $])
    :token (+ :symbol :keyword :litteral :identifier)
    :main (any (* :spaces :token))})

(defn pp-match [str]
  (pp str)
  (pp (peg/match tokens str))
  (print))

(defn pp-match-all [arr]
  (each v arr (pp-match v)))
