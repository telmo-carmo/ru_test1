
my_string$ = "aa;bb;cc;ddd"
delimiter$ = ";"


DIM parts(4)  AS STRING '  Define an array to hold the parts, start idx is 0
count = 0
start = 0
FOR i = 1 TO LEN(my_string$)-1
    IF MID$(my_string$, i, 1) = delimiter$ THEN
        parts(count) = MID$(my_string$, start, i - start)
        count = count + 1
        start = i + 1
    END IF
NEXT

parts(count) = MID$(my_string$, start, LEN(my_string$) - start + 1) ' Get the last part

FOR i = LBOUND%(parts) TO UBOUND%(parts)
    PRINT i, parts(i)
NEXT

res1 = LEN(my_string$)
