### Todo list

- algs4 exercises

- algs4, 1.5.16 plots
  - rust plots crate: https://github.com/38/plotters.git
  - 借助rust feature条件编译，实现记录统计数据的代码，不影响正常情况的性能
  
- benchmark可视化。找一种通用性能剖析方法，比如UF，在union函数中监视parent 和 rank被读了
  多少次，被写了多少次，方便plot。rust如果不具备这个功能，考虑其他语言python,
  js, kotlin... (python也许是个好选择，并且有完善的plot库)
  这个语言需要具备的必要条件：
    - 监听变量（读 / 写）
    - plot
    - 读写本地文件

- algorithms visualization

  algorithms / runtime data / anim render engine...,
  independent of each other, connected by gRPC

  algorithms impl as Server
  anim render engine as Client, render algorithms runtime data
  protobuf choose tonic

  The arch of system must contain all kinds of algorithms,
  a kind of algorithm, impl as a plugin, follow spec，amin engine
  can render it.

- fix mod/struct/fn doc error to make doc work well, add more docs

- The [Cheat Sheet](https://algs4.cs.princeton.edu/cheatsheet/) is comprehensive,
  rewrite to markdown.

- resort Index section in README.md follow order showed in textbook

- strings algorithms lsd/msd/quick3 based bytes

  refact to support chars, std str.chars() poor performance.
  build one struct, traverse char by index  

### 后记

项目缘起

本人做为从数学系转过来做开发的程序员，没有主修过《算法和数据结构》，一直想找个时
间补上这一课，趁这个阶段不忙，就动手了。

通过这段时间对算法的学习，我感到受益匪浅，特别是图的部分，跟现实问题联系更紧密，
感觉像打开了一扇窗，以此为基础可以做出许多有意义的项目。

任何编程语言都可以用来实现算法，常用的如Python，JavaScript，C++，我选择Rust
实现我这一遍的算法，我爱Rust，这是特别好的编程语言，特别是在除错方面，Rust编译
器能帮你非常多。当项目通过编译之后，你知道：80% OK了，可以放心的回家睡觉，养足
精神之后，元气满满的开始第二天的工作。

用Rust实现算法，一开始真的非常难 (如果你不想体验这种煎熬，我建议选择一种支持GC
的高级语言, Python或JavaScript, 开启你的算法之旅; 对于初学编程的同学，学会一
门编程语言之后，往往就不知道该干什么了，而写算法是很好的编程进阶训练)，当咬牙挺过
来之后，会对Rust有更深刻的理解，特别是能熟练运用unsafe和lifetimes标注；但是，
我感觉对Rust的学习是没有尽头的，当写的更多和读的更多之后，你总能预见有下一个高
度需要去攀登，花这些时间也是值得的，因为通过一番努力之后，代码会更好。Rust真的
是非常独特，没有哪种语言会给你这些很奇怪的感受。

关于Rust我还想多说一些，Rust社区为Rust准备的这组工具包，真是堪称豪华：单元测试，
性能测试，fmt, clippy，随着项目迭代，这些工具保证你无忧于重构和一致性，让你毫无
顾虑的大步前进; 同时希望: 这个小项目能为Rust的流行产生一些积极作用。

如果你对开发是严肃的，一定要把算法实现一遍，这就像圣徒的麦加圣地，是一定要去的。