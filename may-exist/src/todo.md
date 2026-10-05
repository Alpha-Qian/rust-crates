`eq` 是我随手取的，来自 equality（类型相等证明），因为 `Is<F, ConstTrue>` 本质上就是「F = ConstTrue」这个等式的证据。但这个名字确实不好，有三个问题：

1. **和 `PartialEq::eq` 撞名**。你的 `PartialEq` 实现里就有 `fn eq(&self, other: &Self)`，读到 `BoolWit::True(eq) => self.ref_some(eq) == other.ref_some(eq)` 时，`eq` 是变量还是方法一眼分不清。
2. **读起来像布尔值或比较结果**，容易被误认为 `a == b` 的返回值，而它其实是一个 ZST 的证明。
3. **丢失了信息**：它没说明「等于什么」。在 `xor` 里还出现了 `f`、`r`、`eq` 混用，更乱。

## 更好的命名

按「这个证明说的是什么」来命名，我推荐：

| 场景 | 名字 | 含义 |
|---|---|---|
| `BoolWit::True(..)` 分支里的 `Is<F, ConstTrue>` | `is_true` | F = True |
| `BoolWit::False(..)` 分支里的 `Is<F, ConstFalse>` | `is_false` | F = False |
| 两个变量的情况 | `f_true` / `r_false` 等 | 谁是什么 |
| 方法里需要的 `Is<F, ConstTrue>` 参数 | `is_true` 或 `proof` | 与上面一致 |
| 由 `project` 得到的目标等式 | `to_xor` 等，或 `sel_eq` | 表明投影的是什么 |

这样代码读起来像自然语言：

```rust
match F::WIT {
    BoolWit::True(is_true)   => StaticOption::from_some(is_true, self.ref_some(is_true)),
    BoolWit::False(is_false) => StaticOption::from_none(is_false),
}
```

原语也同步改：

```rust
const fn from_some(is_true: Is<F, ConstTrue>, v: T) -> Self { ... }
const fn into_none(self, is_false: Is<F, ConstFalse>) { ... }
```

`xor` 里则能区分清楚：

```rust
(BoolWit::True(f_true), BoolWit::False(r_false)) => {
    let sel = f_true.project2::<XorSelFn<T>, _, _>(r_false);  // 存储类型的等式
    let v = self.into_some(f_true);
    other.into_none(r_false);
    StaticOption(sel.flip().cast(v))
}
```

## 更进一步：把名字写进类型里

如果觉得每次要靠变量名来区分，可以在 `const_bool` 里给 `BoolWit` 的两个变体用具名别名：

```rust
pub type IsTrue<F>  = Is<F, ConstTrue>;
pub type IsFalse<F> = Is<F, ConstFalse>;

pub enum BoolWit<F> { True(IsTrue<F>), False(IsFalse<F>) }
```

原语签名就变成 `fn from_some(is_true: IsTrue<F>, v: T)`，类型和变量名互相印证，报错信息里也更易读。

## 小建议

- `flip().cast(..)` 这个模式出现很多次，可以在 `Is` 上加一个 `cast_back(self, b: B) -> A`（内部就是 `flip().cast`），
- 或者让 `from_some` 这样的原语承担，减少调用处出现 `sel`、`eq` 这类临时变量。
- 如果以后证明函数变多（`not_not`、`or_comm` 之类），证明的名字也建议统一约定：命题叫 `xxx_eq`，
- 假设叫 `h`，由 `WIT` 得到的分支证据叫 `is_true`/`is_false`，避免整个库里到处是 `eq`。