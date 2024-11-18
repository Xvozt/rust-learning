

|         Data Type         |     Memory Location      |
|---------------------------|--------------------------|
| Primitive types           | Stack                    |
| Structs (fixed size)      | Stack                    |
| Structs with dynamic data | Stack + Heap             |
| Enums (fixed size)        | Stack                    |
| Enums with dynamic data   | Stack + Heap             |
| String literals           | Read-only data segment   |
| const values              | Read-only data segment   |
| Immutable static vars     | Read-only data segment   |
| Mutable static vars       | Initialized data segment |
| String, Vec               | Stack (metadata) + Heap  |
