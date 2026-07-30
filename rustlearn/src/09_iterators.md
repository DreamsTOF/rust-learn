# 09 迭代器

迭代器（Iterator）是 Rust 函数式编程的核心。与 Java 的 `Stream` 或 C++ 的 `std::ranges` 类似，Rust 的迭代器提供了一种声明式的数据处理方式——通过组合适配器（map、filter、fold 等）来表达数据转换，而非手写循环。Rust 的迭代器是**惰性**的，消费器（如 collect、sum）才会驱动计算。

---

### 练习 09-01: 调用 next() 手动遍历

> 难度：⭐⭐
> 类似 Java 的 Iterator.next()

创建一个数组 `[10, 20, 30, 40, 50]` 的迭代器，手动调用 `.next()` 方法遍历前三个元素并打印。

```rust
// TODO: 补全代码，使用 iter() 创建迭代器，手动调用 next() 三次
fn main() {
    let arr = [10, 20, 30, 40, 50];
    // 创建迭代器
    let mut iter = arr.iter();
    
    // 手动调用 next() 获取前三个元素并打印
    // 每个元素用 println! 打印
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let arr = [10, 20, 30, 40, 50];
    let mut iter = arr.iter();
    
    if let Some(val) = iter.next() {
        println!("第一个: {}", val);
    }
    if let Some(val) = iter.next() {
        println!("第二个: {}", val);
    }
    if let Some(val) = iter.next() {
        println!("第三个: {}", val);
    }
}
```

**说明：** `iter()` 返回数组的不可变引用迭代器，每次调用 `next()` 返回 `Option<&T>`。迭代器是**惰性**的——`next()` 是驱动它的基本消费方法。手动调用 `next()` 在 Rust 中不常见（通常用 for 循环），但理解其机制是掌握迭代器的基础。
</details>

---

### 练习 09-02: next() 与模式匹配

> 难度：⭐⭐
> 类似 Java 的 hasNext() / next() 检查

给定一个字符串切片数组 `["苹果", "香蕉", "樱桃", "榴莲"]`，使用迭代器配合 `while let` 模式匹配，遍历并打印所有元素及序号。

```rust
// TODO: 使用 while let 配合 next() 遍历数组，打印 "序号: 水果名"
fn main() {
    let fruits = ["苹果", "香蕉", "樱桃", "榴莲"];
    // 使用 while let 模式匹配遍历
    
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let fruits = ["苹果", "香蕉", "樱桃", "榴莲"];
    let mut iter = fruits.iter();
    let mut i = 1;
    while let Some(fruit) = iter.next() {
        println!("{}: {}", i, fruit);
        i += 1;
    }
}
```

**说明：** `while let Some(val) = iter.next()` 是标准的手动遍历模式，等价于 for 循环的底层实现。`next()` 返回 `None` 时循环终止。注意 `fruits.iter()` 返回 `Iter<'_, &str>`，其 `Item` 类型是 `&&str`，但由于自动解引用，打印时看起来和 `&str` 一样。
</details>

---

### 练习 09-03: 补全 for 循环遍历

> 难度：⭐⭐
> 类似 Java 的增强 for 循环 / C++ 的 range-based for

补全下面的代码，使用 `for` 循环遍历区间 `1..=5`，计算所有整数的和。

```rust
// TODO: 使用 for 循环遍历 1..=5，计算并打印总和
fn main() {
    // range 本身就是迭代器
    let mut sum = 0;
    // 在这里补全
    
    println!("总和: {}", sum); // 预期输出 15
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let mut sum = 0;
    for i in 1..=5 {
        sum += i;
    }
    println!("总和: {}", sum);
}
```

**说明：** `1..=5` 是 `RangeInclusive<i32>` 类型，它实现了 `Iterator` trait，因此可以直接用于 `for` 循环。`for` 循环本质上是 `into_iter()` + `while let` 的语法糖。`Range` 和 `RangeInclusive` 是最常见的数值迭代器之一。
</details>

---

### 练习 09-04: 补全 next() 返回 Option

> 难度：⭐⭐
> 理解 Option<&T> 的返回值

编写一个函数 `peek_first`，接收一个 `&[i32]` 切片，使用迭代器的 `next()` 方法返回第一个元素（如果存在）。请补全函数体。

```rust
// TODO: 补全函数，返回切片的第一个元素（如果存在）
fn peek_first(slice: &[i32]) -> Option<&i32> {
    // 创建迭代器并调用 next()
}

fn main() {
    let arr = [5, 10, 15];
    match peek_first(&arr) {
        Some(val) => println!("第一个元素: {}", val),
        None => println!("切片为空"),
    }
    
    let empty: [i32; 0] = [];
    match peek_first(&empty) {
        Some(val) => println!("第一个元素: {}", val),
        None => println!("切片为空"),
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn peek_first(slice: &[i32]) -> Option<&i32> {
    let mut iter = slice.iter();
    iter.next()
}

fn main() {
    let arr = [5, 10, 15];
    match peek_first(&arr) {
        Some(val) => println!("第一个元素: {}", val),
        None => println!("切片为空"),
    }
    
    let empty: [i32; 0] = [];
    match peek_first(&empty) {
        Some(val) => println!("第一个元素: {}", val),
        None => println!("切片为空"),
    }
}
```

**说明：** `iter.next()` 返回 `Option<&T>`，这正是函数签名需要的类型。对空切片调用 `iter()` 得到的迭代器第一次 `next()` 就返回 `None`。这种通过组合已有迭代器方法来简化代码的模式在 Rust 中非常常见——比起手写索引边界检查，`.iter().next()` 更加安全且表达力更强。
</details>

---

### 练习 09-05: 实现一个简单的 Counter 迭代器

> 难度：⭐⭐
> 挑战：自定义迭代器

实现一个 `Counter` 结构体，从 `start` 计数到 `end`（包含两端）。需要手动实现 `Iterator` trait。请补全代码。

```rust
struct Counter {
    current: i32,
    end: i32,
}

// TODO: 为 Counter 实现 Iterator trait
// Item 类型为 i32
// 当 current <= end 时，返回 Some(current) 并自增
// 否则返回 None

fn main() {
    let mut c = Counter { current: 1, end: 5 };
    // 使用 for 循环打印 1 到 5
    for val in c {
        println!("{}", val);
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Counter {
    current: i32,
    end: i32,
}

impl Iterator for Counter {
    type Item = i32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current <= self.end {
            let val = self.current;
            self.current += 1;
            Some(val)
        } else {
            None
        }
    }
}

fn main() {
    let c = Counter { current: 1, end: 5 };
    for val in c {
        println!("{}", val);
    }
}
```

**说明：** 实现 `Iterator` trait 只需要定义 `type Item` 和 `next()` 方法。`next()` 返回 `Option<Self::Item>`——有值时返回 `Some(val)`，遍历完毕返回 `None`。注意 `for` 循环会消耗迭代器（调用 `into_iter()`），因为 `Counter` 本身实现了 `Iterator`，所以可以直接用在 `for` 中。一旦 `next()` 返回 `None`，`for` 循环就结束了。
</details>

---

### 练习 09-06: map 将元素翻倍

> 难度：⭐⭐
> 类似 Java 的 stream().map()

使用 `map` 适配器将数组 `[1, 2, 3, 4, 5]` 中的每个元素翻倍，然后 `collect` 到 `Vec<i32>` 中。

```rust
// TODO: 使用 map 将数组元素翻倍，collect 到 Vec 中
fn main() {
    let arr = [1, 2, 3, 4, 5];
    
    // 补全：arr.iter() -> map(翻倍) -> collect
    let doubled: Vec<i32> = arr.iter().map(|x| /* TODO */).collect();
    
    println!("{:?}", doubled); // 预期输出 [2, 4, 6, 8, 10]
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];
    let doubled: Vec<i32> = arr.iter().map(|x| x * 2).collect();
    println!("{:?}", doubled);
}
```

**说明：** `map` 接受一个闭包，对迭代器的每个元素执行转换，返回一个新的迭代器。注意 `arr.iter()` 产生 `&i32`，所以闭包参数 `x` 是 `&i32`，`x * 2` 利用了自动解引用。`.collect()` 是消费器，它将迭代器收集到集合类型中。输出是 `[2, 4, 6, 8, 10]`。
</details>

---

### 练习 09-07: filter 过滤偶数

> 难度：⭐⭐
> 类似 Java 的 stream().filter()

使用 `filter` 适配器从一个整数向量中选出所有偶数，然后 `collect` 到新的 `Vec<i32>`。

```rust
// TODO: 使用 filter 过滤出偶数
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 补全：numbers.into_iter() -> filter(偶数判断) -> collect
    let evens: Vec<i32> = numbers.into_iter().filter(|x| /* TODO */).collect();
    
    println!("{:?}", evens); // 预期输出 [2, 4, 6, 8, 10]
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let evens: Vec<i32> = numbers.into_iter().filter(|x| x % 2 == 0).collect();
    println!("{:?}", evens);
}
```

**说明：** `filter` 接受一个返回 `bool` 的闭包，保留返回 `true` 的元素。这里使用 `into_iter()` 消费 `Vec` 的所有权，所以闭包参数 `x` 是 `i32`（而非 `&i32`）。链式调用 `filter` 后加 `collect` 是 Rust 中最常见的函数式数据过滤模式。
</details>

---

### 练习 09-08: fold 计算阶乘

> 难度：⭐⭐
> 类似 Java 的 stream().reduce() / C++ 的 std::accumulate

使用 `fold` 适配器计算 `5!`（阶乘），即 `1 × 2 × 3 × 4 × 5`。`fold` 接受一个初始值和闭包，闭包接收当前累加器和元素。

```rust
// TODO: 使用 fold 计算 1..=5 的乘积
fn main() {
    // 补全：1..=5 -> fold(初始值, 累加闭包)
    let factorial = (1..=5).fold(/* TODO */);
    
    println!("5! = {}", factorial); // 预期输出 120
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let factorial = (1..=5).fold(1, |acc, x| acc * x);
    println!("5! = {}", factorial);
}
```

**说明：** `fold(init, closure)` 是迭代器最强大的消费器之一。`init` 是初始累加值，闭包的第一个参数 `acc` 是当前累加值，第二个参数 `x` 是当前元素。整个过程等价于：`(((1 * 1) * 2) * 3) * 4 * 5 = 120`。`fold` 可以替代很多手动循环的场景，且表达更清晰。
</details>

---

### 练习 09-09: enumerate 带索引遍历

> 难度：⭐⭐
> 类似 Java 的 IntStream.range() 配合索引

使用 `enumerate` 适配器为数组 `["甲", "乙", "丙", "丁"]` 的元素生成带索引的输出，格式为 `"索引: 值"`。

```rust
// TODO: 使用 enumerate 打印带索引的元素
fn main() {
    let items = ["甲", "乙", "丙", "丁"];
    
    // 补全：items.iter() -> enumerate() -> for 循环打印
    for (i, item) in items.iter().enumerate() {
        println!("{}: {}", /* TODO */);
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let items = ["甲", "乙", "丙", "丁"];
    for (i, item) in items.iter().enumerate() {
        println!("{}: {}", i, item);
    }
}
```

**说明：** `enumerate()` 将迭代器包装为产生 `(usize, &T)` 元组的迭代器，`usize` 从 0 开始自动递增。它等价于手动维护一个计数器，但更安全、更简洁。注意 `enumerate` 在 `iter()` 之后调用——顺序是先有元素，再附上索引。
</details>

---

### 练习 09-10: zip 合并两个迭代器

> 难度：⭐⭐
> 类似 Python 的 zip() / Java 的 Streams.zip()

有两个等长数组 `["一", "二", "三"]` 和 `["one", "two", "three"]`，使用 `zip` 将它们合并成中英文对照的元组序列并打印。

```rust
// TODO: 使用 zip 合并中英文数组
fn main() {
    let chinese = ["一", "二", "三"];
    let english = ["one", "two", "three"];
    
    // 补全：chinese.iter() -> zip(english.iter()) -> for 循环打印
    for (cn, en) in /* TODO */ {
        println!("{} = {}", cn, en);
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let chinese = ["一", "二", "三"];
    let english = ["one", "two", "three"];
    for (cn, en) in chinese.iter().zip(english.iter()) {
        println!("{} = {}", cn, en);
    }
}
```

**说明：** `zip` 将两个迭代器合并为一个产生元组的迭代器。生成的迭代器在任一底层迭代器耗尽时结束——因此可用于不等长迭代器，多余的尾部元素会被丢弃。`zip` 是处理并行数据的利器，常用于合并关联信息。
</details>

---

### 练习 09-11: collect 收集为不同容器

> 难度：⭐⭐
> 类似 Java 的 Collectors.toList() / toSet()

将 `1..=10` 中的偶数通过链式调用 `filter` + `collect` 分别收集到 `Vec<i32>` 和 `std::collections::HashSet<i32>` 中。

```rust
use std::collections::HashSet;

// TODO: 补全 collect 的目标类型
fn main() {
    // 收集到 Vec
    let evens_vec: /* TODO: 类型 */ = (1..=10).filter(|x| x % 2 == 0).collect();
    
    // 收集到 HashSet
    let evens_set: /* TODO: 类型 */ = (1..=10).filter(|x| x % 2 == 0).collect();
    
    println!("Vec: {:?}", evens_vec);
    println!("HashSet: {:?}", evens_set);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::HashSet;

fn main() {
    let evens_vec: Vec<i32> = (1..=10).filter(|x| x % 2 == 0).collect();
    let evens_set: HashSet<i32> = (1..=10).filter(|x| x % 2 == 0).collect();
    
    println!("Vec: {:?}", evens_vec);
    println!("HashSet: {:?}", evens_set);
}
```

**说明：** `collect()` 可以从同一个迭代器链收集到不同的容器类型。编译器通过类型推断（变量类型标注）来确定目标容器。Rust 的标准库为 `Vec`、`HashSet`、`String`、`HashMap` 等常见集合都实现了 `FromIterator`，使得 `collect` 非常灵活。</details>

---

### 练习 09-12: 链式调用——map + filter + collect

> 难度：⭐⭐
> 类似 Java 的 stream().map().filter().collect()

有一个字符串数组 `["  hello  ", " world ", "", "  Rust  ", ""]`，请链式调用迭代器适配器：去除空格（`trim`）、过滤掉空字符串、然后将结果收集到 `Vec<String>`。

```rust
// TODO: 链式调用 map + filter + collect
fn main() {
    let words = ["  hello  ", " world ", "", "  Rust  ", ""];
    
    // 补全：words.iter() -> map(trim) -> filter(非空) -> map(转为String) -> collect
    let cleaned: Vec<String> = words.iter()
        /* TODO */
        .collect();
    
    println!("{:?}", cleaned); // 预期输出 ["hello", "world", "Rust"]
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let words = ["  hello  ", " world ", "", "  Rust  ", ""];
    let cleaned: Vec<String> = words.iter()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
    
    println!("{:?}", cleaned);
}
```

**说明：** 这是典型的函数式链式调用：`map` 转换数据（去除空格），`filter` 筛选数据（去空），再次 `map` 转换类型（`&str` → `String`），最后 `collect` 收集。Rust 的迭代器链是惰性的——每个适配器只构建一个新的迭代器，直到 `collect` 消费时才真正执行。这种风格比嵌套循环更易读、更安全。
</details>

---

### 练习 09-13: count / sum / min / max 消费器

> 难度：⭐⭐
> 类似 Java 的 Stream.count() / min() / max()

给定一个整数切片 `[8, 3, 9, 1, 6, 4]`，使用迭代器消费器分别计算：元素个数、总和、最小值、最大值。

```rust
// TODO: 使用 count / sum / min / max 消费器
fn main() {
    let numbers = [8, 3, 9, 1, 6, 4];
    
    // 补全
    let count = numbers.iter()/* TODO */.count();
    let sum: i32 = numbers.iter()/* TODO */.sum();
    let min = numbers.iter()/* TODO */.min();
    let max = numbers.iter()/* TODO */.max();
    
    println!("count: {}, sum: {}", count, sum);
    println!("min: {:?}, max: {:?}", min, max);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let numbers = [8, 3, 9, 1, 6, 4];
    
    let count = numbers.iter().count();
    let sum: i32 = numbers.iter().sum();
    let min = numbers.iter().min();
    let max = numbers.iter().max();
    
    println!("count: {}, sum: {}", count, sum);
    println!("min: {:?}, max: {:?}", min, max);
}
```

**说明：** `count()` 返回 `usize`，`sum()` 返回数值（需要类型标注），`min()` 和 `max()` 返回 `Option<&T>`（因为空迭代器没有最小/最大值）。这些消费器都会消耗迭代器。注意 `min` 和 `max` 返回 `Option`，所以打印时用 `{:?}`。
</details>

---

### 练习 09-14: any / all 条件检查

> 难度：⭐⭐
> 类似 Java 的 Stream.anyMatch() / allMatch()

给定一个整数数组 `[-3, 5, -1, 7, -2]`，使用 `any` 检查是否存在负数，使用 `all` 检查是否所有数都为正数。

```rust
// TODO: 使用 any 和 all 消费器
fn main() {
    let numbers = [-3, 5, -1, 7, -2];
    
    // 补全
    let has_negative = numbers.iter()/* TODO */;
    let all_positive = numbers.iter()/* TODO */;
    
    println!("存在负数: {}", has_negative);
    println!("全部为正: {}", all_positive);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let numbers = [-3, 5, -1, 7, -2];
    
    let has_negative = numbers.iter().any(|&x| x < 0);
    let all_positive = numbers.iter().all(|&x| x > 0);
    
    println!("存在负数: {}", has_negative);
    println!("全部为正: {}", all_positive);
}
```

**说明：** `any` 和 `all` 都是短路求值的消费器——`any` 在遇到第一个 `true` 时停止遍历，`all` 在遇到第一个 `false` 时停止遍历。它们返回 `bool`。闭包中 `|&x| x < 0` 中的 `&` 对 `&i32` 进行解构，使得闭包体内直接使用 `i32` 值。
</details>

---

### 练习 09-15: find / position 查找元素

> 难度：⭐⭐
> 类似 Java 的 Stream.findFirst()

给定一个数组 `["Rust", "Java", "Python", "C++", "Go"]`，使用 `find` 查找第一个长度 > 3 的语言名，使用 `position` 查找第一个以 "P" 开头的元素索引。

```rust
// TODO: 使用 find 和 position
fn main() {
    let langs = ["Rust", "Java", "Python", "C++", "Go"];
    
    // 补全
    let first_long = langs.iter()/* TODO: find 长度 > 3 */;
    let first_p_pos = langs.iter()/* TODO: position 以 "P" 开头 */;
    
    println!("第一个长度 > 3 的语言: {:?}", first_long);
    println!("第一个以 P 开头的索引: {:?}", first_p_pos);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let langs = ["Rust", "Java", "Python", "C++", "Go"];
    
    let first_long = langs.iter().find(|s| s.len() > 3);
    let first_p_pos = langs.iter().position(|s| s.starts_with("P"));
    
    println!("第一个长度 > 3 的语言: {:?}", first_long);
    println!("第一个以 P 开头的索引: {:?}", first_p_pos);
}
```

**说明：** `find` 接收闭包，返回第一个满足条件的元素的引用 `Option<&T>`。`position` 返回第一个满足条件的元素索引 `Option<usize>`。两者都是短路求值——找到匹配项即停止遍历。注意 `find` 返回 `Option<&&str>`（因为迭代 `&[&str]`），打印时 `{:?}` 会显示 `Some("Python")`。
</details>

---

### 练习 09-16: 实现 Iterator——StepIterator

> 难度：⭐⭐⭐
> 自定义迭代器

实现一个 `StepIterator`，它在给定的起始值和结束值之间以固定步长递增（不含结束值）。例如 `StepIterator::new(0, 10, 3)` 应产生 `0, 3, 6, 9`。

```rust
struct StepIterator {
    current: i32,
    end: i32,
    step: i32,
}

impl StepIterator {
    fn new(start: i32, end: i32, step: i32) -> Self {
        StepIterator { current: start, end, step }
    }
}

// TODO: 为 StepIterator 实现 Iterator trait

fn main() {
    let iter = StepIterator::new(0, 10, 3);
    let result: Vec<i32> = iter.collect();
    println!("{:?}", result); // 预期输出 [0, 3, 6, 9]
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct StepIterator {
    current: i32,
    end: i32,
    step: i32,
}

impl StepIterator {
    fn new(start: i32, end: i32, step: i32) -> Self {
        StepIterator { current: start, end, step }
    }
}

impl Iterator for StepIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let val = self.current;
            self.current += self.step;
            Some(val)
        } else {
            None
        }
    }
}

fn main() {
    let iter = StepIterator::new(0, 10, 3);
    let result: Vec<i32> = iter.collect();
    println!("{:?}", result);
}
```

**说明：** 自定义迭代器的核心是正确实现 `next()` 方法：维护内部状态，每次调用返回当前值并推进，在到达终点时返回 `None`。一旦 `StepIterator` 实现了 `Iterator`，它就能使用所有标准适配器（`map`、`filter`、`collect` 等）。注意 `collect` 会重复调用 `next()` 直到返回 `None`。
</details>

---

### 练习 09-17: 实现 Iterator——Fibonacci

> 难度：⭐⭐⭐
> 自定义无限迭代器

实现一个 `Fibonacci` 迭代器，生成斐波那契数列（0, 1, 1, 2, 3, 5, 8, 13...）。这是一个**无限迭代器**，永远不返回 `None`。

```rust
struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

// TODO: 为 Fibonacci 实现 Iterator trait

fn main() {
    // 取前 10 个斐波那契数
    let first_ten: Vec<u64> = Fibonacci::new()/* TODO */.take(10).collect();
    println!("{:?}", first_ten); // 预期输出 [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.a;
        self.a = self.b;
        self.b = current + self.b;
        Some(current)
    }
}

fn main() {
    let first_ten: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("{:?}", first_ten);
}
```

**说明：** 无限迭代器（`next()` 始终返回 `Some`）需要用 `take(n)` 限制取值的数量。`take` 本身也是一个适配器，它包装原迭代器，在产生 `n` 个元素后返回 `None`。斐波那契迭代器维护两个状态变量 `a` 和 `b`，每次迭代更新它们。`u64` 在 94 次迭代后溢出，实际应用可改用 `u128` 或 `BigUint`。
</details>

---

### 练习 09-18: 实现 Iterator——CyclicSlice

> 难度：⭐⭐⭐
> 自定义循环迭代器

实现一个 `CyclicSlice` 迭代器，它接收一个切片引用，并无限循环遍历其元素。即遍历到末尾后回到开头重新开始。

```rust
struct CyclicSlice<'a> {
    data: &'a [i32],
    index: usize,
}

impl<'a> CyclicSlice<'a> {
    fn new(data: &'a [i32]) -> Self {
        CyclicSlice { data, index: 0 }
    }
}

// TODO: 为 CyclicSlice 实现 Iterator trait
// 提示：始终返回 Some(&i32)，index 到达末尾后回绕

fn main() {
    let arr = [1, 2, 3];
    let result: Vec<&i32> = CyclicSlice::new(&arr).take(8).collect();
    println!("{:?}", result); // 预期输出 [1, 2, 3, 1, 2, 3, 1, 2]
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct CyclicSlice<'a> {
    data: &'a [i32],
    index: usize,
}

impl<'a> CyclicSlice<'a> {
    fn new(data: &'a [i32]) -> Self {
        CyclicSlice { data, index: 0 }
    }
}

impl<'a> Iterator for CyclicSlice<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() {
            return None;
        }
        let val = &self.data[self.index];
        self.index = (self.index + 1) % self.data.len();
        Some(val)
    }
}

fn main() {
    let arr = [1, 2, 3];
    let result: Vec<&i32> = CyclicSlice::new(&arr).take(8).collect();
    println!("{:?}", result);
}
```

**说明：** 周期循环迭代器通过取模运算实现索引回绕。注意空切片的边界情况——直接对空切片取模会 panic，所以需要提前处理。生命周期 `'a` 确保返回的引用不会超过底层数据的生命周期。这里返回 `&'a i32` 而非 `&i32`，是因为 `next` 返回的元素引用来自迭代器之外的源数据。
</details>

---

### 练习 09-19: 实现 Iterator——GroupIterator

> 难度：⭐⭐⭐
> 分组迭代器

实现一个 `GroupIterator`，将底层迭代器的元素按固定大小分组。例如输入 `[1, 2, 3, 4, 5, 6, 7]`，组大小 3，应产生 `[1, 2, 3]`、`[4, 5, 6]`、`[7]`。

```rust
struct GroupIterator<I> {
    iter: I,
    size: usize,
}

// TODO: 为 GroupIterator<I> 实现 Iterator trait
// 要求 Item = Vec<I::Item>，每次从底层迭代器取 size 个元素
// 提示：使用 .by_ref().take(size) 或手动循环 collect

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7];
    let groups: Vec<Vec<i32>> = GroupIterator { iter: data.into_iter(), size: 3 }.collect();
    println!("{:?}", groups); // 预期输出 [[1, 2, 3], [4, 5, 6], [7]]
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct GroupIterator<I> {
    iter: I,
    size: usize,
}

impl<I: Iterator> Iterator for GroupIterator<I> {
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut group = Vec::with_capacity(self.size);
        for _ in 0..self.size {
            match self.iter.next() {
                Some(val) => group.push(val),
                None => break,
            }
        }
        if group.is_empty() { None } else { Some(group) }
    }
}

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7];
    let groups: Vec<Vec<i32>> = GroupIterator { iter: data.into_iter(), size: 3 }.collect();
    println!("{:?}", groups);
}
```

**说明：** `GroupIterator` 包装一个泛型迭代器 `I: Iterator`，其 `Item = Vec<I::Item>`——每组是一个向量。`next()` 从底层迭代器取 `size` 个元素，如果不足则取剩余所有。当底层迭代器已经耗尽且没有取到任何元素时返回 `None`。泛型约束 `I: Iterator` 使得 `GroupIterator` 可以包装任何迭代器，体现了组合性。
</details>

---

### 练习 09-20: 实现 Iterator——交错合并

> 难度：⭐⭐⭐
> 自定义复杂迭代器

实现一个 `Interleave` 迭代器，它接收两个相同类型的迭代器，交替从它们中取值。例如 `Interleave([1,3,5], [2,4,6])` 产生 `1, 2, 3, 4, 5, 6`。

```rust
struct Interleave<I, J> {
    a: I,
    b: J,
    turn_a: bool, // true 时从 a 取，false 时从 b 取
}

// TODO: 为 Interleave 实现 Iterator trait
// 交替从 a 和 b 中取元素，其中一个耗尽后继续从另一个取

fn main() {
    let a = vec![1, 3, 5, 7];
    let b = vec![2, 4, 6];
    let interleaved: Vec<i32> = (Interleave {
        a: a.into_iter(),
        b: b.into_iter(),
        turn_a: true,
    }).collect();
    println!("{:?}", interleaved); // 预期输出 [1, 2, 3, 4, 5, 6, 7]
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Interleave<I, J> {
    a: I,
    b: J,
    turn_a: bool,
}

impl<I: Iterator, J: Iterator<Item = I::Item>> Iterator for Interleave<I, J> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.turn_a {
            match self.a.next() {
                Some(val) => {
                    self.turn_a = false;
                    return Some(val);
                }
                None => {
                    // a 已耗尽，转从 b 继续取
                    return self.b.next();
                }
            }
        } else {
            match self.b.next() {
                Some(val) => {
                    self.turn_a = true;
                    return Some(val);
                }
                None => {
                    // b 已耗尽，转从 a 继续取
                    return self.a.next();
                }
            }
        }
    }
}

fn main() {
    let a = vec![1, 3, 5, 7];
    let b = vec![2, 4, 6];
    let interleaved: Vec<i32> = (Interleave {
        a: a.into_iter(),
        b: b.into_iter(),
        turn_a: true,
    }).collect();
    println!("{:?}", interleaved);
}
```

**说明：** `Interleave` 使用布尔标志 `turn_a` 控制轮流从两个迭代器取数。当一个迭代器耗尽后，剩余元素全部从另一个获取。注意约束 `J: Iterator<Item = I::Item>` 确保两个迭代器的元素类型一致。这个迭代器展示了如何在自定义实现中处理多个内部迭代器的状态切换。
</details>

---

### 练习 09-21: IntoIterator——数组的三种迭代形式

> 难度：⭐⭐⭐
> 类似 Java 的 `Iterable<T>`

Rust 中 `for` 循环本质上是调用 `into_iter()`。数组、`Vec` 和切片实现了不同的 `IntoIterator`。请观察并补全下面代码，理解 `iter()`、`iter_mut()` 和 `into_iter()` 的区别。

```rust
// TODO: 补全代码，分别使用 iter()、iter_mut()、into_iter() 处理数组
fn main() {
    let mut arr = [1, 2, 3];
    
    // 使用 iter() —— 不可变引用
    for val in arr.iter() {
        println!("iter: {}", val);
        // 这里的 val 类型是 &i32
        // 可以读取但不能修改
    }
    
    // 使用 iter_mut() —— 可变引用
    for val in arr.iter_mut() {
        *val *= 10; // 每个元素乘以 10
    }
    println!("修改后: {:?}", arr); // 预期输出 [10, 20, 30]
    
    // TODO: 使用 into_iter() —— 获取所有权
    // 注意：这里 arr 会被消耗吗？试试看
    for val in /* TODO */ {
        println!("into_iter: {}", val);
    }
    
    // 下面的代码会编译通过吗？
    // println!("arr: {:?}", arr);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let mut arr = [1, 2, 3];
    
    for val in arr.iter() {
        println!("iter: {}", val);
    }
    
    for val in arr.iter_mut() {
        *val *= 10;
    }
    println!("修改后: {:?}", arr);
    
    // into_iter() 对数组会获取元素所有权（复制）
    for val in arr.into_iter() {
        println!("into_iter: {}", val);
    }
    
    // 注意：arr 仍然可用！因为 [i32; 3] 实现了 Copy
    // 但如果元素类型不是 Copy，arr 将被消耗
    println!("arr 仍然可用: {:?}", arr);
}
```

**说明：** `IntoIterator` trait 定义了 `.into_iter()` 方法，`for` 循环会隐式调用它。对于数组 `[T; N]`：
- `arr.iter()` → 返回 `Iter<T>`，产生 `&T`
- `arr.iter_mut()` → 返回 `IterMut<T>`，产生 `&mut T`
- `arr.into_iter()` → 消耗数组（但 `[i32; 3]` 实现了 `Copy` 因此数组仍可用）
对于 `Vec<T>`，`into_iter()` 会消耗 `Vec` 并拥有元素所有权。理解这三种迭代方式对于编写正确的 Rust 代码至关重要。
</details>

---

### 练习 09-22: IntoIterator——自定义结构体

> 难度：⭐⭐⭐
> 类似 Java 的 implements `Iterable<T>`

为自定义结构体 `Library` 实现 `IntoIterator`，使其可以用 `for` 循环直接遍历内部的书名列表。

```rust
struct Library {
    books: Vec<String>,
}

impl Library {
    fn new(books: Vec<String>) -> Self {
        Library { books }
    }
}

// TODO: 为 Library 实现 IntoIterator trait
// 提示：可以直接委托给 books.into_iter()

fn main() {
    let lib = Library::new(vec![
        "Rust 编程".to_string(),
        "算法导论".to_string(),
        "设计模式".to_string(),
    ]);
    
    // 补全后这行应该能编译
    for book in lib {
        println!("{}", book);
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Library {
    books: Vec<String>,
}

impl Library {
    fn new(books: Vec<String>) -> Self {
        Library { books }
    }
}

impl IntoIterator for Library {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.books.into_iter()
    }
}

fn main() {
    let lib = Library::new(vec![
        "Rust 编程".to_string(),
        "算法导论".to_string(),
        "设计模式".to_string(),
    ]);
    
    for book in lib {
        println!("{}", book);
    }
}
```

**说明：** 实现 `IntoIterator` 需要定义三个关联项：`Item`（元素类型）、`IntoIter`（迭代器类型）和 `into_iter(self)` 方法。通常可以直接委托给内部集合的 `into_iter()`。注意 `IntoIterator` 的 `into_iter` 接收 `self`（所有权），所以 `for book in lib` 会消耗 `lib`。如果需要不消耗所有权的迭代，可以为 `&Library` 实现 `IntoIterator`。
</details>

---

### 练习 09-23: IntoIterator——为引用实现

> 难度：⭐⭐⭐
> 实现 IntoIterator for &Library

为 `Library` 的引用 `&Library` 实现 `IntoIterator`，使得 `for book in &lib` 也能工作（产生 `&String`，不消耗所有权）。

```rust
struct Library {
    books: Vec<String>,
}

impl Library {
    fn new(books: Vec<String>) -> Self {
        Library { books }
    }
}

// TODO: 为 &Library 实现 IntoIterator
// 提示：使用 self.books.iter()

fn main() {
    let lib = Library::new(vec![
        "书 A".to_string(),
        "书 B".to_string(),
        "书 C".to_string(),
    ]);
    
    // 补全后这行应该能编译
    for book in &lib {
        println!("{}", book);
    }
    
    // lib 所有权仍在，可以再遍历
    for book in &lib {
        println!("再次: {}", book);
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct Library {
    books: Vec<String>,
}

impl Library {
    fn new(books: Vec<String>) -> Self {
        Library { books }
    }
}

impl<'a> IntoIterator for &'a Library {
    type Item = &'a String;
    type IntoIter = std::slice::Iter<'a, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.books.iter()
    }
}

fn main() {
    let lib = Library::new(vec![
        "书 A".to_string(),
        "书 B".to_string(),
        "书 C".to_string(),
    ]);
    
    for book in &lib {
        println!("{}", book);
    }
    
    for book in &lib {
        println!("再次: {}", book);
    }
}
```

**说明：** 为引用类型实现 `IntoIterator` 允许 `for book in &lib` 语法正常工作。这里 `self` 的类型是 `&'a Library`，`self.books.iter()` 返回 `Iter<'a, String>`，产生 `&String`。这种模式使调用者可以灵活选择是否消耗所有权——`for book in lib` 消耗，`for book in &lib` 借用。这是 Rust 中常见的 API 设计模式。
</details>

---

### 练习 09-24: 综合——多步骤数据流水线

> 难度：⭐⭐⭐
> 综合运用迭代器适配器

有一个学生成绩的向量 `[(name, score)]`，请通过链式调用完成以下处理：1) 过滤掉不及格（< 60）的 2) 按分数降序排列 3) 取前 3 名 4) 收集为 `Vec<&str>`（只保留姓名）。

```rust
// TODO: 链式调用来处理成绩数据
fn main() {
    let scores = vec![
        ("张三", 85),
        ("李四", 42),
        ("王五", 93),
        ("赵六", 67),
        ("钱七", 78),
        ("孙八", 55),
    ];
    
    // 补全链式调用
    let top3: Vec<&str> = scores.iter()
        /* TODO */
        .collect();
    
    println!("前三名: {:?}", top3); // 预期输出 ["王五", "张三", "钱七"]
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let scores = vec![
        ("张三", 85),
        ("李四", 42),
        ("王五", 93),
        ("赵六", 67),
        ("钱七", 78),
        ("孙八", 55),
    ];
    
    let top3: Vec<&str> = scores.iter()
        .filter(|(_, score)| *score >= 60)
        .sorted_by(|a, b| b.1.cmp(&a.1)) // 降序
        .take(3)
        .map(|(name, _)| *name)
        .collect();
    
    println!("前三名: {:?}", top3);
}
```

**说明：** 这道题综合了 `filter`、`sorted_by`（需要 `use itertools::Itertools` 或改用 `Vec::sort_by`）、`take` 和 `map`。注意 `sorted_by` 不是标准库方法——标准方式是将结果收集到 `Vec` 再排序。更符合标准库风格的写法是：

```rust
let mut passed: Vec<_> = scores.iter().filter(|(_, s)| *s >= 60).collect();
passed.sort_by(|a, b| b.1.cmp(&a.1));
let top3: Vec<&str> = passed.into_iter().take(3).map(|(n, _)| *n).collect();
```

这里 `sorted_by` 来自 `itertools` crate，为展示迭代器链式调用的理想形式而使用。实际项目中，在 `collect` 之后排序再取前 N 是更常见的模式。
</details>

---

### 练习 09-25: 综合挑战——自定义 IntoIterator + 迭代器适配器

> 难度：⭐⭐⭐
> 综合：自定义类型 + IntoIterator + 链式调用

有一个 `Deck` 结构体表示一副扑克牌（52 张）。请补全 `Card` 结构体和 `IntoIterator for &Deck` 的实现，然后链式调用迭代器方法：选出所有红心花色（Suit::Heart）的牌，按点数排序，取前 5 张。

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Suit {
    Spade,   // 黑桃
    Heart,   // 红心
    Club,    // 梅花
    Diamond, // 方块
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Card {
    suit: Suit,
    rank: i32, // 1(A) 到 13(K)
}

impl Card {
    fn new(suit: Suit, rank: i32) -> Self {
        Card { suit, rank }
    }
}

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Self {
        let mut cards = Vec::with_capacity(52);
        for suit in &[Suit::Spade, Suit::Heart, Suit::Club, Suit::Diamond] {
            for rank in 1..=13 {
                cards.push(Card::new(*suit, rank));
            }
        }
        Deck { cards }
    }
}

// TODO: 为 &Deck 实现 IntoIterator
// 提示：委托给 self.cards.iter()

fn main() {
    let deck = Deck::new();
    
    // TODO: 选出所有红心牌，按点数排序，取前 5 张
    let hearts: Vec<&Card> = (&deck)
        /* TODO */
        .collect();
    
    println!("红心前 5 张:");
    for card in &hearts {
        println!("  {:?} {}", card.suit, card.rank);
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Suit {
    Spade, Heart, Club, Diamond,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Card {
    suit: Suit,
    rank: i32,
}

impl Card {
    fn new(suit: Suit, rank: i32) -> Self {
        Card { suit, rank }
    }
}

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Self {
        let mut cards = Vec::with_capacity(52);
        for suit in &[Suit::Spade, Suit::Heart, Suit::Club, Suit::Diamond] {
            for rank in 1..=13 {
                cards.push(Card::new(*suit, rank));
            }
        }
        Deck { cards }
    }
}

impl<'a> IntoIterator for &'a Deck {
    type Item = &'a Card;
    type IntoIter = std::slice::Iter<'a, Card>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.iter()
    }
}

fn main() {
    let deck = Deck::new();
    
    let mut hearts: Vec<&Card> = (&deck)
        .into_iter()
        .filter(|c| c.suit == Suit::Heart)
        .collect();
    
    hearts.sort_by_key(|c| c.rank);
    let top5: Vec<&Card> = hearts.into_iter().take(5).collect();
    
    println!("红心前 5 张:");
    for card in &top5 {
        println!("  {:?} {}", card.suit, card.rank);
    }
}
```

**说明：** 这道综合题将本章的核心概念融汇在一起：1) 自定义迭代器——`for suit in &[Suit::...]` 利用了 `IntoIterator for &[T]`；2) `IntoIterator` 实现——`&Deck` 委托给 `self.cards.iter()`；3) 链式调用——`filter` 筛选花色；4) `collect` + 排序 + `take`——由于 `sorted_by_key` 不是标准库方法，先 `collect` 到 `Vec`，排序后 `into_iter().take(5)` 是更标准的做法。注意 `(&deck).into_iter()` 和 `(&deck).iter()` 不等价——这里显式写 `into_iter()` 来调用我们实现的 `IntoIterator`，实际上 `for` 循环也会隐式调用它。
</details>

---

恭喜完成第 09 章！迭代器是 Rust 函数式编程的基石——掌握它们能让你写出更安全、更简洁、更具表达力的代码。