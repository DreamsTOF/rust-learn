# 12 HashMap 与 BTreeMap

Rust 标准库提供了两种键值对容器：`HashMap`（基于哈希表的无序映射，类似 Java 的 `HashMap` 或 C++ 的 `std::unordered_map`）和 `BTreeMap`（基于 B 树的有序映射，类似 C++ 的 `std::map` 或 Java 的 `TreeMap`）。本章练习涵盖 HashMap 的基本操作、`entry` API（Rust 独特优势）、自定义 Key 类型，以及 BTreeMap 的有序特性和范围查询。

---

### 练习 12-01: 创建并插入 HashMap

> 难度：⭐
> 类似 Java 的 `HashMap<String, Integer> map = new HashMap<>(); map.put(...)`

补全代码，创建一个 `HashMap<&str, i32>`，插入三组成绩数据。

```rust
use std::collections::HashMap;

fn main() {
    // TODO: 创建一个空的 HashMap，键为 &str，值为 i32
    // let mut scores = ...;

    // TODO: 插入三组成绩：("Alice", 95), ("Bob", 82), ("Charlie", 90)

    println!("{:?}", scores);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Alice", 95);
    scores.insert("Bob", 82);
    scores.insert("Charlie", 90);

    println!("{:?}", scores);
}
```

**说明：** `HashMap::new()` 创建空 Map，`insert(key, value)` 插入键值对。`HashMap` 默认使用 SipHash 哈希算法，具有抗 HashDoS 能力。
</details>

---

### 练习 12-02: 从 Vec 构建 HashMap

> 难度：⭐
> 用 `collect()` 从两个平行 Vec 构建 HashMap

补全代码，将两个 Vec 合并成 HashMap。

```rust
use std::collections::HashMap;

fn main() {
    let keys = vec!["name", "age", "city"];
    let values = vec!["Alice", "30", "Beijing"];

    // TODO: 使用 into_iter() 和 zip() 以及 collect() 构建 HashMap
    // let map: HashMap<&str, &str> = ...;

    println!("{:?}", map);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::HashMap;

fn main() {
    let keys = vec!["name", "age", "city"];
    let values = vec!["Alice", "30", "Beijing"];

    let map: HashMap<&str, &str> = keys.into_iter()
        .zip(values.into_iter())
        .collect();

    println!("{:?}", map);
}
```

**说明：** `zip` 将两个迭代器配对为元组迭代器，`collect` 将元组收集为 `HashMap`。需要显式标注 `HashMap` 类型，因为 `collect` 可以收集到多种集合类型。
</details>

---

### 练习 12-03: 查找与获取

> 难度：⭐⭐
> 类似 Java 的 `map.get(key)` 与 `map.containsKey(key)`

补全代码，根据菜名查找价格，处理不存在的情况。

```rust
use std::collections::HashMap;

fn main() {
    let mut menu = HashMap::new();
    menu.insert("宫保鸡丁", 38);
    menu.insert("鱼香肉丝", 32);
    menu.insert("麻婆豆腐", 22);

    // TODO: 查找 "宫保鸡丁" 的价格，用 match 处理 Some/None
    // let price = menu.get(...);

    // TODO: 查找 "水煮鱼" 的价格，用 unwrap_or 提供默认值 0
    // let fallback = ...;

    // TODO: 检查 "麻婆豆腐" 是否存在，用 contains_key
    // let exists = ...;

    println!("{price:?} {fallback} {exists}");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::HashMap;

fn main() {
    let mut menu = HashMap::new();
    menu.insert("宫保鸡丁", 38);
    menu.insert("鱼香肉丝", 32);
    menu.insert("麻婆豆腐", 22);

    let price = menu.get("宫保鸡丁").copied();
    let fallback = menu.get("水煮鱼").copied().unwrap_or(0);
    let exists = menu.contains_key("麻婆豆腐");

    println!("{price:?} {fallback} {exists}");
}
```

**说明：** `get(key)` 返回 `Option<&V>`，`copied()` 将 `Option<&i32>` 转为 `Option<i32>`。`contains_key` 返回 `bool`，用于仅检查存在性而不获取值。
</details>

---

### 练习 12-04: 遍历 HashMap

> 难度：⭐⭐
> 遍历所有键值对

补全代码，遍历 HashMap 并格式化输出。

```rust
use std::collections::HashMap;

fn main() {
    let mut population = HashMap::new();
    population.insert("北京", 2154);
    population.insert("上海", 2428);
    population.insert("广州", 1490);
    population.insert("深圳", 1756);

    // TODO: 用 for 循环遍历 population，打印 "城市: xxx, 人口: xxx 万"
    for ... {
        println!("...");
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::HashMap;

fn main() {
    let mut population = HashMap::new();
    population.insert("北京", 2154);
    population.insert("上海", 2428);
    population.insert("广州", 1490);
    population.insert("深圳", 1756);

    for (city, pop) in &population {
        println!("城市: {city}, 人口: {pop} 万");
    }
}
```

**说明：** 遍历 `&HashMap` 产生 `(&K, &V)` 引用元组。若使用 `into_iter()` 则会消耗 HashMap。也可以分别用 `.keys()` 和 `.values()` 遍历键或值。
</details>

---

### 练习 12-05: HashMap 基础综合

> 难度：⭐⭐⭐
> 综合练习：购物车结算

编写一个程序，使用 HashMap 实现购物车功能：添加商品、更新数量、删除商品、计算总价。

```rust
use std::collections::HashMap;

// 商品价格表
fn price_list() -> HashMap<&'static str, f64> {
    let mut prices = HashMap::new();
    prices.insert("苹果", 5.5);
    prices.insert("香蕉", 3.2);
    prices.insert("牛奶", 12.0);
    prices.insert("面包", 8.5);
    prices
}

fn main() {
    let prices = price_list();
    // TODO: 创建购物车 cart: HashMap<&str, i32>，记录每种商品的数量

    // TODO: 添加 3 个苹果，2 盒牛奶，1 个面包
    // TODO: 将苹果数量更新为 5 个
    // TODO: 删除面包
    // TODO: 遍历购物车，根据价格表计算总价并打印
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::HashMap;

fn price_list() -> HashMap<&'static str, f64> {
    let mut prices = HashMap::new();
    prices.insert("苹果", 5.5);
    prices.insert("香蕉", 3.2);
    prices.insert("牛奶", 12.0);
    prices.insert("面包", 8.5);
    prices
}

fn main() {
    let prices = price_list();
    let mut cart = HashMap::new();

    cart.insert("苹果", 3);
    cart.insert("牛奶", 2);
    cart.insert("面包", 1);

    cart.insert("苹果", 5);   // 覆盖更新
    cart.remove("面包");

    let mut total = 0.0;
    for (item, qty) in &cart {
        if let Some(price) = prices.get(item) {
            let subtotal = price * (*qty as f64);
            println!("{item}: {qty} × ¥{price:.1} = ¥{subtotal:.1}");
            total += subtotal;
        }
    }
    println!("总计: ¥{total:.1}");
}
```

**说明：** `insert` 已存在 key 会覆盖旧值。`remove(key)` 删除键值对。遍历时注意类型：`cart` 是 `HashMap<&str, i32>`，遍历得到 `(&i32)` 需要用 `*qty` 解引用转为 `i32`，再用 `as f64` 转换以进行浮点运算。
</details>

---

### 练习 12-06: entry API — 计数

> 难度：⭐
> 类似 C++ 的 `map[key]++`，但 Rust 需要 entry API

补全代码，用 `entry` + `or_insert` 统计字符出现次数。

```rust
use std::collections::HashMap;

fn main() {
    let text = "hello rust world";
    let mut char_count: HashMap<char, i32> = HashMap::new();

    for ch in text.chars() {
        // TODO: 使用 entry(ch).or_insert(0) 并递增计数
        // *... += 1;
    }

    println!("{:?}", char_count);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::HashMap;

fn main() {
    let text = "hello rust world";
    let mut char_count: HashMap<char, i32> = HashMap::new();

    for ch in text.chars() {
        *char_count.entry(ch).or_insert(0) += 1;
    }

    println!("{:?}", char_count);
}
```

**说明：** `entry(key)` 返回 `Entry` 枚举，`or_insert(default)` 在 key 不存在时插入默认值并返回 `&mut V`。通过解引用赋值 `*count += 1` 实现计数。这是 Rust 中 HashMap 计数的惯用写法。
</details>

---

### 练习 12-07: entry API — 分组存储

> 难度：⭐
> 用 entry + or_insert 将元素分组放入 Vec

补全代码，按首字母将单词分组。

```rust
use std::collections::HashMap;

fn main() {
    let words = vec!["apple", "banana", "avocado", "blueberry", "cherry", "cranberry"];
    let mut groups: HashMap<char, Vec<&str>> = HashMap::new();

    for word in &words {
        // TODO: 获取首字母，用 entry + or_insert 将单词加入对应组
        // let first = ...;
        // groups.entry(first)....push(*word);
    }

    println!("{:?}", groups);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::HashMap;

fn main() {
    let words = vec!["apple", "banana", "avocado", "blueberry", "cherry", "cranberry"];
    let mut groups: HashMap<char, Vec<&str>> = HashMap::new();

    for word in &words {
        let first = word.chars().next().unwrap();
        groups.entry(first).or_insert(Vec::new()).push(word);
    }

    println!("{:?}", groups);
}
```

**说明：** `or_insert(Vec::new())` 在键不存在时插入空 Vec，然后返回 `&mut Vec`，可以直接调用 `push`。这种模式常用于分组操作。
</details>

---

### 练习 12-08: entry 的 and_modify + or_insert

> 难度：⭐⭐
> 链式调用：存在则修改，不存在则插入

补全代码，使用 `and_modify` 配合 `or_insert` 实现库存更新。

```rust
use std::collections::HashMap;

fn main() {
    let mut inventory: HashMap<&str, i32> = HashMap::new();
    inventory.insert("苹果", 10);
    inventory.insert("香蕉", 5);

    // 进货操作：苹果进货 20 箱，橙子进货 8 箱（橙子原本不存在）
    // TODO: 使用 entry("苹果").and_modify(|v| *v += 20).or_insert(20);
    // TODO: 使用 entry("橙子").and_modify(|v| *v += 8).or_insert(8);

    println!("{:?}", inventory);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::HashMap;

fn main() {
    let mut inventory: HashMap<&str, i32> = HashMap::new();
    inventory.insert("苹果", 10);
    inventory.insert("香蕉", 5);

    inventory.entry("苹果").and_modify(|v| *v += 20).or_insert(20);
    inventory.entry("橙子").and_modify(|v| *v += 8).or_insert(8);

    println!("{:?}", inventory);
}
```

**说明：** `and_modify(|v| ...)` 仅在 key 存在时执行闭包修改值，`or_insert` 仅在 key 不存在时执行。两者可以链式调用，形成"存在则修改，不存在则插入"的语义。
</details>

---

### 练习 12-09: entry 链式操作进阶

> 难度：⭐⭐
> 用 entry API 实现多条件更新

补全代码，对于学生成绩，如果分数低于 60 则改为 60（及格），如果不存在则插入 60。

```rust
use std::collections::HashMap;

fn main() {
    let mut grades: HashMap<&str, i32> = HashMap::new();
    grades.insert("Alice", 55);
    grades.insert("Bob", 78);
    grades.insert("Charlie", 42);

    // 将所有不及格（<60）的分数提到 60，不存在的学生也插入 60
    // TODO: 对每个学生调用 grades.entry(name)...
    for name in &["Alice", "Bob", "Charlie", "David"] {
        // 补全
    }

    println!("{:?}", grades);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::HashMap;

fn main() {
    let mut grades: HashMap<&str, i32> = HashMap::new();
    grades.insert("Alice", 55);
    grades.insert("Bob", 78);
    grades.insert("Charlie", 42);

    for name in &["Alice", "Bob", "Charlie", "David"] {
        grades.entry(name)
            .and_modify(|v| { if *v < 60 { *v = 60; } })
            .or_insert(60);
    }

    println!("{:?}", grades);
}
```

**说明：** `and_modify` 的闭包内可以包含复杂逻辑。这里先判断是否小于 60，再决定是否修改。`or_insert(60)` 对不存在的 key 插入及格分。链式调用使得代码简洁且表达力强。
</details>

---

### 练习 12-10: 词频统计

> 难度：⭐⭐⭐
> entry API 综合应用：统计一篇文章中每个单词的出现次数，并筛选高频词

补全代码，统计文章中的单词频率，并输出出现次数大于等于 2 的单词。

```rust
use std::collections::HashMap;

fn main() {
    let article = "the quick brown fox jumps over the lazy dog the fox is quick";
    
    // 1. 将文章分割为单词（按空格）
    // 2. 用 HashMap 统计每个单词出现次数（使用 entry API）
    // 3. 筛选出出现次数 >= 2 的单词并打印

    // TODO: 补全代码

    // 预期输出（顺序可能不同）：
    // the: 3
    // quick: 2
    // fox: 2
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::HashMap;

fn main() {
    let article = "the quick brown fox jumps over the lazy dog the fox is quick";
    let mut freq: HashMap<&str, i32> = HashMap::new();

    for word in article.split_whitespace() {
        *freq.entry(word).or_insert(0) += 1;
    }

    for (word, count) in &freq {
        if *count >= 2 {
            println!("{word}: {count}");
        }
    }
}
```

**说明：** `split_whitespace()` 按空白字符分割字符串。`entry(word).or_insert(0)` 是 Rust 词频统计的标准模式。最后遍历 HashMap 筛选高频词。
</details>

---

### 练习 12-11: 自定义 Key — 元组作为 Key

> 难度：⭐
> 元组实现了 `Hash + Eq`，可以直接作为 HashMap 的键

补全代码，使用 `(i32, i32)` 元组作为 Key 记录坐标点。

```rust
use std::collections::HashMap;

fn main() {
    // TODO: 创建 HashMap<(i32, i32), &str>，记录坐标对应的地名
    // let mut locations = HashMap::new();

    // 插入数据：(0, 0) -> "原点", (1, 0) -> "东", (0, 1) -> "北"
    // 然后查询 (0, 0) 并打印

    println!("{:?}", locations.get(&(0, 0)));
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::HashMap;

fn main() {
    let mut locations: HashMap<(i32, i32), &str> = HashMap::new();
    locations.insert((0, 0), "原点");
    locations.insert((1, 0), "东");
    locations.insert((0, 1), "北");

    println!("{:?}", locations.get(&(0, 0)));
}
```

**说明：** 只要类型实现了 `Hash` 和 `Eq` trait，就可以作为 HashMap 的 Key。元组在元素类型都实现 `Hash + Eq` 时会自动派生这两个 trait。
</details>

---

### 练习 12-12: 自定义 Key — newtype 模式

> 难度：⭐
> 用元组结构体包装现有类型作为 Key

补全代码，定义一个 `StudentId` 结构体作为 HashMap 的 Key。

```rust
use std::collections::HashMap;

// TODO: 定义 StudentId 结构体（元组结构体），包含一个 u32
// 需要派生 Hash 和 Eq

fn main() {
    let mut roster: HashMap<StudentId, &str> = HashMap::new();
    roster.insert(StudentId(1001), "Alice");
    roster.insert(StudentId(1002), "Bob");

    // 查询学号 1001
    if let Some(name) = roster.get(&StudentId(1001)) {
        println!("1001 => {name}");
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
struct StudentId(u32);

fn main() {
    let mut roster: HashMap<StudentId, &str> = HashMap::new();
    roster.insert(StudentId(1001), "Alice");
    roster.insert(StudentId(1002), "Bob");

    if let Some(name) = roster.get(&StudentId(1001)) {
        println!("1001 => {name}");
    }
}
```

**说明：** newtype 模式通过 `#[derive(Hash, Eq, PartialEq)]` 让包装类型可作为 Key。`PartialEq` 是 `Eq` 的父 trait，两者都需要实现。注意 `Eq` 没有方法，仅作为标记 trait 表示等价关系满足自反性、对称性和传递性。
</details>

---

### 练习 12-13: 自定义 Key — 结构体作为 Key

> 难度：⭐⭐
> 定义包含多个字段的结构体作为 HashMap 的 Key

补全代码，`Person` 作为 Key，存储其年龄信息。

```rust
use std::collections::HashMap;

// TODO: 定义 Person 结构体，包含 name: String 和 birth_year: u16
// 派生适当的 trait

fn main() {
    let mut age_map: HashMap<Person, u8> = HashMap::new();
    
    // TODO: 创建两个 Person 实例作为 Key 插入
    // Person { name: "Alice".to_string(), birth_year: 1993 } -> 31
    // Person { name: "Bob".to_string(), birth_year: 1995 } -> 29

    // 查询 Alice 的年龄并打印
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
struct Person {
    name: String,
    birth_year: u16,
}

fn main() {
    let mut age_map: HashMap<Person, u8> = HashMap::new();
    age_map.insert(
        Person { name: "Alice".to_string(), birth_year: 1993 },
        31,
    );
    age_map.insert(
        Person { name: "Bob".to_string(), birth_year: 1995 },
        29,
    );

    let alice = Person { name: "Alice".to_string(), birth_year: 1993 };
    if let Some(age) = age_map.get(&alice) {
        println!("Alice 的年龄: {age}");
    }
}
```

**说明：** 结构体作为 Key 需要派生 `Hash`、`Eq`、`PartialEq`。注意 `String` 本身已经实现了 `Hash + Eq`，所以包含 `String` 字段的结构体可以自动派生。查询时也需传入 `&Person` 引用。
</details>

---

### 练习 12-14: 枚举作为 Key

> 难度：⭐⭐
> 枚举也可以作为 HashMap 的 Key

补全代码，使用枚举 `Direction` 作为 Key 存储方向描述。

```rust
use std::collections::HashMap;

// TODO: 定义 Direction 枚举，包含 North, South, East, West 四个变体
// 派生必要的 trait

fn main() {
    // TODO: 创建 HashMap<Direction, &str>，存储每个方向的描述
    // 例如 North -> "北", South -> "南", East -> "东", West -> "西"

    // 遍历并打印
    for (dir, desc) in &map {
        println!("{dir:?}: {desc}");
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let mut map: HashMap<Direction, &str> = HashMap::new();
    map.insert(Direction::North, "北");
    map.insert(Direction::South, "南");
    map.insert(Direction::East, "东");
    map.insert(Direction::West, "西");

    for (dir, desc) in &map {
        println!("{dir:?}: {desc}");
    }
}
```

**说明：** 枚举同样可以派生 `Hash + Eq` 作为 Key。枚举变体不带数据时是最简单的 Key 类型之一。派生 `Debug` 可以方便地打印枚举值。
</details>

---

### 练习 12-15: 自定义 Key 综合 — 坐标点分组

> 难度：⭐⭐⭐
> 综合使用自定义 Key 和 entry API 实现坐标点分组

定义 `Point` 结构体，根据坐标值模 3 的结果分组（即按 `(x % 3, y % 3)` 分组）。

```rust
use std::collections::HashMap;

// TODO: 定义 Point 结构体，包含 x: i32, y: i32
// 派生必要的 trait

fn main() {
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 1 },
        Point { x: 3, y: 0 },
        Point { x: 4, y: 4 },
        Point { x: 6, y: 6 },
        Point { x: 2, y: 2 },
    ];

    // TODO: 创建分组 HashMap: HashMap<(i32, i32), Vec<Point>>
    // 按 (p.x % 3, p.y % 3) 分组
    // 使用 entry API

    // 打印每个分组
    for (key, group) in &groups {
        println!("模 ({}, {}): {:?}", key.0, key.1, group);
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 1 },
        Point { x: 3, y: 0 },
        Point { x: 4, y: 4 },
        Point { x: 6, y: 6 },
        Point { x: 2, y: 2 },
    ];

    let mut groups: HashMap<(i32, i32), Vec<Point>> = HashMap::new();

    for p in &points {
        let key = (p.x % 3, p.y % 3);
        groups.entry(key).or_insert(Vec::new()).push(Point { x: p.x, y: p.y });
    }

    for (key, group) in &groups {
        println!("模 ({}, {}): {:?}", key.0, key.1, group);
    }
}
```

**说明：** 本例综合使用了自定义 Key 类型 `Point`、元组作为分组键、entry API 进行分组。注意 `or_insert(Vec::new())` 在分组场景中的典型用法。
</details>

---

### 练习 12-16: BTreeMap 基本操作

> 难度：⭐
> BTreeMap 与 HashMap 的不同在于键是有序的

补全代码，创建 BTreeMap 并演示自动排序特性。

```rust
use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::new();
    map.insert("banana", 3);
    map.insert("apple", 5);
    map.insert("cherry", 2);

    // TODO: 遍历 map，观察输出的顺序与插入顺序是否相同

    // TODO: 使用 .keys() 获取所有键的集合，收集为 Vec 并打印
    // let keys: Vec<&&str> = ...;
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::new();
    map.insert("banana", 3);
    map.insert("apple", 5);
    map.insert("cherry", 2);

    for (k, v) in &map {
        println!("{k}: {v}");
    }

    let keys: Vec<&&str> = map.keys().collect();
    println!("{:?}", keys);
}
```

**说明：** BTreeMap 按键的自然顺序（按 `Ord` trait 的实现）排序存储。字符串按字典序排列，因此输出为 apple → banana → cherry，与插入顺序无关。
</details>

---

### 练习 12-17: BTreeMap 的 first/last 操作

> 难度：⭐
> 利用 BTreeMap 的有序性获取最小/最大键值

补全代码，获取 BTreeMap 中的最小和最大键值对。

```rust
use std::collections::BTreeMap;

fn main() {
    let mut scores: BTreeMap<&str, i32> = BTreeMap::new();
    scores.insert("Charlie", 88);
    scores.insert("Alice", 95);
    scores.insert("Bob", 72);

    // TODO: 使用 .first_key_value() 获取最小键值对
    // TODO: 使用 .last_key_value() 获取最大键值对

    // println!("最低分: {:?}", ...);
    // println!("最高分: {:?}", ...);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::BTreeMap;

fn main() {
    let mut scores: BTreeMap<&str, i32> = BTreeMap::new();
    scores.insert("Charlie", 88);
    scores.insert("Alice", 95);
    scores.insert("Bob", 72);

    if let Some((name, score)) = scores.first_key_value() {
        println!("最低分: {name} = {score}");
    }
    if let Some((name, score)) = scores.last_key_value() {
        println!("最高分: {name} = {score}");
    }
}
```

**说明：** `.first_key_value()` 和 `.last_key_value()` 返回 `Option<(&K, &V)>`，分别获取最小和最大键值对。这两个方法是 BTreeMap 特有的，HashMap 没有此功能。
</details>

---

### 练习 12-18: BTreeMap 范围查询

> 难度：⭐⭐
> 使用 `range()` 方法查询指定范围内的键值对

补全代码，查询 BTreeMap 中成绩在特定范围内的学生。

```rust
use std::collections::BTreeMap;

fn main() {
    let mut scores: BTreeMap<&str, i32> = BTreeMap::new();
    scores.insert("Alice", 95);
    scores.insert("Bob", 72);
    scores.insert("Charlie", 88);
    scores.insert("David", 61);
    scores.insert("Eve", 55);

    // TODO: 使用 .range() 查询成绩在 70 到 90 之间（含 70，不含 90）的学生
    // for (name, score) in scores.range(...) {
    //     println!("{name}: {score}");
    // }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::BTreeMap;

fn main() {
    let mut scores: BTreeMap<&str, i32> = BTreeMap::new();
    scores.insert("Alice", 95);
    scores.insert("Bob", 72);
    scores.insert("Charlie", 88);
    scores.insert("David", 61);
    scores.insert("Eve", 55);

    for (name, score) in scores.range(70..90) {
        println!("{name}: {score}");
    }
}
```

**说明：** `range(range)` 使用 Rust 的范围语法 `start..end`（左闭右开）。也可以使用 `..=end`（全闭）、`start..`（从 start 到末尾）等。BTreeMap 的 `range` 方法利用了 B 树的有序性高效地定位范围。
</details>

---

### 练习 12-19: BTreeMap 范围查询进阶

> 难度：⭐⭐
> 使用 `range` 配合自定义范围进行数据切片

补全代码，按成绩段统计各段人数。

```rust
use std::collections::BTreeMap;

fn main() {
    let scores = vec![95, 72, 88, 61, 55, 82, 91, 43, 78, 67];

    // TODO: 将分数存入 BTreeMap<i32, i32>，键为分数，值为出现次数
    // TODO: 使用 range 统计：
    //   - 不及格 (<60): 人数
    //   - 及格 (60-79): 人数  
    //   - 良好 (80-89): 人数
    //   - 优秀 (>=90): 人数

    // println!("不及格: {}", ...);
    // println!("及格:   {}", ...);
    // println!("良好:   {}", ...);
    // println!("优秀:   {}", ...);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::BTreeMap;

fn main() {
    let scores = vec![95, 72, 88, 61, 55, 82, 91, 43, 78, 67];
    let mut map: BTreeMap<i32, i32> = BTreeMap::new();

    for s in &scores {
        *map.entry(*s).or_insert(0) += 1;
    }

    let fail: i32 = map.range(..60).map(|(_, c)| c).sum();
    let pass: i32 = map.range(60..80).map(|(_, c)| c).sum();
    let good: i32 = map.range(80..90).map(|(_, c)| c).sum();
    let excellent: i32 = map.range(90..=100).map(|(_, c)| c).sum();

    println!("不及格: {fail}");
    println!("及格:   {pass}");
    println!("良好:   {good}");
    println!("优秀:   {excellent}");
}
```

**说明：** 先统计每个分数出现次数，再用 `range` 分段求和。`range(..60)` 表示从最小到 60（不含），`range(90..=100)` 表示 90 到 100（含）。`map(|(_, c)| c)` 提取次数值，`sum()` 求和。
</details>

---

### 练习 12-20: 综合挑战 — 学生成绩报告

> 难度：⭐⭐⭐
> 综合运用 HashMap、entry API、BTreeMap 实现学生成绩统计与排序

给定学生成绩列表，完成以下任务：
1. 使用 HashMap 统计每个学生的总分
2. 使用 BTreeMap 按总分从高到低排序（提示：BTreeMap 键有序，但若要按值排序需要先转换）
3. 输出成绩排名，并标注等级（>=90: A, >=80: B, >=70: C, >=60: D, <60: F）

```rust
use std::collections::{BTreeMap, HashMap};

fn main() {
    // 学生各科成绩：姓名 -> [(科目, 分数), ...]
    let records = vec![
        ("Alice", vec![("数学", 85), ("语文", 92), ("英语", 78)]),
        ("Bob",   vec![("数学", 70), ("语文", 65), ("英语", 72)]),
        ("Charlie", vec![("数学", 95), ("语文", 88), ("英语", 91)]),
        ("David", vec![("数学", 45), ("语文", 55), ("英语", 60)]),
        ("Eve",  vec![("数学", 88), ("语文", 90), ("英语", 85)]),
    ];

    // TODO:
    // 1. 用 HashMap 统计每个学生的总分
    // 2. 用 BTreeMap 实现按总分降序排列
    //    （提示：可以以 (-总分) 为键，或以 (总分, 姓名) 为键）
    // 3. 输出排名，格式：
    //     第1名: Charlie - 274分 - A
    //     第2名: Eve - 263分 - A
    //     ...
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::collections::{BTreeMap, HashMap};

fn grade(total: i32) -> &'static str {
    match total {
        270..=300 => "A",
        240..=269 => "B",
        210..=239 => "C",
        180..=209 => "D",
        _ => "F",
    }
}

fn main() {
    let records = vec![
        ("Alice", vec![("数学", 85), ("语文", 92), ("英语", 78)]),
        ("Bob",   vec![("数学", 70), ("语文", 65), ("英语", 72)]),
        ("Charlie", vec![("数学", 95), ("语文", 88), ("英语", 91)]),
        ("David", vec![("数学", 45), ("语文", 55), ("英语", 60)]),
        ("Eve",  vec![("数学", 88), ("语文", 90), ("英语", 85)]),
    ];

    // 1. 计算每个学生的总分
    let mut total_scores: HashMap<&str, i32> = HashMap::new();
    for (name, subjects) in &records {
        let sum: i32 = subjects.iter().map(|(_, score)| score).sum();
        total_scores.insert(name, sum);
    }

    // 2. 按总分降序排序：用 (-总分, 姓名) 作为 BTreeMap 键
    let mut ranking: BTreeMap<(i32, &str), i32> = BTreeMap::new();
    for (name, total) in &total_scores {
        ranking.insert((-total, name), *total);
    }

    // 3. 输出排名
    for (i, ((_, name), total)) in ranking.iter().enumerate() {
        println!("第{}名: {} - {}分 - {}", i + 1, name, total, grade(*total));
    }
}
```

**说明：** 本题综合了 HashMap（统计总分）、entry API（可选）、BTreeMap（自动排序）。技巧在于用 `(-总分, 姓名)` 作为 BTreeMap 的键以实现降序——BTreeMap 默认按升序排列，取负数后较大的总分对应的键反而更小，从而实现降序效果。当总分相同时，姓名作为第二排序键确保结果唯一。`grade` 函数使用 `match` 匹配范围将总分转换为等级。
</details>
