# arber

[arber](https://en.wikipedia.org/wiki/Gro%C3%9Fer_Arber) is a Merkle-Mountain-Range (MMR) implementation.

The following description is taken from [this](https://github.com/mimblewimble/grin/blob/master/doc/mmr.md) excellent introduction.

Merkle Mountain Ranges [1] are an alternative to Merkle trees [2]. While the
Merkle tree relies on perfectly balanced binary trees, Merkle Mountain Ranges
can be seen either as list of perfectly balanced binary trees or a single binary
tree that would have been truncated from the top right. A Merkle Mountain Range (MMR)
is strictly append-only: elements are added from the left to the right, adding a
parent as soon as 2 children exist, filling up the range accordingly.

This illustrates a range with 11 inserted leaves and total size 19, where each
node is annotated with its order of insertion.

```
Height

3              14
             /    \
            /      \
           /        \
          /          \
2        6            13
       /   \        /    \
1     2     5      9     12     17
     / \   / \    / \   /  \   /  \
0   0   1 3   4  7   8 10  11 15  16 18
```

This can be represented as a flat list, here storing the height of each node at their
position index of insertion:

```
0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18
0  0  1  0  0  1  2  0  0  1  0  0  1  2  3  0  0  1  0
```

This structure can be fully described simply from its size (19).

<br>

🚧 **arber is currently under construction - a hardhat is recommended beyond this point** 🚧

<br>

[1] Peter Todd, [merkle-mountain-range](https://github.com/opentimestamps/opentimestamps-server/blob/master/doc/merkle-mountain-range.md)

[2] [Wikipedia, Merkle Tree](https://en.wikipedia.org/wiki/Merkle_tree)