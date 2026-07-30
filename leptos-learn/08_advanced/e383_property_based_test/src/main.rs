// ============================================================
// 练习 e383: 基于属性的测试 — 用属性测试方法验证状态转换
//
// 核心知识点:
//   - 属性测试方法论：不变式（invariant）、恒等性质（identity）
//   - 用循环 + 随机值模拟属性测试
//   - 计数器状态转换的数学性质
//   - 列表操作的函数式恒等性质
//
// 难度: ⭐⭐ (关键位置有 TODO，补全约 30%)
// ============================================================

use leptos::prelude::*;

// ===== 核心功能函数 =====

/// 计数器加减操作（纯函数）
pub fn counter_transform(current: i32, operation: &str, value: i32) -> i32 {
    // TODO 1: 实现计数器状态转换
    // - "increment" → current + value
    // - "decrement" → current - value
    // - "reset" → 0
    // - 其他 → current（不变）
    current // 占位
}

/// 列表去重后再去重应等价于一次去重（幂等性）
pub fn dedup_idempotent<T: Clone + PartialEq>(items: &[T]) -> Vec<T> {
    // TODO 2: 实现去重（保持顺序，保留首次出现的元素）
    // 提示：使用 Vec 并检查 contains
    items.to_vec() // 占位
}

/// 列表过滤：两次相同条件的过滤等价于一次过滤
pub fn filter_twice_equivalence<T: Clone>(items: &[T], predicate: fn(&T) -> bool) -> Vec<T> {
    // TODO 3: 实现两次过滤 → 结果应等价于一次过滤
    // 先 filter 一次，再 filter 一次（相同条件）
    items.to_vec() // 占位
}

#[component]
fn Exercise() -> impl IntoView {
    let counter = RwSignal::new(0);

    view! {
        <div>
            <h2>"🧪 基于属性的测试演示"</h2>
            <p>"计数: " {move || counter.get()}</p>
            <button on:click=move |_| counter.update(|c| *c += 1)>"+1"</button>
            <button on:click=move |_| counter.update(|c| *c -= 1)>"-1"</button>
            <button on:click=move |_| counter.set(0)>"重置"</button>

            <p style="margin-top:16px;font-size:0.85em;color:#666;">
                "在终端运行 `cargo test -p e383_property_based_test_answer` 查看属性测试结果"
            </p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}

// ===== 属性测试 =====

#[cfg(test)]
mod tests {
    use super::*;

    // ========== 计数器性质测试 ==========

    /// 性质 1: 递增后再递减相同值应回到原始值
    #[test]
    fn test_counter_increment_decrement_roundtrip() {
        for initial in -10..=10 {
            for delta in 1..=5 {
                // TODO 4: 验证 counter_transform 的往返性质
                // after = counter_transform(initial, "increment", delta)
                // result = counter_transform(after, "decrement", delta)
                // assert_eq!(result, initial, "往返性质失败: initial={}, delta={}", initial, delta);
            }
        }
    }

    /// 性质 2: 重置后计数归零
    #[test]
    fn test_counter_reset_always_zero() {
        for initial in [-100, -1, 0, 1, 100, 9999] {
            // TODO 5: 验证无论初始值如何，reset 后结果都是 0
        }
    }

    // ========== 列表操作性质测试 ==========

    /// 性质 3: 去重操作是幂等的（两次去重 = 一次去重）
    #[test]
    fn test_dedup_idempotent() {
        let test_cases = vec![
            vec![1, 2, 3],
            vec![1, 1, 1],
            vec![1, 2, 1, 2, 3, 3],
            vec![],
        ];
        for case in test_cases {
            // TODO 6: 验证 dedup_idempotent 的幂等性
            // 第一次去重结果 = dedup_idempotent(&case)
            // 第二次去重结果 = dedup_idempotent(&第一次结果)
            // 两次结果应相等
        }
    }

    /// 性质 4: 两次相同条件的过滤等价于一次过滤
    #[test]
    fn test_filter_twice_equivalence() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        fn is_even(x: &i32) -> bool { x % 2 == 0 }
        fn is_positive(x: &i32) -> bool { *x > 0 }

        // TODO 7: 验证两次相同条件过滤 = 一次过滤
        // 条件 1: 偶数（is_even）
        // 条件 2: 正数（is_positive）
    }

    /// 性质 5: 过滤 + 去重 vs 去重 + 过滤（交换律检查）
    #[test]
    fn test_filter_dedup_commutative_approx() {
        let data = vec![3, 1, 2, 1, 3, 4, 2, 5];
        fn is_odd(x: &i32) -> bool { x % 2 != 0 }

        // TODO 8: 验证先过滤再去重 = 先去重再过滤（顺序交换）
        // 注意：这里近似验证，因为两种情况可能产生相同结果
    }
}
