# 11 Vec 与切片

`Vec<T>`（动态数组）是 Rust 最常用的堆分配容器，相当于 C++ 的 `std::vector` 或 Java 的 `ArrayList`。切片 `&[T]` / `&mut [T]` 则是对连续内存区域的**视图**，不拥有所有权。本章将从基础创建、增删改查，到排序查找、迭代变换，系统性地练习 Vec 和切片的操作。

---

### 练习 11-01: 使用 vec![] 宏创建 Vec

> 难度：⭐
> 类似 C++ 的 std::vector 初始化列表 / Java 的 Arrays.asList

使用 `vec![]` 宏创建一个包含 `1, 2, 3, 4, 5` 的 `Vec<i32>`，并打印长度和第三个元素。

```rust
// TODO: 使用 vec![] 创建 Vec，打印长度和第三个元素
fn main() {
    // 使用 vec![] 宏创建 Vec

    // 打印长度

    // 打印第三个元素
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    println!("长度: {}", v.len());
    println!("第三个元素: {}", v[2]);
}
```

**说明：** `vec![]` 是最方便的 Vec 创建方式。`v[2]` 通过 `Index` trait 访问元素，如果越界会在运行时 panic。`len()` 返回元素个数。
</details>

---

### 练习 11-02: 使用 Vec::new() 和 push 构建 Vec

> 难度：⭐
> 类似 C++ 的 `std::vector::push_back` / Java 的 `ArrayList.add`

使用 `Vec::new()` 创建一个空 Vec，然后用 `push` 方法添加三个水果名称（字符串），最后打印整个 Vec。

```rust
// TODO: 创建空 Vec 并 push 元素
fn main() {
    // 创建空 Vec<String>

    // push 三个字符串

    // 打印 {:?}
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let mut fruits = Vec::new();
    fruits.push("苹果".to_string());
    fruits.push("香蕉".to_string());
    fruits.push("樱桃".to_string());
    println!("{:?}", fruits);
}
```

**说明：** `Vec::new()` 创建空 Vec，需要 `mut` 才能 `push`。`"{:?}"` 使用 `Debug` 格式化打印整个 Vec。`to_string()` 将 `&str` 转为 `String`。
</details>

---

### 练习 11-03: pop 和 insert 操作

> 难度：⭐⭐
> 类似 C++ 的 `pop_back` / `insert` / Java 的 `remove` / `add(index)`

给定一个 Vec `[10, 20, 30, 40, 50]`，先用 `pop` 移除最后一个元素并打印，再在索引 1 处 `insert` 一个 99，打印每次操作后的 Vec。

```rust
// TODO: 补全 pop 和 insert 操作
fn main() {
    let mut v = vec![10, 20, 30, 40, 50];
    
    // pop 移除最后一个元素，用 match 处理返回值

    // 在索引 1 处插入 99

    // 打印最终 Vec
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let mut v = vec![10, 20, 30, 40, 50];
    
    match v.pop() {
        Some(val) => println!("弹出: {}", val),
        None => println!("Vec 为空"),
    }
    println!("pop 后: {:?}", v);
    
    v.insert(1, 99);
    println!("insert 后: {:?}", v);
}
```

**说明：** `pop()` 返回 `Option<T>`，取出最后一个元素并在 Vec 中移除。`insert(index, value)` 在指定位置插入，该位置及之后的元素右移。两者都可能导致元素移动，`insert` 在 Vec 头部插入是 O(n) 操作。
</details>

---

### 练习 11-04: remove 和 swap_remove 操作

> 难度：⭐⭐
> 类似 C++ 的 `erase` / Java 的 `remove(index)`

给定一个 Vec `[1, 2, 3, 4, 5]`，分别使用 `remove` 和 `swap_remove` 删除索引 1 处的元素，观察两种方式的结果差异。

```rust
// TODO: 分别使用 remove 和 swap_remove 删除元素
fn main() {
    let mut v1 = vec![1, 2, 3, 4, 5];
    // 使用 remove 删除索引 1

    let mut v2 = vec![1, 2, 3, 4, 5];
    // 使用 swap_remove 删除索引 1

    // 打印两个 Vec 的最终结果
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let mut v1 = vec![1, 2, 3, 4, 5];
    let removed1 = v1.remove(1);
    println!("remove 删除了 {}, v1 = {:?}", removed1, v1);

    let mut v2 = vec![1, 2, 3, 4, 5];
    let removed2 = v2.swap_remove(1);
    println!("swap_remove 删除了 {}, v2 = {:?}", removed2, v2);
}
```

**说明：** `remove(index)` 删除指定元素并左移后续元素（O(n)）。`swap_remove(index)` 把指定元素与最后一个元素交换，再弹出（O(1)），但**不保留顺序**。当顺序不重要时优先用 `swap_remove` 以获得更好性能。
</details>

---

### 练习 11-05: Vec 操作综合挑战

> 难度：⭐⭐⭐
> 类似 C++ 的 std::vector 综合操作

给定初始 Vec `[3, 1, 4, 1, 5, 9, 2, 6]`，依次完成以下操作，每步之后打印当前 Vec：

1. 在末尾追加 `5` 和 `3`
2. 删除第一个等于 `1` 的元素
3. 在索引 3 处插入 `7`
4. 如果长度大于 5，用 `swap_remove` 删除最后一个元素
5. 翻转整个 Vec

```rust
// TODO: 完成五步操作，每步后打印 Vec
fn main() {
    let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6];
    println!("初始: {:?}", v);

    // 步骤 1: 追加 5 和 3

    // 步骤 2: 删除第一个等于 1 的元素（用 iter().position() 找到索引）

    // 步骤 3: 在索引 3 处插入 7

    // 步骤 4: 如果长度 > 5，swap_remove 最后一个元素

    // 步骤 5: 翻转 Vec（用 reverse()）

    println!("最终: {:?}", v);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6];
    println!("初始: {:?}", v);

    v.push(5);
    v.push(3);
    println!("步骤1: {:?}", v);

    if let Some(idx) = v.iter().position(|&x| x == 1) {
        v.remove(idx);
    }
    println!("步骤2: {:?}", v);

    v.insert(3, 7);
    println!("步骤3: {:?}", v);

    if v.len() > 5 {
        v.swap_remove(v.len() - 1);
    }
    println!("步骤4: {:?}", v);

    v.reverse();
    println!("步骤5: {:?}", v);
}
```

**说明：** `iter().position(|x| ...)` 返回第一个满足条件的元素的索引（`Option<usize>`）。`reverse()` 原地翻转 Vec 的元素顺序。综合练习展示了 Vec 的核心修改操作。
</details>

---

### 练习 11-06: 创建和打印切片

> 难度：⭐
> 类似 C++ 的 `std::span` / Java 的数组子视图

从一个 `Vec<i32>` 中创建切片 `&[i32]`，分别打印整个切片和从索引 1 开始到索引 4 的切片子集。

```rust
// TODO: 创建切片并打印
fn main() {
    let v = vec![10, 20, 30, 40, 50, 60];
    
    // 创建对整个 Vec 的切片

    // 创建从索引 1 到 4（不含）的子切片

    // 打印两个切片
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let v = vec![10, 20, 30, 40, 50, 60];
    
    let full: &[i32] = &v;
    let part: &[i32] = &v[1..4];
    
    println!("整个切片: {:?}", full);
    println!("子切片 [1..4]: {:?}", part);
}
```

**说明：** `&v` 可以将 `Vec<T>` 自动转为 `&[T]` 切片。`&v[1..4]` 使用**范围语法**创建子切片，左闭右开。切片不拥有数据，只是对原 Vec 的借用。如果范围越界会 panic。
</details>

---

### 练习 11-07: 从数组创建切片

> 难度：⭐
> 类似 C++ 的数组退化为指针

给定一个固定大小数组 `[2, 4, 6, 8, 10]`，创建覆盖全部元素的切片和覆盖前三个元素的切片，打印它们的长度。

```rust
// TODO: 从数组创建切片
fn main() {
    let arr = [2, 4, 6, 8, 10];
    
    // 创建覆盖全部元素的切片

    // 创建覆盖前三个元素的切片

    // 打印两个切片的长度
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let arr = [2, 4, 6, 8, 10];
    
    let all: &[i32] = &arr[..];
    let first_three: &[i32] = &arr[..3];
    
    println!("全部切片长度: {}", all.len());
    println!("前三个切片长度: {}", first_three.len());
}
```

**说明：** `&arr[..]` 取整个数组的切片，`&arr[..3]` 取前三个元素。数组到切片的转换是隐式的，但显式标注类型 `&[i32]` 有助于理解。切片长度在编译期不固定，是动态确定的。
</details>

---

### 练习 11-08: 通过可变切片修改元素

> 难度：⭐⭐
> 类似 C++ 的 `std::span` 可变版本

创建一个 `Vec<i32>`，获取其可变切片 `&mut [i32]`，通过切片将每个元素乘以 2。

```rust
// TODO: 用可变切片修改所有元素
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    
    // 获取可变切片

    // 用 for 循环遍历可变切片，每个元素乘以 2

    // 打印修改后的 Vec
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    
    let slice: &mut [i32] = &mut v;
    for elem in slice.iter_mut() {
        *elem *= 2;
    }
    
    println!("修改后: {:?}", v);
}
```

**说明：** `&mut v` 创建可变切片。`iter_mut()` 返回可以修改元素的迭代器，`*elem` 解引用后赋值。可变切片保证没有其他借用同时存在。
</details>

---

### 练习 11-09: 可变切片的部分修改

> 难度：⭐⭐
> 类似 C++ 指针偏移操作

给定 `Vec<String>`，获取从索引 2 开始的可变子切片，将其中的字符串追加后缀 `"!!!"`。

```rust
// TODO: 修改可变子切片中的字符串
fn main() {
    let mut words = vec!["hello".to_string(), "world".to_string(), "rust".to_string(), "slice".to_string()];
    
    // 获取从索引 2 开始的可变子切片

    // 遍历并追加 "!!!"

    // 打印所有元素
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let mut words = vec!["hello".to_string(), "world".to_string(), "rust".to_string(), "slice".to_string()];
    
    let sub: &mut [String] = &mut words[2..];
    for s in sub.iter_mut() {
        s.push_str("!!!");
    }
    
    for w in &words {
        println!("{}", w);
    }
}
```

**说明：** `&mut words[2..]` 创建从索引 2 到末尾的可变子切片。`push_str` 追加字符串内容。可变子切片与原始 Vec 共享内存，修改会反映在原 Vec 上。
</details>

---

### 练习 11-10: 切片综合挑战

> 难度：⭐⭐⭐
> 类似 C++ 的 std::span 算法操作

给定数组 `[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]`，创建两个可变子切片：前一半和后一半。通过第一个子切片将所有元素加 10，通过第二个子切片将所有元素乘 2，最后打印整个数组。

> 提示：可变借用无法同时重叠，使用 `split_at_mut` 方法。

```rust
// TODO: 使用 split_at_mut 分割切片并分别修改
fn main() {
    let mut arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 使用 split_at_mut(5) 分割为两个可变子切片

    // 前一半每个元素加 10

    // 后一半每个元素乘 2

    // 打印整个数组
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let mut arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    let (left, right) = arr.split_at_mut(5);
    
    for x in left.iter_mut() {
        *x += 10;
    }
    for x in right.iter_mut() {
        *x *= 2;
    }
    
    println!("{:?}", arr);
}
```

**说明：** `split_at_mut(n)` 是安全分割可变切片的唯一方式——它利用编译期检查确保两个子切片不重叠。前 5 个元素变成 `[11, 12, 13, 14, 15]`，后 5 个变成 `[12, 14, 16, 18, 20]`。这是 Rust 借用检查器的精妙之处：它防止了数据竞争。
</details>

---

### 练习 11-11: Vec 排序（sort）

> 难度：⭐
> 类似 C++ 的 `std::sort` / Java 的 `Collections.sort`

对一个无序的 `Vec<i32>` 使用 `.sort()` 方法进行升序排序，然后打印排序后的结果。

```rust
// TODO: 对 Vec 进行排序
fn main() {
    let mut nums = vec![42, 7, 15, 3, 88, 21, 9];
    
    // 调用 sort() 排序

    // 打印排序结果
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let mut nums = vec![42, 7, 15, 3, 88, 21, 9];
    nums.sort();
    println!("排序后: {:?}", nums);
}
```

**说明：** `sort()` 要求元素实现 `Ord` trait（即全序比较）。对于整数等基本类型，`Ord` 已自动实现。`sort()` 是稳定排序（`sort_unstable` 是更快但不稳定的版本）。
</details>

---

### 练习 11-12: 自定义排序（sort_by）

> 难度：⭐
> 类似 C++ 的 `std::sort` 自定义比较器 / Java 的 `Comparator`

给定字符串切片 `["apple", "kiwi", "banana", "cherry", "date"]`，按字符串**长度**降序排列。

```rust
// TODO: 按字符串长度降序排序
fn main() {
    let mut fruits = vec!["apple", "kiwi", "banana", "cherry", "date"];
    
    // 使用 sort_by 按长度降序排列

    // 打印结果
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let mut fruits = vec!["apple", "kiwi", "banana", "cherry", "date"];
    fruits.sort_by(|a, b| b.len().cmp(&a.len()));
    println!("{:?}", fruits);
}
```

**说明：** `sort_by` 接受一个比较闭包，返回 `Ordering`。`b.len().cmp(&a.len())` 中的 `b` 在前实现降序（`a.cmp(&b)` 是升序）。字符串 `len()` 返回字节长度，对 ASCII 字符等同于字符数。
</details>

---

### 练习 11-13: 二分查找基础

> 难度：⭐⭐
> 类似 C++ 的 `std::lower_bound` / Java 的 `Collections.binarySearch`

给定已排序的 Vec `[10, 20, 30, 40, 50, 60, 70]`，使用 `binary_search` 查找 `30` 和 `35`，打印各自的查找结果。

```rust
// TODO: 二分查找两个值
fn main() {
    let nums = vec![10, 20, 30, 40, 50, 60, 70];
    
    // 查找 30

    // 查找 35

    // 打印结果
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let nums = vec![10, 20, 30, 40, 50, 60, 70];
    
    match nums.binary_search(&30) {
        Ok(idx) => println!("30 的索引: {}", idx),
        Err(idx) => println!("30 未找到，可插入索引: {}", idx),
    }
    
    match nums.binary_search(&35) {
        Ok(idx) => println!("35 的索引: {}", idx),
        Err(idx) => println!("35 未找到，可插入索引: {}", idx),
    }
}
```

**说明：** `binary_search` 要求 Vec **已排序**。返回 `Result<usize, usize>`——`Ok(i)` 表示找到的索引，`Err(i)` 表示若插入应插入的位置索引。二分查找是 O(log n)。
</details>

---

### 练习 11-14: 二分查找自定义比较

> 难度：⭐⭐
> 类似 C++ 的 `std::lower_bound` 自定义谓词

给定已按**字符串长度**升序排序的 Vec `["a", "ab", "abc", "abcd", "abcde"]`，使用 `binary_search_by` 查找长度为 3 的第一个字符串。

```rust
// TODO: 使用 binary_search_by 按长度查找
fn main() {
    let words = vec!["a", "ab", "abc", "abcd", "abcde"];
    
    // 使用 binary_search_by 查找长度为 3 的字符串

    // 打印结果
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let words = vec!["a", "ab", "abc", "abcd", "abcde"];
    
    match words.binary_search_by(|s| s.len().cmp(&3)) {
        Ok(idx) => println!("找到长度为 3 的字符串: {}", words[idx]),
        Err(idx) => println!("未找到，可插入索引: {}", idx),
    }
}
```

**说明：** `binary_search_by` 接受一个闭包，闭包返回 `Ordering`。比较函数必须与排序所用的顺序一致，否则结果不可预测。这里 Vec 已按长度升序排列，所以 `s.len().cmp(&3)` 是正确的比较逻辑。
</details>

---

### 练习 11-15: 排序 + 查找综合挑战

> 难度：⭐⭐⭐
> 类似 C++ 中排序后批量查找

给定学生成绩数据 `[("Alice", 85), ("Bob", 72), ("Charlie", 95), ("Diana", 72), ("Eve", 85)]`，先按成绩降序排序，再使用 `binary_search_by` 查找是否存在成绩为 85 的学生（注意处理重复值）。

```rust
// TODO: 排序后二分查找
fn main() {
    let mut students = vec![
        ("Alice", 85),
        ("Bob", 72),
        ("Charlie", 95),
        ("Diana", 72),
        ("Eve", 85),
    ];
    
    // 按成绩降序排序

    // 使用 binary_search_by 查找成绩 85

    // 打印结果（找到第一个 85 分的学生）
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let mut students = vec![
        ("Alice", 85),
        ("Bob", 72),
        ("Charlie", 95),
        ("Diana", 72),
        ("Eve", 85),
    ];
    
    students.sort_by(|a, b| b.1.cmp(&a.1));
    println!("排序后: {:?}", students);
    
    match students.binary_search_by(|s| s.1.cmp(&85)) {
        Ok(idx) => {
            println!("找到 85 分的学生: {} ({})", students[idx].0, students[idx].1);
            // 可能存在重复的 85 分，检查前后
            if idx > 0 && students[idx - 1].1 == 85 {
                println!("前一个也是 85 分: {} ({})", students[idx - 1].0, students[idx - 1].1);
            }
            if idx + 1 < students.len() && students[idx + 1].1 == 85 {
                println!("后一个也是 85 分: {} ({})", students[idx + 1].0, students[idx + 1].1);
            }
        }
        Err(_) => println!("未找到 85 分的学生"),
    }
}
```

**说明：** 二分查找在有重复值时只能找到**其中一个**（不保证是第一个或最后一个）。查找重复集合时需要手动向两侧扩展。排序的比较逻辑必须与二分查找一致——这里是降序，所以比较函数也是 `s.1.cmp(&85)`。
</details>

---

### 练习 11-16: 三种迭代方式

> 难度：⭐
> 类似 Java 中普通 for 循环 / 增强 for 循环的区别

给定 Vec `[100, 200, 300, 400]`，分别使用 `iter()`、`iter_mut()` 和 `into_iter()` 遍历并观察区别。

```rust
// TODO: 分别使用 iter()、iter_mut()、into_iter() 遍历
fn main() {
    let v = vec![100, 200, 300, 400];
    
    // 使用 iter() 遍历，只读

    // 重新创建 v，使用 iter_mut() 遍历，每个元素加 50

    // 重新创建 v，使用 into_iter() 遍历，打印每个值
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let v = vec![100, 200, 300, 400];
    
    // iter() — 不可变引用
    for x in v.iter() {
        print!("{} ", x);
    }
    println!(); // v 仍然可用
    
    let mut v = vec![100, 200, 300, 400];
    // iter_mut() — 可变引用
    for x in v.iter_mut() {
        *x += 50;
    }
    println!("{:?}", v); // v 已修改
    
    // into_iter() — 消费 Vec
    for x in v.into_iter() {
        print!("{} ", x);
    }
    println!();
    // 此后再也不能使用 v（所有权已转移）
}
```

**说明：** `iter()` 返回 `&T`，不取得所有权；`iter_mut()` 返回 `&mut T`，可修改元素；`into_iter()` 消费 Vec，返回 `T`（所有权转移）。`for x in &v` 等价于 `for x in v.iter()`，`for x in &mut v` 等价于 `for x in v.iter_mut()`，`for x in v` 等价于 `for x in v.into_iter()`。
</details>

---

### 练习 11-17: 迭代器变换 (map/collect)

> 难度：⭐
> 类似 Java Stream 的 map / collect / C++ 的 std::ranges::transform

给定 Vec `[1, 2, 3, 4, 5]`，使用迭代器的 `map` 和 `collect` 将其每个元素平方，收集到新的 `Vec<i32>` 中。

```rust
// TODO: 使用 map 和 collect 变换 Vec
fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    
    // 使用 iter()、map()、collect() 生成平方后的新 Vec

    // 打印结果
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let squared: Vec<i32> = nums.iter().map(|x| x * x).collect();
    println!("平方后: {:?}", squared);
}
```

**说明：** `map(|x| x * x)` 对每个元素应用闭包，`collect()` 将迭代器收集为指定容器。`collect` 的类型由左侧变量类型 `Vec<i32>` 推断。迭代器适配器是**惰性**的——`collect` 驱动整个计算链。
</details>

---

### 练习 11-18: retain 和 dedup

> 难度：⭐⭐
> 类似 C++ 的 `std::erase_if` + `std::unique` / Java 的 `removeIf`

给定 Vec `[1, 2, 2, 3, 4, 4, 4, 5, 6, 6]`，先用 `retain` 保留所有偶数，再用 `dedup` 去除连续的重复项，打印每步结果。

```rust
// TODO: retain 过滤后 dedup 去重
fn main() {
    let mut v = vec![1, 2, 2, 3, 4, 4, 4, 5, 6, 6];
    
    // retain 保留偶数

    // dedup 去除连续重复

    // 打印最终结果
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let mut v = vec![1, 2, 2, 3, 4, 4, 4, 5, 6, 6];
    
    v.retain(|&x| x % 2 == 0);
    println!("retain 后: {:?}", v);
    
    v.dedup();
    println!("dedup 后: {:?}", v);
}
```

**说明：** `retain` 接受一个返回 `bool` 的闭包，保留满足条件的元素（原地过滤）。`dedup` 仅去除**连续且相等**的重复项——如果要去除所有重复，需先排序。`retain` 是 O(n)，`dedup` 是 O(n)。
</details>

---

### 练习 11-19: 分块操作 (chunks / windows)

> 难度：⭐⭐
> 类似 Python 的列表分块 / C++ 的滑动窗口

给定数组 `[1, 2, 3, 4, 5, 6, 7, 8, 9]`，分别使用 `chunks(3)` 和 `windows(3)` 打印不同的分组结果，观察两者区别。

```rust
// TODO: 使用 chunks 和 windows 分组
fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    
    // 用 chunks(3) 分成每 3 个一组

    // 用 windows(3) 滑动窗口

    // 打印两种结果
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    
    println!("chunks(3):");
    for chunk in arr.chunks(3) {
        println!("  {:?}", chunk);
    }
    
    println!("windows(3):");
    for win in arr.windows(3) {
        println!("  {:?}", win);
    }
}
```

**说明：** `chunks(n)` 将切片分成不重叠的 n 元素块，最后一块可能不足 n 个。`windows(n)` 产生长度为 n 的连续滑动窗口，每个窗口依次平移一个元素。`chunks` 适合分块处理，`windows` 适合滑动窗口算法（如移动平均、模式匹配）。
</details>

---

### 练习 11-20: 综合数据处理挑战

> 难度：⭐⭐⭐
> 类似实际工程中的数据统计任务

给定一个整数数组 `[3, 7, 2, 9, 3, 7, 5, 7, 2, 8, 3, 1, 6, 4, 7, 9, 5, 2, 8, 6]`，完成以下任务：

1. 统计每个数字出现的次数（用 `HashMap`）
2. 找出出现次数最多的数字
3. 计算所有数字的平均值（保留两位小数）
4. 过滤出所有出现次数大于 1 的数字并去重，升序排序

```rust
// TODO: 完成综合数据处理
use std::collections::HashMap;

fn main() {
    let data = vec![3, 7, 2, 9, 3, 7, 5, 7, 2, 8, 3, 1, 6, 4, 7, 9, 5, 2, 8, 6];
    
    // 1. 统计每个数字出现次数

    // 2. 找出出现次数最多的数字

    // 3. 计算平均值

    // 4. 过滤出现次数 > 1 的数字，去重并升序排序

    // 打印所有结果
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::HashMap;

fn main() {
    let data = vec![3, 7, 2, 9, 3, 7, 5, 7, 2, 8, 3, 1, 6, 4, 7, 9, 5, 2, 8, 6];
    
    // 1. 统计频次
    let mut freq = HashMap::new();
    for &n in &data {
        *freq.entry(n).or_insert(0) += 1;
    }
    println!("频次统计: {:?}", freq);
    
    // 2. 找出出现次数最多的数字
    let max_num = freq.iter()
        .max_by_key(|(_, &count)| count)
        .map(|(num, count)| (*num, *count))
        .unwrap();
    println!("出现最多的数字: {} ({} 次)", max_num.0, max_num.1);
    
    // 3. 计算平均值
    let sum: i32 = data.iter().sum();
    let avg = sum as f64 / data.len() as f64;
    println!("平均值: {:.2}", avg);
    
    // 4. 过滤出现次数 > 1 的数字，去重并升序排序
    let mut result: Vec<i32> = data.clone();
    result.sort();
    result.dedup();
    result.retain(|&x| *freq.get(&x).unwrap() > 1);
    println!("出现多次的数字(升序): {:?}", result);
}
```

**说明：** `HashMap` 结合 `entry()` API 是 Rust 中统计频次的惯用法。`max_by_key` 找出最大值的条目。`dedup` 前需要先 `sort` 才能保证去除所有重复项。`{:.2}` 格式化浮点数为两位小数。本题综合了 Vec、切片、迭代器、HashMap 等多个知识点的协作。
</details>
