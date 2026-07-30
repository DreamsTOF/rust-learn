// ============================================================
// 练习 e383: 基于属性的测试 — 用属性测试方法验证状态转换
//
// 核心知识点:
//   - 属性测试方法论：不变式（invariant）、恒等性质（identity）
//   - 用循环 + 随机值模拟属性测试
//   - 计数器状态转换的数学性质
//   - 列表操作的函数式恒等性质
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

// ===== 核心功能函数 =====

/// 计数器加减操作（纯函数）
pub fn counter_transform(current: i32, operation: &str, value: i32) -> i32 {
    match operation {
        "increment" => current + value,
        "decrement" => current - value,
        "reset" => 0,
        _ => current,
    }
}

/// 列表去重后再去重应等价于一次去重（幂等性）
pub fn dedup_idempotent<T: Clone + PartialEq>(items: &[T]) -> Vec<T> {
    let mut result = Vec::new();
    for item in items {
        if !result.contains(item) {
            result.push(item.clone());
        }
    }
    result
}

/// 列表过滤：两次相同条件的过滤等价于一次过滤
pub fn filter_twice_equivalence<T: Clone>(items: &[T], predicate: fn(&T) -> bool) -> Vec<T> {
    let first: Vec<T> = items.iter().filter(|x| predicate(*x)).cloned().collect();
    first.into_iter().filter(|x| predicate(&x)).collect()
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

    /// 性质 1: 递增后再递减相同值应回到原始值（往返性质）
    #[test]
    fn test_counter_increment_decrement_roundtrip() {
        for initial in -10..=10 {
            for delta in 1..=5 {
                let after_increment = counter_transform(initial, "increment", delta);
                let result = counter_transform(after_increment, "decrement", delta);
                assert_eq!(
                    result, initial,
                    "往返性质失败: initial={}, delta={}",
                    initial, delta
                );
            }
        }
    }

    /// 性质 2: 重置后计数总是 0
    #[test]
    fn test_counter_reset_always_zero() {
        let test_values = [-100, -1, 0, 1, 100, 9999];
        for initial in test_values {
            assert_eq!(
                counter_transform(initial, "reset", 0),
                0,
                "重置后应该总是 0, initial={}",
                initial
            );
        }
    }

    /// 性质 3: 递增后一定大于原始值（当 delta > 0）
    #[test]
    fn test_increment_increases() {
        for current in -50..=50 {
            for delta in 1..=20 {
                let result = counter_transform(current, "increment", delta);
                assert!(
                    result > current,
                    "递增后应大于原始值: current={}, delta={}, result={}",
                    current, delta, result
                );
            }
        }
    }

    /// 性质 4: 递减后一定小于原始值（当 delta > 0）
    #[test]
    fn test_decrement_decreases() {
        for current in -50..=50 {
            for delta in 1..=20 {
                let result = counter_transform(current, "decrement", delta);
                assert!(
                    result < current,
                    "递减后应小于原始值: current={}, delta={}, result={}",
                    current, delta, result
                );
            }
        }
    }

    // ========== 列表操作性质测试 ==========

    /// 性质 5: 去重操作是幂等的（两次去重 = 一次去重）
    #[test]
    fn test_dedup_idempotent() {
        let test_cases = vec![
            vec![1, 2, 3],
            vec![1, 1, 1],
            vec![1, 2, 1, 2, 3, 3],
            vec![],
            vec![5, 5, 5, 3, 3, 1],
        ];
        for case in test_cases {
            let once = dedup_idempotent(&case);
            let twice = dedup_idempotent(&once);
            assert_eq!(
                once, twice,
                "去重幂等性失败: case={:?}, once={:?}, twice={:?}",
                case, once, twice
            );
        }
    }

    /// 性质 6: 去重后元素数量 ≤ 原始元素数量
    #[test]
    fn test_dedup_length_non_increasing() {
        let test_cases = vec![
            vec![1, 2, 3],
            vec![1, 1, 1],
            vec![1, 2, 1, 2, 3, 3],
            vec![],
        ];
        for case in test_cases {
            let result = dedup_idempotent(&case);
            assert!(
                result.len() <= case.len(),
                "去重后不应增加长度: case.len={}, result.len={}",
                case.len(),
                result.len()
            );
        }
    }

    /// 性质 7: 两次相同条件的过滤等价于一次过滤
    #[test]
    fn test_filter_twice_equivalence() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        fn is_even(x: &i32) -> bool { x % 2 == 0 }
        fn is_positive(x: &i32) -> bool { *x > 0 }

        // 验证偶数过滤
        let even_once = data.iter().filter(|x| is_even(*x)).cloned().collect::<Vec<_>>();
        let even_twice = data.iter()
            .filter(|x| is_even(*x))
            .cloned()
            .filter(|x| is_even(x))
            .collect::<Vec<_>>();
        assert_eq!(even_once, even_twice, "偶数过滤两次应等价于一次");

        // 验证正数过滤
        let pos_once = data.iter().filter(|x| is_positive(*x)).cloned().collect::<Vec<_>>();
        let pos_twice = data.iter()
            .filter(|x| is_positive(*x))
            .cloned()
            .filter(|x| is_positive(x))
            .collect::<Vec<_>>();
        assert_eq!(pos_once, pos_twice, "正数过滤两次应等价于一次");
    }

    /// 性质 8: 先过滤后去重 与 先去重后过滤 结果相同（交换律）
    #[test]
    fn test_filter_dedup_commutative() {
        let data = vec![3, 1, 2, 1, 3, 4, 2, 5];
        fn is_odd(x: &i32) -> bool { x % 2 != 0 }

        // 先过滤再去重
        let filtered_then_dedup = {
            let filtered: Vec<i32> = data.iter().filter(|x| is_odd(*x)).cloned().collect();
            dedup_idempotent(&filtered)
        };

        // 先去重再过滤
        let deduped_then_filter = {
            let deduped = dedup_idempotent(&data);
            deduped.into_iter().filter(|x| is_odd(x)).collect::<Vec<_>>()
        };

        assert_eq!(
            filtered_then_dedup, deduped_then_filter,
            "过滤与去重应满足交换律: filtered_then_dedup={:?}, deduped_then_filter={:?}",
            filtered_then_dedup, deduped_then_filter
        );
    }
}
