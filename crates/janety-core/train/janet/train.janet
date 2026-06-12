(def hello '{:main (<- "hello")})
(pp (peg/match hello "hello"))

(def numbers 
  '{:main (<- (* :d* -1))})
(pp (peg/match numbers "056387"))

(def has-letter
  '{:main (<- (* (any :A) :a))})
(pp (peg/match has-letter "5a66"))

(def word-space-word 
  '{:main (<- (* :a+ " " :a+))})
(pp (peg/match word-space-word "toto tata"))

(def date
  '{:day (* :d :d)
    :month (* :d :d)
    :main (<- (* :day "/" :month -1))})
(pp (peg/match date "31/12"))

(def with-quotes
  '{:main (* "\"" (<- (any (if-not "\"" 1))) "\"" -1)})
(pp (peg/match with-quotes "\"toto\""))

(def yes-or-no
  '{:main (* (<- (+ "yes" "no")) -1)})
(pp (peg/match yes-or-no "yes"))

(def num-list
  '{:main (* (<- (* :d (any (* "," :d)))) -1)})
(pp (peg/match num-list "1,2,3"))

(def spe-word
  '{:main (* (<- (some (+ :w "_"))) -1)})
(pp (peg/match spe-word "__a_o_aenaen"))
