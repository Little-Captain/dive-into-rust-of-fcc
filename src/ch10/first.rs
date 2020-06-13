// 不带自动内存回收(Garbage Collection)的内存安全是 Rust 语言最重要的创新，
// 是它与其他语言最主要的区别所在，是 Rust 语言设计的核心。

// 内存管理基础
// 堆和栈

// 一个进程在执行的时候，它所占用的内存的虚拟地址空间一般被分割成好几个区域，
// “区域”称为“段”(Segment)。常见的几个段如下：
// 1. 代码段。编译后的机器码存在的区域。一般这个段是只读的。
// 2. bss 段。存放未初始化的全局变量和静态变量的区域。
// 3. 数据段。存放有初始化的全局变量和静态变量的区域。
// 4. 函数调用栈(call stack segment)。
//    存放函数参数、局部变量以及其他函数调用相关信息的区域。
// 5. 堆(heap)。存放动态分配内存的区域。

// 函数调用栈(call stack)
// 也可以简称为栈(stack)。
// 因为函数调用栈本来就是基于栈这样一个数据结构实现的。
// 它具备“后入先出”(LIFO)的特点。最先进入的数据也是最后出来的数据。
// 一般来说，CPU 有专门的指令可以用于入栈和出栈操作。
// 当一个函数被调用时，就会有指令把当前指令的地址压入栈内保存起来，然后跳转到被调用的函数中执行。
// 函数返回的时候，就会把栈里面先前的指令地址弹出来继续执行

// 堆
// 动态分配的预留内存空间。
// 和栈不一样，从堆上分配和重新分配块没有固定模式，用户可以在任何时候分配和释放它。
// 这样就使得跟踪哪部分堆已经被分配和被释放变得异常复杂；
// 有许多定制的堆分配策略用来为不同的使用模式下调整堆的性能。
// 堆是在内存中动态分配的内存，是无序的。
// 每个线程都有一个栈，但是每一个应用程序通常都只有一个堆。
// 在堆上的变量必须要手动释放，不存在作用域的问题。

// 一般来说，操作系统提供了在堆上分配和释放内存的系统调用，
// 但是用户不是直接使用这个系统调用，而是使用封装的更好的“内存分配器”(Allocator)。
// 在 C 语言里面，运行时(runtime)就提供了 malloc 和 free 这两个函数来管理堆内存。

// 堆和栈之间的区别有：
// 1. 栈上保存的局部变量在退出当前作用域的时候会自动释放；
// 2. 堆上分配的空间没有作用域，需要手动释放；
// 3. 一般栈上分配的空间大小是编译阶段就可以确定的(C 语言里面的 VLA 除外)；
// 4. 栈有一个确定的最大长度，超过了这个长度会产生“栈溢出”(stack overflow);
// 5. 堆的空间一般要更大一些，堆上的内存耗尽了，就会产生“内存分配不足”(out of memory)。
pub fn first() {}

// 段错误 segfaults
// segfault 实际上是“segmentation fault”的缩写形式，即“段错误”。
// segfault 是这样形成的：进程空间中的每个段通过硬件 MMU 映射到真正的物理空间；
// 在这个映射过程中，可以给不同的段设置不同的访问权限，比如代码段就只能读不能写；
// 进程在执行过程中，如果违反了这些权限，CPU 会直接产生一个硬件异常；
// 硬件异常会被操作系统内核处理，一般内核会向对应的进程发送一条信号；
// 如果进程没有实现特殊的信号处理函数，默认情况下，这个进程会直接非正常退出；
// 如果操作系统打开了 core dump 功能，在进程退出的时候操作系统会把它当时的
// 内存状态、寄存器状态以及各种相关信息保存到一个文件中，供用户以后调试使用。

// 在传统系统级编程语言 C/C++ 里面，制造 segfault 是很容易的。
// 程序员需要非常小心才能避免这种错误，这也是为什么会有那么多的代码标准来规范程序员的行为。
// 而另外一类编程语言规避 segfault 的办法是使用自动垃圾回收机制。
// 在这些编程语言中，指针的能力被大幅限制，内存分配和释放都在一个运行时环境中被严格管理。
// 在一些应用场景下用这样的代价换取开发效率和安全性是非常划算的，
// 而在某些应用场景下这样的代价是不可接受的。

// Rust 的主要设计目标之一，就是在不用自动垃圾回收机制的前提下避免产生 segfault。
pub fn second() {}

// 常见的内存不安全的例子
// 1. 空指针
// 解引用空指针是不安全的。这块地址空间一般是受保护的，
// 对空指针解引用在大部分平台上会产生 segfault。
// 2. 野指针
// 野指针指的是未初始化的指针。它的值取决于它这个位置以前遗留下来的是什么值。
// 所以它可能指向任意一个地方。对它解引用，可能会造成 segfault，也可能不会，
// 纯凭运气。无论如何，这个行为都不是预期行为，是一定会产生 bug 的。
// 3. 悬空指针
// 悬空指针指的是内存空间在被释放了之后，继续使用。
// 它跟野指针类似，同样会读写已经不属于这个指针的内容。
// 4. 使用未初始化内存
// 不只是指针类型，任何一种类型不初始化就直接使用都是危险的，
// 造成的后果完全无法预测。
// 5. 非法释放
// 内存分配和释放要配对。
// 如果对同一个指针释放两次，会制造出内存错误。
// 如果指针并不是内存分配器返回的值，对其执行释放操作，也是危险的。
// 6. 缓冲区溢出
// 指针访问越界了，结果也是类似于野指针，会读取或者修改临近内存空间的值，
// 造成危险。
// 7. 执行非法函数指针
// 如果一个函数指针不是准确地指向一个函数地址，那么调用这个函数指针会导致
// 一段随机数据被当成指令来执行，是非常危险的。
// 8. 数据竞争
// 在有并发的场景下，针对同一块内存同时读写，且没有同步措施。

// 在 Rust 中，有一些内存错误是不算在“内存安全”范畴内的，
// 比如内存泄漏以及内存耗尽。
// 内存泄漏显然是一种 bug，但是它不会直接造成非常严重的后果，
// 内存耗尽也不是事关安全性的问题，出现内存耗尽的时候，Rust 程序
// 的行为依然是确定性的和可控的

// 另外，panic 也不属于内存安全相关的问题。
// panic 和 core dump 之间有重要区别：
// panic 是发生不可恢复错误后，程序主动执行的一种错误处理机制；
// core dump 则是程序失控之后，触发了操作系统的保护机制而被动退出的。
// 发生 panic 的时候，此处就是确定性的第一现场，
// 可以根据 call stack 信息很快找到事发地点，然后修复。
// panic 是防止更严重 内存安全错误的重要机制。
pub fn third() {}
