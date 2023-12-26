

$$
\begin{aligned}
π&=M_4+M_6\\
M_4&=\sum_{n=0}^{∞}(-1)^n2^{-6n-3}\left\{\frac{8}{4n+1},\frac{1}{4n+3}\right\}\\
&=2\arctan\frac{4}{7}\\
M_6&=\sum_{n=0}^{∞}(-1)^n2^{-6n-3}\left\{\frac{16}{6n+1},\frac{2}{6n+3},\frac{1}{6n+5}\right\}\\
&=\frac{2}{3}\left(4\arctan\frac{1}{2}+2\arctan\frac{2}{3}+\arctan\frac{1}{8}\right)
\end{aligned}
$$


```
M4=Sum[(-1)^n*2^(-3-6n){8/(1+4n),1/(3+4n)},{n,0,Infinity}]
M6=Sum[(-1)^n*2^(-3-6n){16/(1+6n),2/(3+6n),1/(5+6n)},{n,0,Infinity}]
Total[Flatten@{M4,M6}]//FullSimplify
```


---

$$
\begin{aligned}
π&=M_4+M_{10}\\
M_4&=\sum_{n=0}^{∞}(-1)^n2^{-6n-3}\left\{\frac{8}{4n+1},\frac{1}{4n+3}\right\}\\
&=2\arctan\frac{4}{7}\\
M_10&=\sum_{n=0}^{∞}(-1)^n2^{-6n-3}\left\{\frac{16}{6n+1},\frac{2}{6n+3},\frac{1}{6n+5}\right\}\\
&=\frac{2}{3}\left(4\arctan\frac{1}{2}+2\arctan\frac{2}{3}+\arctan\frac{1}{8}\right)


\end{aligned}
$$


```
M4=Sum[(-1)^n*2^(-3-6n){8/(1+4n),1/(3+4n)},{n,0,Infinity}]
M6=Sum[(-1)^n*2^(-3-6n){16/(1+6n),2/(3+6n),1/(5+6n)},{n,0,Infinity}]
Total[Flatten@{M4,M6}]//FullSimplify
```