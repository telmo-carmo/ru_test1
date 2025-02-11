' from EndBASIC
' Copyright 2024 Julio Merino
'  https://github.com/endbasic/endbasic/blob/master/repl/examples/fibonacci.bas
'

DIM SHARED steps AS INTEGER

FUNCTION fibonacci(n AS INTEGER)
    SELECT CASE n
        CASE 0: fibonacci = 0
        CASE 1: fibonacci = 1
        CASE ELSE: fibonacci = fibonacci(n - 1) + fibonacci(n - 2)
    END SELECT
    steps = steps + 1
END FUNCTION

DIM res1 AS INTEGER

res1 = fibonacci(20)
PRINT "fibonacci of 10 is:"; res1
PRINT "took"; steps; "steps to calculate"