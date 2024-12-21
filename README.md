<h> Parsing the file is not included in the runtime. </h>

For the fastest time, run with release profile:

    cargo run -r

To get more stats and pretty print the solutions, run in debug mode:

    cargo run

Result on my Ryzen 5 5600x in release mode:

    7, 9, 1, 3, 8, 4, 6, 2, 5
    7, 9, 1, 6, 8, 4, 3, 2, 5
    9, 2, 3, 1, 6, 7, 8, 5, 4
    9, 2, 6, 1, 3, 7, 8, 5, 4

    48.19µs

Order of tiles:

    2  1  8
    3  0  7
    4  5  6

When pretty printing in debug mode:

    7, 9, 1, 3, 8, 4, 6, 2, 5
      y    y    p  
    B 1 pP 9 gG 5 y
      G    B    B  
      g    b    b  
    Y 3 pP 7 yY 2 p
      B    G    G  
      b    g    g  
    Y 8 pP 4 yY 6 p
      B    G    B  
    
    7, 9, 1, 6, 8, 4, 3, 2, 5
      y    y    p  
    B 1 pP 9 gG 5 y
      G    B    B  
      g    b    b  
    Y 6 pP 7 yY 2 p
      B    G    G  
      b    g    g  
    Y 8 pP 4 yY 3 p
      B    G    B  
    
    9, 2, 3, 1, 6, 7, 8, 5, 4
      p    p    y  
    g 3 Bb 2 Gg 4 G
      Y    Y    P  
      y    y    p  
    B 1 pP 9 gG 5 y
      G    B    B  
      g    b    b  
    Y 6 pP 7 yY 8 p
      B    G    B  
    
    9, 2, 6, 1, 3, 7, 8, 5, 4
      p    p    y  
    g 6 Bb 2 Gg 4 G
      Y    Y    P  
      y    y    p  
    B 1 pP 9 gG 5 y
      G    B    B  
      g    b    b  
    Y 3 pP 7 yY 8 p
      B    G    B  
    
    14500 rotations
    872 recursions
    966.153µs