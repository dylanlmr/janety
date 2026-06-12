(import ./lexer :as l)

(def ints ["3" "32" "0678" "  3  " " 234234 "])
(def ops ["(+ 1 4)" "(/ 34 98)" "   (  *  134  47 )  "])
(def if-statement "  (  if ( > 34 333) ( if (  = \"test\" \"test\" ) true false ) \"toto\" )")
(def lets ["let truex [1 2 3]"])


(l/pp-match-all ints)
(l/pp-match-all ops)
(l/pp-match if-statement)
(l/pp-match-all lets)