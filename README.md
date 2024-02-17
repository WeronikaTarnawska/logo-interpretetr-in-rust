# Logo interpreter written in rust

## Supported commands

### Turtle

#### Motion

`forward expr` `fd expr`
    Move turtle forward expr pixels

`back expr` `bk expr`
    Move turtle backward expr pixels

`left expr` `lt expr`
    Rotate expr degrees counterclockwise

`right expr` `rt expr`
    Rotate expr degrees clockwise

#### Turtle visibility

`showturtle` `st`
    Show the turtle

`hideturtle` `ht`
    Hide the turtle

#### Many Turtles

`setturtle index`
    Switch to the turtle numbered index (starting from 0 for the default turtle present at start). If the turtle has not been used yet, it will be created at the center, facing upwards, visible, with the pen down.

#### Pen and background

`pendown` `pd`
    Turtle resumes leaving a trail

`penup` `pu`
    Turtle stops leaving a trail

`setcolor expr`
    Set pen color to *expr*

`clearscreen`
    Clear canvas

### Control flow

#### Procedure Definition

`to procname inputs ... statements ... end`
    Define a new named procedure.
    `to star :n  repeat 5 [ fd :n rt 144 ]  end`

#### Loop

`repeat expr [ statements ... ]`
    Repeat statements expr times
    `repeat 4 [ fd 100 rt 90 ]`

#### If, IfElse

`if expr [ statements ... ]`
    Execute statements if the expression is non-zero

`ifelse expr [ statements ... ] [ statements ... ]`
    Execute first set of statements if the expression is non-zero, otherwise execute the second set

#### Wait

`wait time`
    Pauses execution. time is in 60ths of a second.

#### Return from recursive function

`stop`
    End the running procedure with no output value.

### Other commands

`show thing`
Print thing to stdout

### Arithmetic expressions

Supports infix addition $+$, substraction $-$, multiplication $*$, division $/$ in the usual operator precedence.
`expr + expr`
`expr - expr`
`expr * expr`
`expr / expr`

Comparison operators $<$, $=$ return numbers: 0 for false and 1 for true:
`expr < expr`
`expr = expr`

All numbers are float32.

### Other expressions

`random expr`
  Return random number $\in [0, expr)$

`pick [expr expr ...]`
  Pick random item from list
  `show pick [2 3 4] + pick [6 7 8]`

## Sample programms

Fern:

```txt
to fern :size :sign
  if :size < 1 [ stop ]
  fd :size
  rt 70 * :sign fern :size * 0.5 :sign * -1 lt 70 * :sign
  fd :size
  lt 70 * :sign fern :size * 0.5 :sign rt 70 * :sign
  rt 7 * :sign fern :size - 1 :sign lt 7 * :sign
  bk :size * 2
end
pu bk 100 lt 90 fd 100 rt 90
setcolor green
clearscreen pu bk 150 pd
fern 25 1
```

Squares:

```txt
to square :length
  repeat 4 [ fd :length rt 90 ]
end
to randomcolor
  setcolor pick [ red orange yellow green blue violet ]
end
clearscreen
repeat 36 [ randomcolor square random 200 rt 10 ]
```

Tree:

```txt
to tree :size
   if :size < 5 [forward :size back :size stop]
   forward :size/3
   left 30 tree :size*2/3 right 30
   forward :size/6
   right 25 tree :size/2 left 25
   forward :size/3
   right 25 tree :size/2 left 25
   forward :size/6
   back :size
end
clearscreen
tree 150
```

Chaos:

```txt
to star :size
  repeat 5 [ fd :size rt 144 ]
end

to randomcolor
  setcolor pick [ red orange yellow green blue violet ]
end


pu rt 90 fd 200 lt 90 pd randomcolor
repeat 36 [  star random 100 rt 10 ]

setturtle 1
pu rt 90 fd 100 lt 90 pd randomcolor
repeat 36 [  star random 100 rt 10 ]

setturtle 2
pu rt -90 fd 200 lt -90  pd randomcolor
repeat 36 [  star random 100 rt 10 ]

setturtle 3
pu rt -90 fd 100 lt -90 pd randomcolor
repeat 36 [  star random 100 rt 10 ]

setturtle 4 randomcolor
repeat 36 [  star random 100 rt 10 ]
```
