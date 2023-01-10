# ov
- two major ways for CPU to talk with its peripheral
  - memory-mapped I/O
  - port-mapped I/O or isolated I/O

# memory-mapped I/O
- one address space for both peripherals and the main memory
  - e.g. VGA display buffer

# port-mapped I/O
- peripherals have a separate address space from the main memory
  - port address space
  - e.g. a port address space from 0 to 8, each corresponding to a peripheral register

# ref
- [section 4.5.2](https://www.robots.ox.ac.uk/~dwm/Courses/2CO_2014/2CO-N4.pdf)
