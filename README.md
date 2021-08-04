### Collection of algorithms in Rust

#### common

  - find the largest M elements (TopM)
  - priority queue (PQ)
  - union find (UF)
  - LinkedList
  - Queue
  - Stack

#### graph

- directed graph
  - cycle detection (DirectedCycle, EdgeWeightedDigraphCycle)
  - strong connectivity in digraphs (KosarajuSCC)
  - topological sort (Topological)
  - transitive closure (TransitiveClosure)
  - Finding paths (DepthFirstPaths, BreadthFirstPaths)
- minimum spanning trees 
  - Lazy version of Prim’s MST algorithm (LazyPrimMST)
  - Prim’s algorithm (PrimMST)
  - Kruskal’s algorithm (KruskalMST)
- shortest paths
  - Dijkstra’s algorithm (DijkstraSP, DijkstraAllPairsSP)
- undirected graph
  - bipartite (Bipartite)
  - connected components (CC)
  - cycle detection (Cycle)
  - depth first search (DepthFirstSearch, NonRecursiveDFS)
  - Finding paths (DepthFirstPaths, BreadthFirstPaths)
  - Symbol graphs (SymbolGraph)

#### search
  - Binary Search (binary.rs)

#### sort
  - Bubble Sort (bubble.rs)
  - Insertion Sort (insert.rs)
  - Merge Sort (merge.rs)
  - Quick Sort (quick.rs)
  - Robert. W. Floyd Heap sort (floyd_sort)
  - Selection Sort (selection.rs)
  - Shell's Sort (shell.rs)
  - Tournament Tree Sort (tree_selection.rs)

#### tree
  - Binary Search Tree (bst.rs)
  - Red black (rb2.rs) 

#### Running

```
# setup Rust Toolchain
make test
```

#### References

Algorithms 4th Edition by Robert Sedgewick, Kevin Wayne
