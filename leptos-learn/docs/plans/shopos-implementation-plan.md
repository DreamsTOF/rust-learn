# ShopOS Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use subagent-driven-development to implement this plan task-by-task.

**Goal:** Create a complete 40-step ShopOS e-commerce admin backend with exercise and answer projects

**Architecture:** Two cargo-leptos SSR projects (shopos_answer/ and shopos/) under leptos-learn workspace. Answer project is complete with all features. Exercise project has TODO comments for students.

**Tech Stack:** Leptos 0.9 (nightly) + Thaw UI + leptos_router + leptos-use + Server Functions + SQLite + Axum

---

### Execution Strategy

1. Create both project directory structures (Cargo.toml, index.html, style.css, main.rs, lib.rs, app.rs)
2. Write complete answer project code (all 40 steps) - Phase by Phase
3. Derive exercise project from answer (add TODO comments)

### File Map

```
leptos-learn/projects/
├── shopos_answer/           # Answer project (complete, no TODOs)
│   ├── Cargo.toml
│   ├── index.html
│   ├── style.css
│   ├── src/
│   │   ├── main.rs          # Server entry
│   │   ├── lib.rs           # App + routes
│   │   ├── app.rs           # Root component
│   │   ├── layout.rs        # Layout + sidebar + topbar
│   │   ├── state.rs         # App state (auth, settings)
│   │   ├── error.rs         # Error types
│   │   ├── components/
│   │   │   ├── nav/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── sidebar.rs
│   │   │   │   └── topbar.rs
│   │   │   ├── mod.rs
│   │   │   └── product_form.rs
│   │   ├── pages/
│   │   │   ├── mod.rs
│   │   │   ├── home.rs
│   │   │   ├── auth/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── login.rs
│   │   │   │   └── register.rs
│   │   │   ├── products/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── list.rs
│   │   │   │   ├── detail.rs
│   │   │   │   └── import.rs
│   │   │   ├── categories.rs
│   │   │   ├── cart.rs
│   │   │   ├── checkout.rs
│   │   │   ├── orders/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── list.rs
│   │   │   │   └── detail.rs
│   │   │   ├── user/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── profile.rs
│   │   │   │   ├── addresses.rs
│   │   │   │   └── security.rs
│   │   │   ├── admin/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── coupons.rs
│   │   │   │   ├── returns.rs
│   │   │   │   ├── invoices.rs
│   │   │   │   ├── audit.rs
│   │   │   │   ├── settings.rs
│   │   │   │   └── reports.rs
│   │   │   ├── dashboard.rs
│   │   │   └── reconciliation.rs
│   │   ├── server/
│   │   │   ├── mod.rs
│   │   │   ├── categories.rs
│   │   │   ├── products.rs
│   │   │   ├── auth.rs
│   │   │   ├── users.rs
│   │   │   ├── addresses.rs
│   │   │   ├── orders.rs
│   │   │   ├── coupons.rs
│   │   │   ├── returns.rs
│   │   │   ├── shipments.rs
│   │   │   ├── invoices.rs
│   │   │   ├── dashboard.rs
│   │   │   ├── notifications.rs
│   │   │   ├── audit.rs
│   │   │   ├── settings.rs
│   │   │   └── reports.rs
│   │   ├── db/
│   │   │   ├── mod.rs
│   │   │   └── seed.rs
│   │   ├── hooks/
│   │   │   ├── mod.rs
│   │   │   └── cart.rs
│   │   └── i18n/
│   │       ├── mod.rs
│   │       ├── zh-CN.rs
│   │       └── en-US.rs
│   └── migrations/
│       └── 001_initial.sql
├── shopos/                  # Exercise project (with TODOs)
│   ├── Cargo.toml
│   ├── index.html
│   ├── style.css
│   └── src/
│       └── ...              # Same structure with TODO comments
```

### Phase 1: Scaffold & Database (A-01~A-05)
- A-01: Project scaffold, Cargo.toml, main.rs, lib.rs, app.rs
- A-02: Layout + Thaw UI theme
- A-03: Sidebar + Topbar navigation
- A-04: Database schema + migrations
- A-05: Seed data + health check

### Phase 2: Product Management (A-06~A-11)
- A-06: Category CRUD with tree
- A-07: Product list with pagination
- A-08: Search + multi-condition filter
- A-09: Create product form
- A-10: Edit product + SKU management
- A-11: Product detail + image gallery

### Phase 3: User & Auth (A-12~A-16)
- A-12: Register + password hash
- A-13: Login + Session management
- A-14: User profile
- A-15: Address management
- A-16: Password change + security

### Phase 4: Cart & Orders (A-17~A-22)
- A-17: Cart with localStorage
- A-18: Cart page
- A-19: Checkout page
- A-20: Order create (transaction)
- A-21: Order list + filter
- A-22: Order detail + state machine

### Phase 5: Operations (A-23~A-27)
- A-23: Coupon admin
- A-24: Coupon validation
- A-25: Return/refund
- A-26: Shipping tracking
- A-27: Invoice management

### Phase 6: Analytics & Notifications (A-28~A-31)
- A-28: Dashboard
- A-29: SSE notifications
- A-30: Audit log
- A-31: System settings

### Phase 7: Engineering (A-32~A-35)
- A-32: i18n
- A-33: Dark mode + responsive
- A-34: Tests
- A-35: Docker + CI

### Phase 8: Advanced (A-36~A-40)
- A-36: Batch import
- A-37: Reviews
- A-38: Reports export
- A-39: Payment reconciliation
- A-40: API docs
