# CSX0003RUST

Rust playground; familiarising with ownership, generics, trait objects, etc

- Random Select
- Merge sort (mutable slices, in-place merging)
- Quick sort (mutable slices, in-place partitioning)
- Linked list 
- Binary tree

No performance and/or optimisation warranties :-)

sample output:
```
	Input: (8)[58, 85, 85, -82, -13, -76, -91, 53] =>
	Input: (4)[58, 85, 85, -82] =>
	Input: (2)[58, 85] =>
	Input: (2)[85, -82] =>
	Inversion Vector: [2, 0, 0, 0]
	Merge: 0:[58, 85] <> 1:[-82, 85] => 3:[-82, 58, 85, 85]
	Input: (4)[-13, -76, -91, 53] =>
	Input: (2)[-13, -76] =>
	Input: (2)[-91, 53] =>
	Inversion Vector: [2, 0, 0, 0]
	Merge: 1:[-76, -13] <> 0:[-91, 53] => 3:[-91, -76, -13, 53]
	Inversion Vector: [4, 0, 3, 3, 3, 0, 0, 0]
	Merge: 3:[-82, 58, 85, 85] <> 3:[-91, -76, -13, 53] => 19:[-91, -82, -76, -13, 53, 58, 85, 85]

Merge Sort: (19, [-91, -82, -76, -13, 53, 58, 85, 85])
	Input: [58, 85, 85, -82, -13, -76, -91, 53], (@5th)
	-:[-13, 85, 85, -82, 58, -76, -91, 53],(1,2)
	-:[-13, 85, 85, -82, 58, -76, -91, 53],(1,3)
	s:[-13, -82, 85, 85, 58, -76, -91, 53],(2,4)
	-:[-13, -82, 85, 85, 58, -76, -91, 53],(2,5)
	s:[-13, -82, -76, 85, 58, 85, -91, 53],(3,6)
	s:[-13, -82, -76, -91, 58, 85, 85, 53],(4,7)
	-:[-13, -82, -76, -91, 58, 85, 85, 53],(4,8)
	f:[-91, -82, -76, -13, 58, 85, 85, 53], (4)
	Input: [-91, -82, -76], (@2nd)
	s:[-82, -91, -76],(2,2)
	-:[-82, -91, -76],(2,3)
	f:[-91, -82, -76], (2)
	Input: [58, 85, 85, 53], (@3rd)
	-:[85, 85, 58, 53],(1,2)
	s:[85, 58, 85, 53],(2,3)
	s:[85, 58, 53, 85],(3,4)
	f:[53, 58, 85, 85], (3)
	Input: [53, 58], (@2nd)
	s:[58, 53],(2,2)
	f:[53, 58], (2)
Quick Sort: [-91, -82, -76, -13, 53, 58, 85, 85]

bTree Sort: [85, 85, 58, 53, -13, -76, -82, -91]

List      : [58, 85, 85, -82, -13, -76, -91, 53]
```
