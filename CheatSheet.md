# Algorithms and Data Structures Cheatsheet

We summarize the performance characteristics of classic algorithms and data structures for sorting, priority queues, symbol tables, and graph processing.

We also summarize some of the mathematics useful in the analysis of algorithms, including commonly encountered functions; useful formulas and approximations; properties of logarithms; asymptotic notations; and solutions to divide-and-conquer recurrences.


### Sorting

 The table below summarizes the number of compares for a variety of sorting algorithms, as implemented in this textbook. It includes leading constants but ignores lower-order terms.


### Which string sorting algorithm should I use?

order of growth of typical number calls to charAt() to sort N Strings
from an R-character alphabet (average length w, max length W)

| algorithm | stable? | inplace? | running time | extra space | sweet spot | 
|-----------|---------|----------|--------------|-------------|------------|
| insertion sort for strings | yes | yes | between N and N^2 | 1 | small arrays, arrays in order|
| quicksort | no | yes | N * log^2 N | log N | general-purpose when space is tight |
| mergesort | yes | no | N * log^2 N | N | general-purpose stable sort |
| 3-way quicksort | no | yes | between N and N * log N | log N | large number of equal keys |
| LSD string sort | yes | no | N * W | N | short fixed length strings |
| MSD string sort | yes | no | between N and N * w | N + W * R | random strings |
| 3-way string quicksort | no | yes | between N and N * w | W + log N | general-purpose strings with long prefix maches|







