# MIRRA P1: witness and corrected N=14 budget

Date: 2026-09-03. Status: **derived review candidate; not a release certificate or attestation**.
No consensus source, frozen constant, proof pin, certificate, or RC5 Wasm was changed.

## Answers

At `u=16373744595`, `x_q32=-u`, the unchanged RC5 Python kernel and its
Decimal-90 oracle both return **94906266**. An independent rational interval
also proves correct rounding at this input. The unrounded ideal output is
`94906265.624420515904782034603574...`; the actual absolute error is
`0.375579484095217965396425...` output ulp, not zero. Zero disagreement with
the rounded oracle and zero real approximation error are different statements.

The true discrete maximum argument magnitude is attained at this witness:

`780414346016661 / 2251799813685248`

`= 0.346573590278192344982244321727193892002105712890625`.

It is slightly **below**, not exactly equal to, `ln(2)/2`. The exact half-way
peak cannot be reached: `L64/2 == 2 (mod 4)`, whereas `u*2^32 - k*L64` is
divisible by four. The witness's residual misses the half-way peak by
32,841,010 raw Q64 units. Near-peak arguments are nevertheless enough to
invalidate the old half-sized domain bound.

| Computation | Result, output ulp | Interpretation |
|---|---:|---|
| Old formula with only its `t_max` corrected to the true discrete maximum | `0.5000001002531813107583981692115...` | Diagnostic recomputation; not the repaired proof |
| Fresh derivation below, using the safe analytic domain enclosure | `0.5000000425665135563095684854886...` | Conservative all-domain bound, pending independent review |
| Exact outward nine-decimal ceiling of the fresh bound | `0.500000043` | Numerical ceiling, not an issued certificate |

The stronger result is possible without changing N: a sharper Horner bound
retains its factorial denominators, each k-case retains its `2^-k` output
scale, and k=0 has **exact** argument conversion and no logarithm split.
This is a new derivation, not reproduction or rehabilitation of the old proof
artifact. Subject to independent acceptance of this derivation, even the old
decimal claim `0.500000065` is loose enough; the defective supporting artifact
still must be superseded and correctly bound to the release.

## Mathematical contract

Let `S=2^32`, `Q=2^56`, `L=12786308645202655660`, `N=14`, and `h=1/(2Q)`.
For integer `0 <= u <= 16S`, define

```
k = RNE(u*S / L)
r = u*S - k*L
a = -RNE(r/256) / Q
```

The actual series argument is `a`; the integer called `v` in the source is
`a*Q`. The objective is an absolute bound on
`abs(kernel(-u) - S*exp(-u/S))`, not universal correct rounding.

### 1. Range and log enclosure

RNE implies `abs(r) <= L/2`; the argument rounding contributes at most h.
Use the safe, slightly larger rational enclosure

```
T = L/(512Q) + h
  = 3196577161300663979 / 9223372036854775808
  = 0.3465735902799726616532391409997...
```

The exact maximum above is established by testing both integer neighbors of
every nearest-integer k boundary, plus the two domain endpoints: 48 candidates.
Within a constant-k interval the residual is affine and rounded residual is
monotone, so its absolute maximum lies at an endpoint. The endpoints give
`0 <= k <= 23`; this is not the old check's unconditional `or k_max == 23`.

Independently enclose ln2 with

```
l = sum(1/(j*2^j), j=1..240)
l <= ln2 <= l + 1/(241*2^240) = Ulog
D = max(abs(L/2^64-l), abs(L/2^64-Ulog))
```

For the tail, replace each remaining `1/j` by `1/241` and sum the geometric
series. A separate `2*atanh(1/3)` series enclosure lies inside this interval.
All endpoints and D are exact fractions; D is an upper bound, not a
rounded Decimal approximation to an irrational difference.

The series identities are standard; see [NIST DLMF logarithmic series](https://dlmf.nist.gov/4.6)
and [NIST DLMF exponential series](https://dlmf.nist.gov/4.2#E19).
The MIRRA-specific inequalities and numerical bounds below are derived here.

### 2. Horner rounding

The normalized computed stages obey, for n=N down to 1,

`s_n = 1 + (a/n)*s_(n+1) + e_n`, with `s_(N+1)=1` and `abs(e_n)<=h`.

The exact stages have the same recurrence without e_n, ending at the degree-14
exponential polynomial. Expanding the error recurrence gives the bound

```
H = h * sum(T^j/j!, j=0..13)
  = 9.813077866773594159969758...e-18
```

Each earlier error is multiplied by `a/1`, then `a/2`, and so on; retaining
these denominators gives `T^j/j!`, not just a geometric contraction.

### 3. Truncation

For `abs(a)<=T`, take absolute values in the exponential-series tail.
Its first omitted term is `T^15/15!`; every subsequent ratio is at most
`T/16`. Therefore

```
R = (T^15/15!) / (1-T/16)
  = 9.771088264312980045609068...e-20
```

This supplies a direct geometric majorant of the tail. No approximate
Lagrange constant or unproved helper-output comparison is required.

### 4. Argument conversion and logarithm split

Write `delta=L/2^64-ln2` and `b=-r/2^64-k*delta`. Then

`exp(-u/S) = 2^-k * exp(b)` and `abs(a-b) <= h+kD`.

For k=0, `r=u*S` is divisible by 256. Its Q56 conversion is exact and
`delta` is multiplied by zero: **a=b=-u/S exactly**. Set A_0=0;
for k=1..23 set A_k=h+kD.

For each A_k<1, the positive exponential series gives
`exp(A_k)-1 <= A_k/(1-A_k)`. Since `a<=T`,

`abs(exp(a)-exp(b)) <= E * A_k/(1-A_k) = G_k`,

where E is a rational upper bound for exp(T). The code obtains E from a
degree-48 positive series plus its geometric tail:

`E = P_48(T) + (T^49/49!)/(1-T/50)`.

This covers the split and conversion multiplicatively, without dropping a
positive exponential remainder or cross term.

### 5. Output scaling and final RNE

The final integer division is exactly a rounding of `S*2^-k*s_1`.
It contributes at most one-half output ulp. For each possible k,

```
B_k = 1/2 + (S/2^k) * (H + R + G_k)
```

All 24 B_k are computed and compared as exact fractions. The largest **bound**
is at k=0; this is not a claim about where the actual maximum kernel error occurs.

| k | B_k, output ulp (decimal display) |
|---|---:|
| 0 | `0.5000000425665135563095684854886...` |
| 1 | `0.5000000423914800565619901134014...` |
| 2 | `0.5000000212131395397610898990008...` |
| 3 | `0.5000000106152695256205923706506...` |
| 23 | `0.5000000000000102894445804801376...` |

At k=0, the output-unit budget is Horner `4.214684851089403...e-8`,
truncation `4.196650454155365...e-10`, argument error zero, plus final RNE 0.5.
The JSON records the complete exact fraction for the global bound and every
per-k component. Displayed decimal truncations are not themselves certified
upper bounds; the integer-computed ceiling `0.500000043` is outward.

### 6. Integer headroom

An invariant for every normalized Horner stage is
`abs(s_n) <= M=(1+h)/(1-T)`: the recurrence bounds the next stage by
`1+T*M+h=M`. The code checks `Q^2*T*M < 2^127` and `Q*M < 2^127` exactly.
Thus the signed i128 Horner product and accumulator bounds cover every input.
It also checks `1/E > H+R`, proving the final series is positive.
The range-reduction operands are below 2^69 and the final shift is at most 47.

## Validation performed and remaining scope

- Verified the unchanged RC5 source archive, Wasm, and original scripts ZIP hashes.
- Compared an independent Fraction-rounding kernel model to the actual RC5
  Python kernel at 2,048 points: 48 boundary/endpoints plus 2,000 seeded inputs.
- At those points, checked argument rounding and exact polynomial-vs-Horner
  error against the analytic enclosure; all passed.
- Verified the witness against Decimal-90 and independently enclosed its ideal
  value using a degree-96 positive exponential series and reciprocal intervals.
  Interval width is below `5.383e-90` output ulp and wholly within the rounding bin.
- Verified the exact result and deterministic JSON generation.

This is a source-grounded analytic proof candidate with reproducible rational
calculation, not a Lean/Coq proof, third-party sign-off, full-domain enumeration,
new Rust/Wasm execution, or public ICP certificate validation. No rerun of the
151,640-point historical sweep is claimed. The kernel remains N=14 throughout.

Before release: independently review this derivation; bind it to exact source,
constants and checker; issue versioned proof evidence; decide certificate
wording; rerun downstream assumptions/TV checks; update any compiled proof
pin and measure the resulting Wasm; complete runtime and upstream sign-off.
Original RC5 remains unchanged and not release-ready.
