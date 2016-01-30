考虑如下**一级拉马努金级数(Level-1 Ramanujan Series)**:

$$
\frac{1}{\pi }=\sum_{k=0}^∞(-1)^{k+1}\binom{2 k}{k} \binom{3 k}{k} \binom{6 k}{3 k}c^{-k-\frac{1}{2}} (a k+b)
$$

其中 a, b, c 是三个未知整数, j 函数定义如下, 也是个整数


$$
\begin{aligned}
c &= j\left(\frac{1+\sqrt{-τ}}{2}\right)\\
j(τ)&=\left({\frac {E_{4}(\tau )}{η^{8}(τ)}}\right)^{3}={\frac {1}{q}}+744+196884q+21493760q^{2}+\cdots 
\end{aligned}
$$

选取如下参数

$$
\left\{
\begin{aligned}
a &= 163×40133016 \\
b &= 163096908 \\
τ &= 163\\
c &= 640320^3\\ 
\end{aligned}
\right.
$$

可以看到没有任何根号, 都是精确的数字.

接着想办法解决一大堆阶乘求和的精度问题.

---

令

$$
f(k)=(-1)^{k+1}\binom{2 k}{k} \binom{3 k}{k} \binom{6 k}{3 k} c^{-k}
$$

注意到两式之比可以表达成:

$$
\frac{f(n)}{f(n-1)}=\frac{8}{c}\frac{(6 n-1) (6 n-3) (6 n-5)}{n^3}
$$

于是有

$$
\begin{aligned}
\frac{1}{π}
=c^{-\frac{1}{2}}\left(b+\sum_{k=1}^∞\prod_{n=1}^k\left(\frac{8}{c}×\frac{(6 n-1) (6 n-3) (6 n-5)}{n^3}\right) (a k+b)\right)
\end{aligned}
$$

又令:

$$
\begin{aligned}
P(p,q)&=\prod_{j=p}^{q-1}(6 j-1) (6 j-3) (6 j-5)\\
Q(p,q)&=\prod_{j=p}^{q-1}c× \left(\frac{j}{2}\right)^3 \\
S(p,q)&=\sum_{k=p}^{q-1}\frac{P(p,k+1)}{Q(p,k+1)}(a k+b)\\
R(p,q)&=Q(p,q) S(p,q)\\
\end{aligned}
$$


以上可以重写为递推定义:

$$
\begin{aligned}
P(p,q)&=P(p,r)×P(r,q)\\
Q(p,q)&=Q(p,r)×Q(r,q)\\
S(p,q)&=S(p,r)+\frac{P(p,r)}{Q(p,r)}S(r,q)\\
R(p,q)&=Q(r,q)R(p,r) + P(p,r)R(r,q)\\
\end{aligned}
$$

边界条件为:

$$
\begin{aligned}
P(p,p+1)&=(6p-1)(6p-3)(6p-5)\\
Q(p,p+1)&=\frac{cp^3}{8}\\
S(p,p+1)&=\frac{P(p,p+1)}{Q(p,p+1)}(a p + b)\\
R(p,p+1)&=P(p,p+1)(a p + b)\\
\end{aligned}
$$

这个才是可编程计算的无精度损失版本.

---

把 S 的表达式代入原式有:

$$
\frac{1}{π}=c^{-\frac{1}{2}}\left(b+S(1,∞)\right)
$$

翻个身

$$
π=\frac{\sqrt{c}}{b+S(1,∞)}
$$

当然计算机没法算到无穷 直接截断即可, 对于 t = 163 就是:

$$
π≈\frac{5122560\sqrt{10005}}{163096908+S(1,n)}
$$

一般编程计算的是这个公式才对.

---

但是这个和原始的 `chudnovsky.c` 里的参数还是有区别, 这是因为


$$
\mathrm{GCD}(163 × 40133016, 163096908) = 12
$$

所有参数可以约去 12, 于是有:

$$
π≈\frac{426880\sqrt{10005}}{13591409+S(1,n)}
$$

这个就是超级计算机上跑的能算一万亿的版本了.

当然你这个大数乘法的得分块计算, 单机没法算.

