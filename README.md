### Overview 

This repository contains the Rust source code 
for the algorithms in the textbook 
[Algorithms, 4th Edition](http://amzn.to/13VNJi7) 
by Robert Sedgewick and Kevin Wayne.

The official Java source code is 
[here](https://github.com/kevin-wayne/algs4).

### Goals

Make a Rust implementation of the library so that
a Rust programmer can follow this book easily or
prefer to demonstrate the algorithms using Rust.

Try to keep the interface and variable name consistent
with the original book while writing idiomatic rust
code.

I hope that this project helped you understand why 
Rust is so wonderful and loved right now. Rust is proving 
to be a productive tool for reliable and efficient software. 
In Rust, the compiler plays
a gatekeeper role by refusing to compile code with
these elusive bugs, including concurrency bugs.
By working alongside the compiler, you can spend your
time focusing on the program’s logic rather than
chasing down bugs. After you finish a day's work,
go home and rest, can be at ease a good night's sleep,
never worry about system crash.

### Index

The table index [follow](https://algs4.cs.princeton.edu/code/).

| 1   | FUNDAMENTALS         |                                  |
|-----|----------------------|----------------------------------|
| 1.2 | Stack                | LIFO stack                       |
| 1.3 | Queue                | FIFO queue                       |
| 1.4 | LinkedList           | multiset (linked list)           |
| 1.5 | QuickFindUF          | quick find                       |
| -   | QuickUnionUF         | quick union                      |
| -   | WeightedQuickUnionUF | weighted quick union             |
| -   | UF                   | union-by-rank with path halving  |
| 2   | SORTING              |                                  |
| 2.1 | insert.rs            | insertion sort                   |
| 2.2 | selection.rs         | selection sort                   |
| 2.3 | shell.rs             | shellsort                        |
| 2.4 | merge.rs             | Merge Sort                       |
| 2.5 | quick.rs             | quicksort                        |
| -   | Quick3Way            | quicksort with 3-way partitioning |
| 2.6 | PQ::new_max_pq       | max heap priority queue          |
| -   | PQ::new_min_pq       | min heap priority queue          |
| -   | IndexPQ::new_min_pq  | index min heap priority queue    |
| -   | IndexPQ::new_max_pq  | index max heap priority queue    |
| -   | TopM                 | Find the largest M elements      |
| 2.7 | floyd.rs             | heapsort                         |
| 3   | SEARCHING            |                                  |
| 3.4 | rb2.rs               | red-black tree                   |
| 3.6 | SparseVector         | sparse vector                    |
| 4   | GRAPHS               |                                  |
| -   | Graph                | undirected graph                 |
| -   | DepthFirstSearch     | depth-first search in a graph    | 
| -   | NonRecursiveDFS      | DFS in a graph (nonrecursive)    |
| 4.1 | DepthFirstPaths      | paths in a graph (DFS)           |
| 4.2 | BreadthFirstPaths    | paths in a graph (BFS)           |
| 4.3 | CC                   | connected components of a graph  |
| -   | Bipartite            | bipartite or odd cycle (DFS)     |
| -   | Cycle                | cycle in a graph                 |
| -   | SymbolGraph          | symbol graph                     |
| -   | Digraph              | directed graph                   |
| 4.4 | DepthFirstPaths      | paths in a digraph (DFS)         |
| -   | BreadthFirstPaths    | paths in a digraph (BFS)         |
| -   | DirectedCycle        | cycle in a digraph               |
| 4.5 | Topological          | topological order in a DAG       |
| -   | TransitiveClosure    | transitive closure               |
| 4.6 | KosarajuSCC          | strong components (Kosaraju–Sharir) |
| -   | EWGraph              | edge-weighted graph              |
| -   | LazyPrimMST          | MST (lazy Prim)                  |
| 4.7 | PrimMST              | MST (Prim)                       |
| 4.8 | KruskalMST           | MST (Kruskal)                    |
| -   | EdgeWeightedDigraphCycle | edge-weighted digraph        |
| 4.9 | DijkstraSP           | shortest paths (Dijkstra)        |
| -   | DijkstraAllPairsSP   | all-pairs shortest paths         |
| 4.10 | AcyclicSP           | shortest paths in a DAG          |
| -    | AcyclicLP           | longest paths in a DAG           |
| -   | CPM                  | critical path method             |
| 4.11 | BellmanFordSP       | shortest paths (Bellman–Ford)    |
| -   | Arbitrage            | arbitrage detection              |
| 5   | STRINGS              |                                  |
| -   | Alphabet             | alphabet                         |
| -   | count.rs             | alphabet client                  |
| 5.1 | LSD                  | LSD radix sort                   |
| 5.2 | MSD                  | MSD radix sort                   |
| 5.3 | Quick3String         | 3-way string quicksort           |
| 5.4 | TrieST               | multiway trie symbol table       |
| 5.5 | TST                  | ternary search trie              |
| 5.6 | KMP                  | substring search (Knuth–Morris–Pratt) |



### Running

```
# setup Rust Toolchain
make test
```

### Roadmap

- Implement algorithms in the textbook
  [Algorithms, 4th Edition](http://amzn.to/13VNJi7)
- Algorithms visualization (Easy to study and short learning curve)