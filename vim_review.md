1. Move Cursor: h j k l
2. Exit vim:
   1. :q
   2. :q!
   3. <Esc>
3. Deletion: x
4. Insertion: i
5. Appending: a
6. Editing a file: 
   1. :wq

Lesson2
1. Deletion commands(in normal mode)
   1. dw     d2w
   2. d$     d3$
   3. de     d2e
   4. dd
2. Count+command+Motion
3. Undo commands
   1. u
   2. U
   3. `ctrl` + `R`
4. Move to the start of the line: 0
5. Put commands: p
6. Replace command: rx
7. The change operator: 
   1. ce
   2. cc
   3. cw
   4. c$
8. Cursor location and File status:
   1. `ctrl`+`G`
   2. G
   3. gg
9. Search Command:
   1.  /
   2.  n
   3.  N
   4.  ?
   5.  `ctrl` + `o`
   6.  `ctrl`+`I`
10. Matching Parenthese search: %
11. The substitute command
    1.  :s/ole/new/g
    2.  :s/thee/the<ENTER>
    3.  :#,#s/old/new/g
    4.  :%s/old/new/g
    5.  :%s/old/new/gc

Lesson5
1. More on writing file
   1. :!dir
   2. :!ls
   3. :w TEST
   4. :!del TEST
   5. :!rm TEST
2. Selecting text to write

Lesson6
1. The open command: o
2. The append command: a
3. The replace command: R
4. Copy and paste text: y p v
5. Set option: :set ...
6. 