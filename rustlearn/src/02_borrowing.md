# 02 借用与引用

借用（Borrowing）是 Rust 所有权体系中的核心概念，它允许你在不转移所有权的情况下访问数据。通过引用（Reference），你可以"借用"一个值而不获取它的所有权，从而在保持内存安全的同时实现灵活的代码复用。

---

### 练习 02-01: 创建不可变引用

> 难度：⭐
> 类似 C++ 的 const T&（不可变引用）

创建一个 `String` 变量，然后用 `&` 获取它的不可变引用，最后打印该引用指向的值。

```rust
fn main() {
    // TODO: 创建一个 String 变量 s，内容为 "Rust"
    // TODO: 创建 s 的不可变引用 r
    // TODO: 打印 r 的值
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let s = String::from("Rust");
    let r = &s;
    println!("{}", r);
}
```

**说明：** `&s` 创建了指向 `s` 的不可变引用 `r`，其类型为 `&String`。通过 `{}` 格式化占位符可以直接解引用并打印。不可变引用允许你读取数据，但不能修改它。
</details>

---

### 练习 02-02: 不可变引用与解引用

> 难度：⭐
> 类似 C++ 的 const T&（不可变引用）

创建一个整数变量，获取它的不可变引用，然后分别用引用本身和解引用两种方式打印。

```rust
fn main() {
    // TODO: 创建 i32 变量 x = 42
    // TODO: 创建 x 的不可变引用 ref_x
    // TODO: 分别打印 ref_x 和 *ref_x，观察区别
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let x = 42;
    let ref_x = &x;
    println!("ref_x 的地址: {:p}", ref_x);
    println!("*ref_x 的值: {}", *ref_x);
}
```

**说明：** `ref_x` 的类型是 `&i32`，直接打印会显示内存地址（`{:p}` 格式说明符）。`*ref_x` 是解引用操作，获取引用指向的原始值。`{}` 会自动解引用，所以 `println!("{}", ref_x)` 也会打印值而不是地址。
</details>

---

### 练习 02-03: 多个不可变引用

> 难度：⭐
> 类似 C++ 的多个 const T&

创建一个 `Vec<i32>` 向量，然后创建两个不可变引用，分别读取第一个和最后一个元素并打印。

```rust
fn main() {
    // TODO: 创建 Vec<i32>，内容为 [10, 20, 30, 40, 50]
    // TODO: 创建两个不可变引用 r1 和 r2
    // TODO: 通过 r1 打印第一个元素，通过 r2 打印最后一个元素
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let v = vec![10, 20, 30, 40, 50];
    let r1 = &v;
    let r2 = &v;
    println!("第一个元素: {}", r1[0]);
    println!("最后一个元素: {}", r2[4]);
}
```

**说明：** Rust 允许多个不可变引用同时存在，因为只读操作不会造成数据竞争。这里 `r1` 和 `r2` 都是对 `v` 的不可变引用，可以同时使用。
</details>

---

### 练习 02-04: 不可变引用遍历

> 难度：⭐
> 类似 C++ 的 const T& 遍历容器

创建一个数组，通过不可变引用来遍历它并打印每个元素。

```rust
fn main() {
    // TODO: 创建数组 [1, 2, 3, 4, 5]
    // TODO: 创建该数组的不可变引用
    // TODO: 通过引用遍历数组并打印每个元素
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];
    let ref_arr = &arr;
    for elem in ref_arr.iter() {
        println!("{}", elem);
    }
}
```

**说明：** `&arr` 创建了对数组的引用，类型为 `&[i32; 5]`。调用 `.iter()` 方法会在引用上自动工作，返回数组中每个元素的不可变引用。通过引用遍历不会消费数组的所有权。
</details>

---

### 练习 02-05: 不可变引用综合

> 难度：⭐⭐⭐
> 类似 C++ 的 const T& 传递

编写一个函数，接收一个字符串切片（`&str`）作为参数，统计其中元音字母（a, e, i, o, u）的个数并返回。在 `main` 中创建一个 `String`，通过不可变引用调用该函数。

```rust
// TODO: 定义 count_vowels 函数，接收 &str 参数，返回 u32
fn main() {
    let s = String::from("hello world");
    // TODO: 通过不可变引用调用 count_vowels
    // TODO: 打印结果
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn count_vowels(text: &str) -> u32 {
    let mut count = 0;
    for ch in text.chars() {
        match ch {
            'a' | 'e' | 'i' | 'o' | 'u' => count += 1,
            _ => {}
        }
    }
    count
}

fn main() {
    let s = String::from("hello world");
    let result = count_vowels(&s);
    println!("元音字母个数: {}", result);
    // s 的所有权仍在 main 中，这里仍然可以使用 s
    println!("原字符串仍然是: {}", s);
}
```

**说明：** `&s` 作为 `&String` 类型，会自动被 Rust 隐式强制转换为 `&str`（通过解引用强制多态）。函数只借用了字符串，没有获取所有权，因此调用后原变量 `s` 仍然可用。不可变引用确保了函数不会修改原始数据。
</details>

---

### 练习 02-06: 通过可变引用修改值

> 难度：⭐
> 类似 C++ 的 T&（可变引用）

创建一个 `i32` 变量，通过可变引用将其值修改为原来的两倍。

```rust
fn main() {
    // TODO: 创建可变变量 x = 10
    // TODO: 创建 x 的可变引用 y
    // TODO: 通过 y 将 x 的值修改为 20
    // TODO: 打印 x 的新值
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let mut x = 10;
    let y = &mut x;
    *y = 20;
    // x 的值已被修改
    println!("x = {}", x);
}
```

**说明：** 使用 `&mut x` 创建可变引用，通过 `*y = 20` 解引用赋值来修改原始值。注意变量 `x` 本身必须声明为 `mut`。可变引用允许你读取并修改被借用的值。
</details>

---

### 练习 02-07: 通过可变引用修改 String

> 难度：⭐
> 类似 C++ 的 T& 修改字符串

创建一个 `String`，通过可变引用向其中追加一段文本。

```rust
fn main() {
    // TODO: 创建可变 String s，内容为 "Hello"
    // TODO: 创建 s 的可变引用
    // TODO: 通过引用调用 push_str 追加 " World"
    // TODO: 打印 s
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let mut s = String::from("Hello");
    let r = &mut s;
    r.push_str(" World");
    // s 已被修改
    println!("{}", s);
}
```

**说明：** `&mut s` 创建了可变引用 `r`，通过 `r.push_str(...)` 可以直接修改 `s` 的内容。注意调用 `push_str` 时不需要显式解引用——Rust 会自动处理。修改后原变量 `s` 反映的是更新后的值。
</details>

---

### 练习 02-08: 只有一个可变引用

> 难度：⭐⭐
> 类似 C++ 的 T& 但 Rust 限制更严格

尝试在同一个作用域中创建两个可变引用，观察编译错误。然后修改代码使其正确编译。

```rust
fn main() {
    let mut value = 100;
    // TODO: 下面的代码会报错，请修改使其能正确编译
    let a = &mut value;
    let b = &mut value;
    *a += 1;
    *b += 1;
    println!("{}", value);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let mut value = 100;
    // 方式一：使用完一个可变引用后再创建下一个
    let a = &mut value;
    *a += 1;
    let b = &mut value;
    *b += 1;
    println!("{}", value);
}
```

**说明：** Rust 规定在同一个作用域中，同一时刻只能有一个可变引用。当 `a` 在使用时不能创建 `b`。但如果我们先使用完 `a`（`*a += 1` 之后 `a` 不再被使用），可以再创建 `b`。这是因为 Rust 编译器会追踪引用的最后一次使用位置（NLL——Non-Lexical Lifetimes）。
</details>

---

### 练习 02-09: 可变引用不能别名

> 难度：⭐⭐
> 类似 C++ 的 T& 别名导致未定义行为

下面的代码尝试用两种不同的方式同时修改同一个值，请分析错误并修复。

```rust
fn main() {
    let mut data = String::from("hello");
    let r1 = &mut data;
    let r2 = &mut data;  // 这行会报错
    r1.push_str(" world");
    r2.push_str(" !");
    println!("{}", data);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let mut data = String::from("hello");
    {
        let r1 = &mut data;
        r1.push_str(" world");
    } // r1 的作用域在此结束
    let r2 = &mut data;
    r2.push_str(" !");
    println!("{}", data);
}
```

**说明：** 通过引入花括号 `{}` 创建新的作用域，让 `r1` 在使用完后被销毁（其借用结束），然后再创建 `r2`。这是 Rust 防止数据竞争的核心机制——任意时刻最多只能有一个可变引用。
</details>

---

### 练习 02-10: 可变引用综合

> 难度：⭐⭐⭐
> 类似 C++ 的 T& 交换两个值

编写一个函数，接收两个 `i32` 的可变引用，交换它们指向的值。在 `main` 中验证。

```rust
// TODO: 定义 swap 函数，接收两个 &mut i32 参数
fn main() {
    let mut a = 5;
    let mut b = 10;
    // TODO: 调用 swap 交换 a 和 b 的值
    // TODO: 打印交换后的 a 和 b
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn swap(x: &mut i32, y: &mut i32) {
    let temp = *x;
    *x = *y;
    *y = temp;
}

fn main() {
    let mut a = 5;
    let mut b = 10;
    swap(&mut a, &mut b);
    println!("a = {}, b = {}", a, b);
}
```

**说明：** 函数 `swap` 通过两个可变引用直接修改原始值。`*x` 和 `*y` 解引用获取或设置值。注意调用时传入 `&mut a` 和 `&mut b`，且 `a` 和 `b` 必须声明为 `mut`。这种模式在 C++ 中也有对应的 `void swap(int& x, int& y)`。
</details>

---

### 练习 02-11: 不可变引用与可变引用不能共存

> 难度：⭐
> 检查 Rust 借用规则

下面的代码试图同时使用不可变引用和可变引用，请理解错误并修复。

```rust
fn main() {
    let mut s = String::from("rust");
    let r1 = &s;
    let r2 = &mut s;
    println!("{} {}", r1, r2);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let mut s = String::from("rust");
    let r1 = &s;
    println!("{}", r1);  // r1 的最后一次使用
    let r2 = &mut s;
    println!("{}", r2);
}
```

**说明：** Rust 不允许同时存在不可变引用和可变引用。因为如果同时存在，通过可变引用修改数据时，不可变引用就可能读到不一致的值。在当前代码中，只要在创建可变引用前结束不可变引用的使用（NLL 机制），就可以通过编译。
</details>

---

### 练习 02-12: 不可变引用与可变引用的顺序

> 难度：⭐
> 理解引用作用域重叠

指出下面代码的问题并修正，使其能正确编译运行。

```rust
fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let first = &numbers[0];
    numbers.push(6);
    println!("第一个元素是: {}", first);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let first = &numbers[0];
    println!("第一个元素是: {}", first);
    // 不可变引用 first 已使用完毕，现在可以修改
    numbers.push(6);
    println!("现在数组长度为: {}", numbers.len());
}
```

**说明：** `numbers.push(6)` 需要 `&mut numbers`（因为 `push` 会修改 `Vec`），但 `first` 是 `&numbers[0]` 的不可变引用。Rust 不允许在不可变引用有效时进行可变操作。调整顺序，先使用完不可变引用再修改即可。
</details>

---

### 练习 02-13: NLL 作用域结束

> 难度：⭐⭐
> 理解 NLL（Non-Lexical Lifetimes）

分析下面代码的借用作用域，判断 NLL 在何时允许新的借用。

```rust
fn main() {
    let mut name = String::from("Alice");
    let r = &mut name;
    r.push_str(" Smith");
    // 思考：这里可以创建新的借用吗？
    let r2 = &name;
    println!("{}", r2);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let mut name = String::from("Alice");
    let r = &mut name;
    r.push_str(" Smith");
    // r 在 push_str 之后不再被使用，NLL 认为其借用已结束
    let r2 = &name;
    println!("{}", r2);
}
```

**说明：** 这段代码可以正常编译。在 `r.push_str(" Smith")` 之后，`r` 不再被使用，NLL（Non-Lexical Lifetimes）机制会自动结束 `r` 的借用，因此后续可以创建不可变引用 `r2`。NLL 让 Rust 的借用检查更加灵活，不再依赖于花括号作用域。
</details>

---

### 练习 02-14: NLL 与作用域重叠

> 难度：⭐⭐
> 理解 NLL 何时不生效

阅读代码，判断它是否能通过编译。如果不能，请修改使其通过。

```rust
fn main() {
    let mut x = 10;
    let r1 = &x;
    let r2 = &mut x;
    println!("{}", r1);
    *r2 += 1;
    println!("{}", x);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let mut x = 10;
    let r1 = &x;
    println!("{}", r1);  // r1 最后一次使用，借用结束
    let r2 = &mut x;
    *r2 += 1;
    println!("{}", x);
}
```

**说明：** 原代码中 `r1` 在 `println!` 之前都有效，而 `r2` 的创建发生在 `r1` 仍被使用时，这违反了借用规则。修改后，`r1` 在 `println!` 之后不再被使用（NLL 结束其借用），因此可以安全地创建可变引用。
</details>

---

### 练习 02-15: 借用规则排查错误

> 难度：⭐⭐⭐
> 查找并修复借用相关错误

下面的代码包含多个借用/引用相关的错误。请找出所有错误并修复。

```rust
fn main() {
    let s = String::from("hello");
    let r1 = &mut s;
    let r2 = &s;
    println!("r1 = {}, r2 = {}", r1, r2);
    let r3 = &s;
    r1.push_str(" world");
    println!("r3 = {}", r3);
    let r4 = &mut s;
    println!("r1 = {}", r1);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let mut s = String::from("hello");
    
    // 先使用可变引用
    let r1 = &mut s;
    r1.push_str(" world");
    println!("r1 = {}", r1);
    // r1 的借用在此结束
    
    // 再使用不可变引用
    let r2 = &s;
    let r3 = &s;
    println!("r2 = {}, r3 = {}", r2, r3);
    // r2, r3 的借用在此结束
    
    // 最后再使用可变引用
    let r4 = &mut s;
    r4.push_str(" !");
    println!("r4 = {}", r4);
}
```

**说明：** 原代码存在多处违规：1）`s` 不是 `mut` 却试图创建可变引用；2）可变引用 `r1` 和不可变引用 `r2` 共存；3）在 `r1` 仍然有效时再次创建 `r1` 的引用。Rust 借用规则的核心是"任意时刻，要么有一个可变引用，要么有任意多个不可变引用"。修复策略是按顺序使用，互不重叠。
</details>

---

### 练习 02-16: 悬垂引用检测

> 难度：⭐
> 理解 Rust 如何阻止悬垂引用

为什么下面的代码无法编译？请解释并修复。

```rust
fn main() {
    let r;
    {
        let x = 10;
        r = &x;
    }
    println!("{}", r);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let x = 10;
    let r = &x;
    println!("{}", r);
    // 或者将 x 移动到外部作用域
}
```

**说明：** 原代码中 `x` 在内部作用域结束后被销毁，但 `r` 仍然引用它，形成悬垂引用（dangling reference）。Rust 编译器通过生命周期检查在编译期就阻止了这种情况。修复方式是将被引用的变量移到外部作用域，确保引用时变量仍然有效。
</details>

---

### 练习 02-17: 函数返回悬垂引用

> 难度：⭐
> 理解生命周期与悬垂引用的关系

下面的函数试图返回一个指向局部变量的引用，为什么不行？请修改。

```rust
// TODO: 下面的函数存在悬垂引用，请修改
fn get_ref() -> &String {
    let s = String::from("hello");
    &s
}

fn main() {
    let r = get_ref();
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
// 方案：返回 String 本身，转移所有权
fn get_owned() -> String {
    let s = String::from("hello");
    s
}

fn main() {
    let r = get_owned();
    println!("{}", r);
}
```

**说明：** 函数 `get_ref` 返回对局部变量 `s` 的引用，但 `s` 在函数结束时就被销毁了，引用指向了无效内存。Rust 编译器会检测并拒绝这种悬垂引用。正确的做法是返回 `String` 本身，将所有权转移给调用者。
</details>

---

### 练习 02-18: 引用作为函数参数（不可变）

> 难度：⭐⭐
> 类似 C++ 的 const T& 参数传递

编写一个函数，接收一个 `&String` 参数，返回该字符串的长度。在 `main` 中调用并验证原变量仍然可用。

```rust
// TODO: 定义 string_length 函数，接收 &String，返回 usize
fn main() {
    let s = String::from("Rust programming");
    // TODO: 调用 string_length
    // TODO: 打印结果，同时验证 s 仍然可用
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn string_length(s: &String) -> usize {
    s.len()
}

fn main() {
    let s = String::from("Rust programming");
    let len = string_length(&s);
    println!("字符串长度: {}", len);
    // s 的所有权未被转移，仍然可用
    println!("原字符串: {}", s);
}
```

**说明：** `&String` 表示对 `String` 的不可变引用。函数只借用数据，不获取所有权，因此调用后 `s` 仍然有效。参数类型也可以写成 `&str` 以接受更多类型的字符串引用，这里为了练习明确使用了 `&String`。
</details>

---

### 练习 02-19: 引用作为函数参数（可变）

> 难度：⭐⭐
> 类似 C++ 的 T& 参数传递

编写一个函数，接收一个 `&mut String` 参数，在字符串末尾添加 "!!!"。在 `main` 中调用并验证效果。

```rust
// TODO: 定义 add_exclamation 函数，接收 &mut String，无返回值
fn main() {
    let mut msg = String::from("Hello");
    // TODO: 调用 add_exclamation
    // TODO: 打印修改后的 msg
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn add_exclamation(s: &mut String) {
    s.push_str("!!!");
}

fn main() {
    let mut msg = String::from("Hello");
    add_exclamation(&mut msg);
    println!("{}", msg);
}
```

**说明：** `&mut String` 表示对 `String` 的可变引用。函数可以通过该引用修改原始字符串，而不需要获取所有权。这意味着调用者保留所有权，且函数结束后仍可使用该变量。注意变量和引用都需要使用 `mut` 关键字。
</details>

---

### 练习 02-20: 引用综合运用——借用版 word count

> 难度：⭐⭐⭐
> 综合运用不可变引用、可变引用、切片

实现一个单词计数函数 `word_count`，它接收一个 `&str`（字符串切片），返回单词数量。然后在 `main` 中调用它。再实现一个辅助函数 `report_word_count`，它接收一个 `&String` 的不可变引用，打印字符串内容及其单词数。

```rust
// TODO: 定义 word_count 函数，接收 &str，返回 usize
// TODO: 定义 report_word_count 函数，接收 &String，打印内容和单词数

fn main() {
    let sentence = String::from("Rust is a systems programming language");
    // TODO: 调用 report_word_count（传入不可变引用）
    // TODO: 验证 sentence 的所有权未被转移——再次打印 sentence
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn word_count(text: &str) -> usize {
    if text.is_empty() {
        return 0;
    }
    text.split_whitespace().count()
}

fn report_word_count(s: &String) {
    let count = word_count(s.as_str());
    println!("内容: \"{}\"", s);
    println!("单词数: {}", count);
}

fn main() {
    let sentence = String::from("Rust is a systems programming language");
    report_word_count(&sentence);
    // sentence 的所有权仍在 main 中
    println!("原变量仍然可用: {}", sentence);
}
```

**说明：** `word_count` 通过 `&str` 借用字符串，`split_whitespace()` 返回迭代器并用 `.count()` 统计。`report_word_count` 接收 `&String`，通过 `.as_str()` 转换为 `&str` 传给 `word_count`。两个函数的参数都是引用，没有转移所有权，因此 `main` 中的 `sentence` 在调用后仍然可用。整个程序完全通过借用完成，没有所有权的转移。
</details>
