use support::{command, assert_output};

#[test]
fn test_ls() {
  assert_output(
    command().args(&["ls", "-d", "basic"]),
    "basic/123456-foo\nbasic/234567-bar\n",
    ""
  );
  assert_output(
    command().args(&["ls", "-d", "nested"]),
    "nested/b/123456-foo\nnested/234567-bar\nnested/a/345678-baz\nnested/b/c/456789-qux\n",
    ""
  );
}

#[test]
fn test_status() {
  assert_output(
    command().args(&["status", "-d", "basic", "-t", "test", "-c", ""]),
    "𝙭 basic/123456-foo\n𝙭 basic/234567-bar\n",
    ""
  );
  assert_output(
    command().args(&["status", "-d", "basic", "-t", "test", "-c", "123456-foo"]),
    "✔ basic/123456-foo\n𝙭 basic/234567-bar\n",
    ""
  );
  assert_output(
    command().args(&["status", "-d", "basic", "-t", "test", "-c", "123456-foo,234567-bar"]),
    "✔ basic/123456-foo\n✔ basic/234567-bar\n",
    ""
  );
}
