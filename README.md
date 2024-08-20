# Pulldown cmark Doctest Example

- To run doctest & see the events


```bash
cargo test --doc -- --nocapture
```

output

```bash
   Doc-tests pullmark_doctest_example

running 1 test
Test executable failed (exit status: 101).

stdout:
Start(Paragraph)
Text(Borrowed("My Document"))
End(Paragraph)
Start(Heading { level: H1, id: None, classes: [], attrs: [] })
Text(Borrowed("Checklist Section"))
End(Heading(H1))
Start(List(None))
Start(Item)
Text(Borrowed("["))
Text(Borrowed("x"))
Text(Borrowed("]"))
Text(Borrowed(" Item 1 "))
InlineHtml(Borrowed("<!--id:1-->"))
End(Item)
Start(Item)
Text(Borrowed("["))
Text(Borrowed("x"))
Text(Borrowed("]"))
Text(Borrowed(" Item 2 "))
InlineHtml(Borrowed("<!--id:2-->"))
End(Item)
Start(Item)
Text(Borrowed("["))
Text(Borrowed(" "))
Text(Borrowed("]"))
Text(Borrowed(" Item 3 "))
InlineHtml(Borrowed("<!--id:3-->"))
End(Item)
End(List(false))
Start(Heading { level: H1, id: None, classes: [], attrs: [] })
Text(Borrowed("Other Section"))
End(Heading(H1))

stderr:
Ok(())
thread 'main' panicked at src/checklist.rs:36:3:
assertion failed: result.is_err()
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

test src/checklist.rs - checklist::checklist_check (line 14) ... FAILED

failures:

failures:
    src/checklist.rs - checklist::checklist_check (line 14)

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.37s

error: doctest failed, to rerun pass `--doc`

```

- To run unittest & see the events

```bash
cargo test -- --nocapture
```

output

```bash

running 1 test
Start(Heading { level: H1, id: None, classes: [], attrs: [] })
Text(Borrowed("My Document"))
End(Heading(H1))
Start(Heading { level: H2, id: None, classes: [], attrs: [] })
Text(Borrowed("Checklist Section"))
End(Heading(H2))
Start(List(None))
Start(Item)
Text(Borrowed("["))
Text(Borrowed("x"))
Text(Borrowed("]"))
Text(Borrowed(" Item 1 "))
InlineHtml(Borrowed("<!--id:1-->"))
End(Item)
Start(Item)
Text(Borrowed("["))
Text(Borrowed("x"))
Text(Borrowed("]"))
Text(Borrowed(" Item 2 "))
InlineHtml(Borrowed("<!--id:2-->"))
End(Item)
Start(Item)
Text(Borrowed("["))
Text(Borrowed(" "))
Text(Borrowed("]"))
Text(Borrowed(" Item 3 "))
InlineHtml(Borrowed("<!--id:3-->"))
End(Item)
End(List(false))
Start(Heading { level: H2, id: None, classes: [], attrs: [] })
thread 'checklist::test_checklist_check' panicked at src/checklist.rs:176:5:
assertion failed: result.is_err()
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test checklist::test_checklist_check ... FAILED

failures:

failures:
    checklist::test_checklist_check

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```
