# Data Types Memory Consumption

<show-structure for="chapter" depth="3"/>

## Scalar Types

### Integer Types

{type="narrow"}
i8
: 8-bit signed integer (1 byte). Range: -128 to 127.

u8
: 8-bit unsigned integer (1 byte). Range: 0 to 255.

i16
: 16-bit signed integer (2 bytes). Range: -32,768 to 32,767.

u16
: 16-bit unsigned integer (2 bytes). Range: 0 to 65,535.

i32
: 32-bit signed integer (4 bytes). Range: -2,147,483,648 to 2,147,483,647. This is generally the default for
integers.

u32
: 32-bit unsigned integer (4 bytes). Range: 0 to 4,294,967,295.

i64
: 64-bit signed integer (8 bytes). Range: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807.

u64
: 64-bit unsigned integer (8 bytes). Range: 0 to 18,446,744,073,709,551,615.

i128
: 128-bit signed integer (16 bytes). Range: -170,141,183,460,469,231,731,687,303,715,884,105,728 to
170,141,183,460,469,231,731,687,303,715,884,105,727.

u128
: 128-bit unsigned integer (16 bytes). Range: 0 to 340,282,366,920,938,463,463,374,607,431,768,211,455.

isize
: Pointer-sized signed integer. Size depends on the architecture: 4 bytes on a 32-bit system, 8 bytes on a 64-bit
system. The range is architecture dependent.

usize
: Pointer-sized unsigned integer. Size and range are architecture-dependent.

### Floating-Point Types

{type="narrow"}
f32
: 32-bit floating-point number (4 bytes). Precision is limited; precision loss can occur when dealing with very high
or very low values.

f64
: 64-bit floating-point number (8 bytes). This is the default for floating-point numbers and has double the precision
of f32.

### Boolean Type

{type="narrow"}
bool
: 1 byte. It can be either true or false.

### Character Type

{type="narrow"}
char
: 4 bytes. Represents a Unicode Scalar Value, which includes more than just ASCII: also includes accented letters,
Chinese/Japanese/Korean characters, emojis, etc.

## Compound Types

Tuples
: The size of a tuple is the total sizes of its components. For example, a tuple (i32, f64, u8) would be 4 + 8 +
1 = 13 bytes, not considering any padding the compiler might add for alignment purposes.

Arrays
: The size of an array is the size of a single element times the number of elements. For example, an
array [i32; 5] would be 4 bytes * 5 = 20 bytes.

> Note that the sizes of compound types will increase with more elements or if they contain types that themselves take
> up more space.
>
{style="note"}

> For example, choosing between i32 and i64 might depend on the need for larger number ranges versus saving memory and
> potential performance gains with smaller types.
>
{style="tip"}
