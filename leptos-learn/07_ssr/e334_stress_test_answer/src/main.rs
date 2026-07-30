// ============================================================
// Exercise e334 — Answer: Stress Test
//
// Core: k6 / Artillery load testing, concurrent connections,
//       stress/spike/soak scenarios, results analysis
// ============================================================

use leptos::prelude::*;

const K6_SCRIPT: &str = "\
import http from 'k6/http';
import { check, sleep } from 'k6';

export const options = {
  stages: [
    { duration: '30s', target: 20 },   // 逐步增加到 20 VUs
    { duration: '1m',  target: 100 },  // 逐步增加到 100 VUs
    { duration: '30s', target: 0 },    // 逐步缩减到 0 VUs
  ],
  thresholds: {
    http_req_duration: ['p(95)<500'],  // 95% 请求在 500ms 内
    http_req_failed: ['rate<0.01'],    // 错误率 < 1%
  },
};

export default function () {
  const res = http.get('http://localhost:3000/');
  check(res, { 'status is 200': (r) => r.status === 200 });
  sleep(1);
}";

const ARTILLERY_CONFIG: &str = "\
# artillery.yml
config:
  target: 'http://localhost:3000'
  phases:
    - duration: 60
      arrivalRate: 5       # 每秒 5 个新用户
      rampTo: 50            # 逐步增加到每秒 50
    - duration: 120
      arrivalRate: 50       # 保持 50 RPS 120 秒
    - duration: 30
      arrivalRate: 0        # 逐步降为 0
scenarios:
  - name: 'SSR page load'
    flow:
      - get:
          url: '/'
      - think: 1";

const TEST_SCENARIOS: &str = "\
# 三种核心测试场景

## 1. 压力测试 (Stress Test)
   目标: 找出系统的极限拐点
   方法: 每 10 秒增加 10 VUs，直到错误率 > 5%
   指标: 观察 p95 延迟何时开始急剧上升

## 2. 尖峰测试 (Spike Test)
   目标: 突发流量下的韧性
   方法: 0 VU → 5 秒内跃升到 500 VU → 保持 30 秒 → 骤降
   指标: 恢复时间、限流/降级是否正确触发

## 3. 浸泡测试 (Soak Test)
   目标: 长时间运行的内存/CPU 泄漏
   方法: 100 VUs 持续 8-24 小时
   指标: 内存增长趋势、GC 频率、响应时间漂移";

const RESULTS_INTERPRETATION: &str = "\
# k6 输出解读

http_req_duration..............: avg=120ms  p(50)=85ms  p(95)=340ms  p(99)=820ms
http_reqs......................: 12,432  34.2/s
http_req_failed................: 0.3%
iterations.....................: 12,432  34.2/s
vus............................: 5    min=0  max=87

# 关键指标
  p(95) < 500ms   → ✅ 通过
  错误率 < 1%     → ✅ 通过
  最大 VUs = 87   → 未达到目标 100 VUs（可能需要扩容）";

#[component]
fn ConfigBlock(title: &'static str, code: &'static str) -> impl IntoView {
    view! {
        <div style="margin-bottom: 20px;">
            <h3 style="margin-bottom: 8px;">{title}</h3>
            <pre style="background:#1e1e1e;color:#d4d4d4;padding:12px;border-radius:6px;font-size:13px;overflow-x:auto;margin:0;">
                {code}
            </pre>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div style="font-family:system-ui,sans-serif;max-width:900px;margin:20px auto;padding:0 16px;">
            <h2>"⚡ 负载测试与压力测试"</h2>
            <p style="color:#666;margin-bottom:20px;">
                "使用 k6 和 Artillery 对 SSR 应用进行负载测试，分析并发连接下的性能表现。"
            </p>

            <ConfigBlock title="k6 测试脚本" code={K6_SCRIPT} />
            <ConfigBlock title="Artillery 配置文件" code={ARTILLERY_CONFIG} />
            <ConfigBlock title="测试场景" code={TEST_SCENARIOS} />
            <ConfigBlock title="结果解读" code={RESULTS_INTERPRETATION} />

            <div style="margin-top:20px;padding:12px;background:#f0f8ff;border-radius:6px;font-size:13px;color:#333;">
                <strong>"💡 负载测试策略: "</strong>
                "在 CI/CD 管道中集成 k6 测试，每次部署前自动运行压力测试。"
                "设置性能阈值（p95 延迟、错误率）作为发布门禁。"
                "使用持续浸泡测试发现内存泄漏和 GC 问题。"
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
