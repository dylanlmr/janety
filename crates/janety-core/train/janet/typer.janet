(import ./parser :as lex)

(def code "g/easy.g")

(def src (slurp code))

(pp src)
(print)

#(each p parsed
#  (lex/pp-match (string/format "%q" p)))

(lex/pp-match src)