# 21 unsafe Rust

Rust 的核心承诺是：**如果你不碰 `unsafe`，编译器保证内存安全。** `unsafe` 关键字将安全责任从编译器移交给你——它并不是关闭安全检查，而是允许你做五种编译器无法验证的操作：解引用裸指针、调用 unsafe 函数（包括 FFI）、实现 unsafe trait、访问/修改可变静态变量、访问 union 字段。C++ 程序员会发现，Rust 的 `unsafe` 块正是 C++ 中默认的指针操作——区别在于 Rust 显式标出了不安全边界。

---

### 练习 21-01: 创建裸指针

> 难度：⭐⭐
> 类似 C++ 中取地址 `&v`，但 Rust 区分 `*const T` 和 `*mut T`

补全代码，从引用创建 `*const i32` 和 `*mut i32` 裸指针。

```rust
fn main() {
    let x: i32 = 42;
    let y: &i32 = &x;

    // TODO: 从引用创建裸指针
    // let ptr_const: *const i32 = ...;
    // let ptr_mut:   *mut i32   = ...;

    // 下面两行仅用于避免警告，不解引用
    let _ = ptr_const;
    let _ = ptr_mut;
    println!("裸指针创建成功");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let x: i32 = 42;
    let y: &i32 = &x;

    let ptr_const: *const i32 = x as *const i32;
    let ptr_mut:   *mut i32   = &x as *const i32 as *mut i32;

    let _ = ptr_const;
    let _ = ptr_mut;
    println!("裸指针创建成功");
}
```

**说明：** 通过 `as *const T` 或 `as *mut T` 将引用转为裸指针。`*mut T` 需要先转 `*const T` 再转 `*mut T`，因为从不可变引用直接转可变裸指针是不安全的——但创建裸指针本身是安全的，危险在解引用时。

</details>

---

### 练习 21-02: 安全地解引用裸指针

> 难度：⭐⭐

在 `unsafe` 块中解引用裸指针，读取其指向的值。注意：创建指针安全，解引用必须在 `unsafe` 中。

```rust
fn main() {
    let x = 100i32;
    let ptr: *const i32 = &x as *const i32;

    // TODO: 在 unsafe 块中解引用 ptr 并打印值
    // 补全下面的 unsafe 块
    unsafe {
        // let val = ...;
        // println!("指针指向的值: {}", val);
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let x = 100i32;
    let ptr: *const i32 = &x as *const i32;

    unsafe {
        let val = *ptr;
        println!("指针指向的值: {}", val);
    }
}
```

**说明：** `*ptr` 解引用裸指针必须在 `unsafe` 块中。即使指针是合法的，编译器也无法验证——它把这个责任交给了你。

</details>

---

### 练习 21-03: 通过裸指针修改值（挑战）

> 难度：⭐⭐

使用 `*mut i32` 裸指针修改变量的值。挑战：必须同时确保指针有效且变量未提前释放。

```rust
fn main() {
    let mut x = 10i32;
    let ptr: *mut i32 = &mut x as *mut i32;

    // TODO: 在 unsafe 块中通过 ptr 将 x 的值改为 42

    // 不要修改下面这行
    println!("x = {}", x); // 应输出 x = 42
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let mut x = 10i32;
    let ptr: *mut i32 = &mut x as *mut i32;

    unsafe {
        *ptr = 42;
    }

    println!("x = {}", x);
}
```

**说明：** 通过 `*mut T` 裸指针写入需要 unsafe 块。安全保证完全由开发者负责——如果 x 在解引用前被 drop，这就是一个悬挂指针（类似 C++ 的 dangling pointer）。

</details>

---

### 练习 21-04: 调用 unsafe 函数填空

> 难度：⭐⭐
> 调用 unsafe 函数必须在 unsafe 块中

标准库提供了 `std::slice::from_raw_parts` 这个 unsafe 函数，它可以从裸指针和长度创建切片。补全代码将裸指针还原为切片。

```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];
    let ptr = arr.as_ptr();
    let len = arr.len();

    // TODO: 调用 from_raw_parts 创建 &[i32] 切片
    // 需要 unsafe 块
    // let slice: &[i32] = ...;

    // 不要修改下面这行
    println!("slice = {:?}", slice); // slice = [1, 2, 3, 4, 5]
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];
    let ptr = arr.as_ptr();
    let len = arr.len();

    let slice: &[i32] = unsafe { std::slice::from_raw_parts(ptr, len) };

    println!("slice = {:?}", slice);
}
```

**说明：** `std::slice::from_raw_parts` 是 unsafe 的，因为它要求调用者保证指针非空、对齐、且指向的内存已初始化。这里 ptr 来自合法数组，是安全的——但编译器无法验证。

</details>

---

### 练习 21-05: 补全自定义 unsafe 函数

> 难度：⭐⭐

补全 `unsafe fn deref` 函数，使其解引用传入的 `*const i32` 并返回值。然后在 `safe_wrapper` 中安全地调用它。

```rust
/// 解引用裸指针（调用者必须保证指针有效）
unsafe fn deref(ptr: *const i32) -> i32 {
    // TODO: 解引用 ptr 并返回其值
}

fn safe_wrapper() -> i32 {
    let x = 99;
    // TODO: 安全地调用 deref
}

fn main() {
    println!("{}", safe_wrapper()); // 输出 99
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
unsafe fn deref(ptr: *const i32) -> i32 {
    *ptr
}

fn safe_wrapper() -> i32 {
    let x = 99;
    unsafe { deref(&x as *const i32) }
}

fn main() {
    println!("{}", safe_wrapper());
}
```

**说明：** `unsafe fn` 本身就是一个安全边界——调用者必须用 `unsafe` 块或 `unsafe fn` 来调用它。这里 `safe_wrapper` 用 unsafe 块调用 `deref`，但对外暴露了安全接口。

</details>

---

### 练习 21-06: 统计 vector（挑战）

> 难度：⭐⭐

实现一个函数 `total`，它接收 `*const i32` 和长度 `len`，计算所有元素的和。调用者必须保证指针有效。

```rust
// TODO: 定义 unsafe fn total，接收 *const i32 和长度 usize，返回 i32
// 在函数体内解引用指针构造切片

fn main() {
    let v = vec![10, 20, 30, 40];
    let ptr = v.as_ptr();
    let len = v.len();

    let result = unsafe { total(ptr, len) };
    println!("total = {}", result); // total = 100
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
unsafe fn total(ptr: *const i32, len: usize) -> i32 {
    let slice: &[i32] = std::slice::from_raw_parts(ptr, len);
    slice.iter().sum()
}

fn main() {
    let v = vec![10, 20, 30, 40];
    let ptr = v.as_ptr();
    let len = v.len();

    let result = unsafe { total(ptr, len) };
    println!("total = {}", result);
}
```

**说明：** `from_raw_parts` 要求调用者保证切片生命周期内底层内存有效且不变。这里 `v` 在 `total` 调用期间存活且不修改，满足条件。如果 `v` 在 `total` 执行前被 drop，则行为未定义。

</details>

---

### 练习 21-07: extern "C" 调用 C 标准库函数

> 难度：⭐⭐⭐
> 类似 C++ 中 `extern "C"` 声明外部函数

填空：使用 `extern "C"` 块声明 C 标准库的 `abs` 函数，然后在 Rust 中调用它。

```rust
// TODO: 在 extern "C" 块中声明 abs 函数
// 签名：int abs(int)

fn main() {
    let n = -42;
    // TODO: 调用 abs(n)，结果应等于 n.abs()（但在 C 中计算）
    // let result = ...;
    println!("abs({}) = {}", n, result);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
extern "C" {
    fn abs(x: i32) -> i32;
}

fn main() {
    let n = -42;
    let result = unsafe { abs(n) };
    println!("abs({}) = {}", n, result);
}
```

**说明：** `extern "C"` 声明外部函数，FFI 调用始终是 unsafe 的——Rust 无法验证 C 函数的实现是否正确。这里 `abs` 是标准 C 函数，行为明确。

</details>

---

### 练习 21-08: 链接 C 字符串函数 strlen

> 难度：⭐⭐⭐

补全代码，调用 C 标准库的 `strlen` 计算字符串长度（不含结尾 `\0`）。

```rust
// TODO: 声明 extern "C" 块中的 strlen
// 注意 C 字符串以 \0 结尾

fn main() {
    let s = b"Hello\0"; // C 风格字符串（含结尾 NUL）
    // TODO: 将 s.as_ptr() 作为 *const i8 (即 *const c_char) 传入 strlen
    // 并用 unsafe 调用
    // println!("length = {}", len);
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
extern "C" {
    fn strlen(s: *const i8) -> usize;
}

fn main() {
    let s = b"Hello\0";
    let len = unsafe { strlen(s.as_ptr() as *const i8) };
    println!("length = {}", len); // 5
}
```

**说明：** C 字符串以 NUL 结尾。`b"Hello\0"` 确保末尾有 `\0`。`strlen` 遍历直到遇到 `\0`，如果调用者忘记终止符则是未定义行为——Rust 无法阻止这种错误。

</details>

---

### 练习 21-09: 封装 FFI 调用为安全接口（挑战）

> 难度：⭐⭐⭐

将 21-08 中的 `strlen` 封装成一个安全的 Rust 函数 `safe_strlen`，使其接收 `&str` 并返回长度，内部使用 unsafe 调用 FFI。注意不能用 `s.len()` 偷懒——必须真的经过 FFI 调用。

```rust
extern "C" {
    fn strlen(s: *const i8) -> usize;
}

// TODO: 编写 safe_strlen，接收 &str，内部通过 FFI 计算长度
fn safe_strlen(s: &str) -> usize {
    // 提示：需要先确保字符串以 \0 结尾
    // 可以用 CString（来自 std::ffi::CString）
}

fn main() {
    let s = "Rust FFI";
    let len = safe_strlen(s);
    println!("length = {}", len); // 8
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::ffi::CString;

extern "C" {
    fn strlen(s: *const i8) -> usize;
}

fn safe_strlen(s: &str) -> usize {
    let c_str = CString::new(s).expect("CString 不能含 NUL 字节");
    unsafe { strlen(c_str.as_ptr()) }
}

fn main() {
    let s = "Rust FFI";
    let len = safe_strlen(s);
    println!("length = {}", len);
}
```

**说明：** `CString::new` 在末尾追加 `\0` 并检查字符串内是否已含 NUL 字节（会返回错误）。封装后，调用者无需接触 unsafe —— 安全边界在 `safe_strlen` 内部。

</details>

---

### 练习 21-10: 实现 Send trait（unsafe trait）

> 难度：⭐⭐⭐
> unsafe trait 要求实现者手动保证线程安全

标准库中 `Send` 是一个 unsafe trait：实现者必须保证类型可以安全地跨线程传递。以下 `MyBox` 包装了 `*mut i32`，编译器默认不会为它实现 `Send`。补全 `unsafe impl Send`。

```rust
struct MyBox(*mut i32);

// TODO: unsafe impl Send for MyBox {}
// 为什么这是安全的？因为 MyBox 是独占所有权，且只在单线程场景使用。

fn main() {
    let mut x = 42;
    let b = MyBox(&mut x as *mut i32);

    // 如果 MyBox 未实现 Send，下面这行会编译报错
    let handle = std::thread::spawn(move || {
        println!("MyBox 已跨线程发送");
        let _ = b;
    });
    handle.join().unwrap();
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
struct MyBox(*mut i32);

unsafe impl Send for MyBox {}

fn main() {
    let mut x = 42;
    let b = MyBox(&mut x as *mut i32);

    let handle = std::thread::spawn(move || {
        println!("MyBox 已跨线程发送");
        let _ = b;
    });
    handle.join().unwrap();
}
```

**说明：** 实现 `Send` 是 unsafe 的，因为编译器无法验证你的类型是否真的线程安全。这里 `MyBox` 只包含一个裸指针，且通过 `move` 转移所有权，没有共享，因此是安全的。但如果 `MyBox` 内部有非原子引用计数等，则可能不安全。

</details>

---

### 练习 21-11: 实现 Sync trait

> 难度：⭐⭐⭐

`Sync` 是另一个 unsafe trait：实现者必须保证 `&T` 可以安全地跨线程共享。补全代码，为自定义的 `Counter` 实现 `Sync`（内部用 `Mutex` 保护）。

```rust
use std::sync::Mutex;

struct Counter {
    value: Mutex<i32>,
}

// TODO: unsafe impl Sync for Counter {}
// 提示：Mutex<i32> 已经实现了 Sync

fn main() {
    let c = Counter { value: Mutex::new(0) };
    let shared = &c;
    // 如果 Counter 未实现 Sync，下面这行会编译报错
    std::thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| {
                let mut v = shared.value.lock().unwrap();
                *v += 1;
            });
        }
    });
    println!("final: {}", *c.value.lock().unwrap());
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::sync::Mutex;

struct Counter {
    value: Mutex<i32>,
}

unsafe impl Sync for Counter {}

fn main() {
    let c = Counter { value: Mutex::new(0) };
    let shared = &c;
    std::thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| {
                let mut v = shared.value.lock().unwrap();
                *v += 1;
            });
        }
    });
    println!("final: {}", *c.value.lock().unwrap());
}
```

**说明：** `Counter` 的所有内部字段（`Mutex<i32>`）都实现了 `Sync`，因此 `impl Sync` 是安全的——但编译器不会自动推导，因为你自定义了结构体。`unsafe impl Sync` 告诉编译器："我保证这是线程安全的。"

</details>

---

### 练习 21-12: 自定义迭代器与 unsafe trait（挑战）

> 难度：⭐⭐⭐

裸指针迭代器：为 `PtrIter` 实现 `Iterator`，但 `std::iter::TrustedLen` 是 unsafe trait。补全 `unsafe impl TrustedLen`。

```rust
struct PtrIter {
    start: *const i32,
    end: *const i32,
}

impl Iterator for PtrIter {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.start == self.end {
            None
        } else {
            let val = unsafe { *self.start };
            self.start = unsafe { self.start.add(1) };
            Some(val)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.end as usize - self.start as usize) / std::mem::size_of::<i32>();
        (len, Some(len))
    }
}

// TODO: unsafe impl std::iter::TrustedLen for PtrIter {}

fn main() {
    let arr = [10, 20, 30];
    let iter = PtrIter {
        start: arr.as_ptr(),
        end: unsafe { arr.as_ptr().add(3) },
    };
    let collected: Vec<i32> = iter.collect();
    println!("{:?}", collected); // [10, 20, 30]
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::iter::TrustedLen;

struct PtrIter {
    start: *const i32,
    end: *const i32,
}

impl Iterator for PtrIter {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.start == self.end {
            None
        } else {
            let val = unsafe { *self.start };
            self.start = unsafe { self.start.add(1) };
            Some(val)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.end as usize - self.start as usize) / std::mem::size_of::<i32>();
        (len, Some(len))
    }
}

unsafe impl TrustedLen for PtrIter {}

fn main() {
    let arr = [10, 20, 30];
    let iter = PtrIter {
        start: arr.as_ptr(),
        end: unsafe { arr.as_ptr().add(3) },
    };
    let collected: Vec<i32> = iter.collect();
    println!("{:?}", collected);
}
```

**说明：** `TrustedLen` 是 unsafe trait，实现者承诺 `size_hint` 返回的上限精确且不会变化。这里 `PtrIter` 基于固定长度数组，满足条件。注意：如果实现不正确（例如长度计算有误），可能会导致 `collect` 产生未定义行为。

</details>

---

### 练习 21-13: transmute 基础——字节重新解释

> 难度：⭐⭐⭐
> 类似 C++ 的 `reinterpret_cast` 和 C++20 的 `std::bit_cast`

`std::mem::transmute` 将一种类型的位模式直接重新解释为另一种类型。补全代码，将 `u32` 的字节重新解释为 `i32`。

```rust
fn main() {
    let x: u32 = 0xFFFFFFFF;
    // TODO: 使用 transmute 将 x 转为 i32
    // let y: i32 = ...;
    println!("y = {}", y); // y = -1

    // 在 C++ 中同样 reinterpret_cast<int32_t>(0xFFFFFFFF) == -1
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let x: u32 = 0xFFFFFFFF;
    let y: i32 = unsafe { std::mem::transmute(x) };
    println!("y = {}", y);
}
```

**说明：** `transmute` 是 unsafe 的——它要求源类型和目标类型大小相同（否则编译报错）。0xFFFFFFFF 作为 u32 是 4294967295，作为补码 i32 是 -1。这是纯粹位模式转换，不做任何检查。

</details>

---

### 练习 21-14: transmute 将引用转为裸指针

> 难度：⭐⭐⭐

`transmute` 也可以用于引用到指针的转换。补全代码，将 `&i32` 转为 `*const i32`（虽然用 `as` 更安全，但这里练习 transmute）。

```rust
fn main() {
    let x = 42i32;
    let r: &i32 = &x;

    // TODO: 使用 transmute 将 &i32 转为 *const i32
    // let ptr: *const i32 = ...;

    unsafe {
        println!("值: {}", *ptr);
    }
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
fn main() {
    let x = 42i32;
    let r: &i32 = &x;

    let ptr: *const i32 = unsafe { std::mem::transmute(r) };

    unsafe {
        println!("值: {}", *ptr);
    }
}
```

**说明：** 引用和裸指针在底层具有相同的大小（指针宽度），因此 `transmute` 可以转换。但更推荐的 Rust 风格是使用 `as` 转换（`r as *const i32`），因为 transmute 过于宽松——如果类型大小不匹配，编译期就会报错，但这仍是编译时检查而非运行时。

</details>

---

### 练习 21-15: transmute 的安全封装（挑战）

> 难度：⭐⭐⭐

编写一个安全的泛型函数 `bits_as`，使用 `transmute` 将 `A` 转换为 `B`（要求大小相同），并用 `assert_eq!` 做编译期大小检查。

```rust
use std::mem;

// TODO: 编写 safe 函数 bits_as<A, B>(val: A) -> B
// 要求：A 和 B 大小相同，否则 panic
// 使用 assert_eq!(size_of::<A>(), size_of::<B>()) 做运行时检查

fn main() {
    let x: u64 = 42;
    let y: i64 = bits_as::<u64, i64>(x);
    println!("y = {}", y); // y = 42

    // 下面的代码如果用 transmute 会编译错：大小不同
    // let _: u32 = bits_as::<u64, u32>(0u64); // panic 或编译错误
    println!("OK");
}
```

<details>
<summary>点击查看参考答案</summary>

```rust
use std::mem;

fn bits_as<A, B>(val: A) -> B {
    assert_eq!(mem::size_of::<A>(), mem::size_of::<B>());
    unsafe { mem::transmute::<A, B>(val) }
}

fn main() {
    let x: u64 = 42;
    let y: i64 = bits_as::<u64, i64>(x);
    println!("y = {}", y);

    // 如果取消注释下面这行，会在运行时 panic，因为大小不同
    // let _: u32 = bits_as::<u64, u32>(0u64);

    println!("OK");
}
```

**说明：** `transmute` 本身在大小不匹配时会在编译期报错，但这里我们演示通过 `assert_eq!` 做运行时防护来提供安全封装。实际项目中更推荐用 `as` 转换或安全 API（如 `u64::from_ne_bytes`）替代 transmute。

</details>

---

> **总结：** Rust 的 `unsafe` 并非"不安全"，而是"编译器信任你"——你承诺手动保证内存安全、类型有效、线程安全等不变量。unsafe 代码应尽量封装在小型安全接口内，对外隐藏裸指针和 transmute 等危险操作。原则：**unsafe 代码的漏洞应该少到可以被手动验证，而不是多到不值得检查。**
