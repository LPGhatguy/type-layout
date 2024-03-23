/*!
[![GitHub CI Status](https://github.com/LPGhatguy/type-layout/workflows/CI/badge.svg)](https://github.com/LPGhatguy/type-layout/actions)
[![type-layout on crates.io](https://img.shields.io/crates/v/type-layout.svg)](https://crates.io/crates/type-layout)
[![type-layout docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/type-layout)

type-layout is a type layout debugging aid, providing a `#[derive]`able trait
that reports:
- The type's name, size, and minimum alignment
- Each field's name, type, offset, and size
- Padding due to alignment requirements

**type-layout currently only functions on structs with named fields.** This is a
temporary limitation.

## Examples

The layout of types is only defined if they're `#[repr(C)]`. This crate works on
non-`#[repr(C)]` types, but their layout is unpredictable.

```rust
use type_layout::TypeLayout;

#[derive(TypeLayout)]
#[repr(C)]
struct Foo {
    a: u8,
    b: u32,
}

println!("{}", Foo::type_layout());
// prints:
// Foo (size 8, alignment 4)
// | Offset | Name      | Size |
// | ------ | --------- | ---- |
// | 0      | a         | 1    |
// | 1      | [padding] | 3    |
// | 4      | b         | 4    |
```

Over-aligned types have trailing padding, which can be a source of bugs in some
FFI scenarios:

```rust
use type_layout::TypeLayout;

#[derive(TypeLayout)]
#[repr(C, align(128))]
struct OverAligned {
    value: u8,
}

println!("{}", OverAligned::type_layout());
// prints:
// OverAligned (size 128, alignment 128)
// | Offset | Name      | Size |
// | ------ | --------- | ---- |
// | 0      | value     | 1    |
// | 1      | [padding] | 127  |
```

## Minimum Supported Rust Version (MSRV)

type-layout supports Rust 1.34.1 and newer. Until type-layout reaches 1.0,
changes to the MSRV will require major version bumps. After 1.0, MSRV changes
will only require minor version bumps, but will need significant justification.
*/

use std::borrow::Cow;
use std::fmt::{self, Display};
use std::str;

#[doc(hidden)]
pub use memoffset;

pub use type_layout_derive::TypeLayout;

pub trait TypeLayout {
    fn type_layout() -> TypeLayoutInfo;
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct TypeLayoutInfo {
    pub name: Cow<'static, str>,
    pub size: usize,
    pub alignment: usize,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Field {
    pub name: Cow<'static, str>,
    pub ty: Cow<'static, str>,
    pub size: usize,
    pub offset: usize,
}

impl fmt::Display for TypeLayoutInfo {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            formatter,
            "{} (size {}, alignment {})",
            self.name, self.size, self.alignment
        )?;

        // Calculate the sum of all fields' sizes to detect if the
        // struct is padded.
        let fields_size: usize = self.fields.iter().map(|f| f.size).sum();
        let padding_header_length = if fields_size < self.size {
            "[padding]".len()
        } else {
            0
        };

        let longest_name = self
            .fields
            .iter()
            .map(|field| field.name.len())
            .max()
            .unwrap_or(1)
            .max(padding_header_length);

        let widths = RowWidths {
            offset: "Offset".len(),
            name: longest_name,
            size: "Size".len(),
        };

        write_row(
            formatter,
            widths,
            Row {
                offset: "Offset",
                name: "Name",
                size: "Size",
            },
        )?;

        write_row(
            formatter,
            widths,
            Row {
                offset: "------",
                name: str::repeat("-", longest_name),
                size: "----",
            },
        )?;

        let mut offset = 0;

        for field in &self.fields {
            if field.offset > offset {
                write_row(
                    formatter,
                    widths,
                    Row {
                        offset,
                        name: "[padding]",
                        size: field.offset - offset,
                    },
                )?;
            }

            write_row(
                formatter,
                widths,
                Row {
                    offset: field.offset,
                    name: &*field.name,
                    size: field.size,
                },
            )?;
            offset = field.offset + field.size;
        }

        // Handle tail padding.
        if offset < self.size {
            write_row(
                formatter,
                widths,
                Row {
                    offset,
                    name: "[padding]",
                    size: self.size - offset,
                },
            )?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy)]
struct RowWidths {
    offset: usize,
    name: usize,
    size: usize,
}

struct Row<O, N, S> {
    offset: O,
    name: N,
    size: S,
}

fn write_row<O: Display, N: Display, S: Display>(
    formatter: &mut fmt::Formatter,
    widths: RowWidths,
    row: Row<O, N, S>,
) -> fmt::Result {
    writeln!(
        formatter,
        "| {:<offset_width$} | {:<name_width$} | {:<size_width$} |",
        row.offset,
        row.name,
        row.size,
        offset_width = widths.offset,
        name_width = widths.name,
        size_width = widths.size
    )
}
