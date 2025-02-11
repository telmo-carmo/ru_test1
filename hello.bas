REM a simple IO test 

DIM res1 AS INTEGER

INPUT "What's your name"; name$
PRINT "Hello,"; name$ + "!"
res1 = LEN(name$)