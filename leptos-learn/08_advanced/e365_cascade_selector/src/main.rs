// ============================================================
// 练习 e365: cascade_selector — 级联选择器（省市联动）
//
// 核心知识点:
//   - 静态嵌套数据结构
//   - 信号驱动的级联更新
//   - select + option 渲染
//
// 难度: ⭐⭐ (关键位置有 TODO，补全约 50%)
// ============================================================

use leptos::prelude::*;

/// 行政区域节点
struct Area {
    name: &'static str,
    children: &'static [Area],
}

// 省市区静态数据
const PROVINCES: &[Area] = &[
    Area {
        name: "北京市",
        children: &[
            Area {
                name: "北京市",
                children: &[
                    Area { name: "海淀区", children: &[] },
                    Area { name: "朝阳区", children: &[] },
                    Area { name: "东城区", children: &[] },
                    Area { name: "西城区", children: &[] },
                    Area { name: "丰台区", children: &[] },
                ],
            },
        ],
    },
    Area {
        name: "广东省",
        children: &[
            Area {
                name: "广州市",
                children: &[
                    Area { name: "天河区", children: &[] },
                    Area { name: "越秀区", children: &[] },
                    Area { name: "海珠区", children: &[] },
                    Area { name: "白云区", children: &[] },
                ],
            },
            Area {
                name: "深圳市",
                children: &[
                    Area { name: "南山区", children: &[] },
                    Area { name: "福田区", children: &[] },
                    Area { name: "宝安区", children: &[] },
                ],
            },
            Area {
                name: "东莞市",
                children: &[
                    Area { name: "南城街道", children: &[] },
                    Area { name: "莞城街道", children: &[] },
                ],
            },
        ],
    },
    Area {
        name: "浙江省",
        children: &[
            Area {
                name: "杭州市",
                children: &[
                    Area { name: "西湖区", children: &[] },
                    Area { name: "上城区", children: &[] },
                    Area { name: "滨江区", children: &[] },
                    Area { name: "余杭区", children: &[] },
                ],
            },
            Area {
                name: "宁波市",
                children: &[
                    Area { name: "海曙区", children: &[] },
                    Area { name: "鄞州区", children: &[] },
                ],
            },
        ],
    },
    Area {
        name: "四川省",
        children: &[
            Area {
                name: "成都市",
                children: &[
                    Area { name: "武侯区", children: &[] },
                    Area { name: "锦江区", children: &[] },
                    Area { name: "青羊区", children: &[] },
                    Area { name: "金牛区", children: &[] },
                ],
            },
        ],
    },
];

/// 渲染一组 <option>
fn render_options(items: &[Area], selected_idx: Option<usize>) -> Vec<leptos::View> {
    items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let is_selected = selected_idx == Some(i);
            view! {
                <option value={i.to_string()} prop:selected={is_selected}>
                    {item.name}
                </option>
            }
        })
        .collect()
}

#[component]
fn Exercise() -> impl IntoView {
    let (province_idx, set_province_idx) = signal::<Option<usize>>(None);
    let (city_idx, set_city_idx) = signal::<Option<usize>>(None);
    let (district_idx, set_district_idx) = signal::<Option<usize>>(None);

    // TODO: 实现 current_province / current_city / current_district 派生信号
    // 通过 province_idx()/city_idx()/district_idx() 和 PROVINCES 数组获取当前选中项

    // TODO: 实现 cities() / districts() 派生信号
    // cities() 根据当前省份返回其 children，districts() 根据当前城市返回其 children

    // TODO: 实现 on_province_change / on_city_change / on_district_change
    // 使用 event_target_value(&ev) 获取 select 值，解析为 usize 后设置对应信号
    // 选择上级时必须清空下级信号（set_xxx.set(None)）

    view! {
        <div style="max-width: 500px; margin: 20px auto; font-family: sans-serif;">
            <h3>"省市区域级联选择器"</h3>

            <div style="display: flex; flex-direction: column; gap: 16px;">
                <!-- 省 -->
                <div>
                    <label style="display: block; margin-bottom: 4px; font-weight: bold; color: #555;">
                        "省份"
                    </label>
                    <select on:change={on_province_change}
                        style="width: 100%; padding: 8px; border: 1px solid #ccc;
                               border-radius: 4px; font-size: 1em;">
                        <option value="">"-- 请选择省份 --"</option>
                        {render_options(PROVINCES, province_idx())}
                    </select>
                </div>

                <!-- 市 -->
                <div>
                    <label style="display: block; margin-bottom: 4px; font-weight: bold; color: #555;">
                        "城市"
                    </label>
                    <select on:change={on_city_change}
                        disabled={move || province_idx().is_none()}
                        style="width: 100%; padding: 8px; border: 1px solid #ccc;
                               border-radius: 4px; font-size: 1em;">
                        <option value="">"-- 请选择城市 --"</option>
                        {move || render_options(cities(), city_idx())}
                    </select>
                </div>

                <!-- 区 -->
                <div>
                    <label style="display: block; margin-bottom: 4px; font-weight: bold; color: #555;">
                        "区/县"
                    </label>
                    <select on:change={on_district_change}
                        disabled={move || city_idx().is_none()}
                        style="width: 100%; padding: 8px; border: 1px solid #ccc;
                               border-radius: 4px; font-size: 1em;">
                        <option value="">"-- 请选择区/县 --"</option>
                        {move || render_options(districts(), district_idx())}
                    </select>
                </div>
            </div>

            <div style="margin-top: 24px; padding: 16px; background: #f5f5f5;
                        border-radius: 8px; text-align: center;">
                <p style="margin: 0; color: #666;">
                    "选择结果: "
                    <strong>
                        {move || {
                            let p = current_province().map(|a| a.name).unwrap_or("");
                            let c = current_city().map(|a| a.name).unwrap_or("");
                            let d = current_district().map(|a| a.name).unwrap_or("");
                            format!("{} {} {}", p, c, d)
                        }}
                    </strong>
                </p>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
