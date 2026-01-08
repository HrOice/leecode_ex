哈希表
> 哈希表是根据关键码的值而直接进行访问的数据结构。
>

类型：
1.数组，通过下标（key）访问元素value
2.hashMap，通过key访问元素，比数组更紧凑。
    - 线性探测
    - 拉链法
3.hashSet，同hashMap，不过没有value

hashMap会维护额外结构，有时候使用数组更优。
比如限定范围，小写字母之类，用数组替换hashmap更优秀。

像字母出现次数，求和等算法可以使用hash，需要统计次数就用数组 or hashMap，只判断是否存在就用hashSet。



