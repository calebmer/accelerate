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

#[test]
fn test_env_overrides() {
  assert_output(
    command().env("ACCELERATE_DIRECTORY", "extension").args(&["ls"]),
    "extension/123456-foo\nextension/234567-bar\n",
    ""
  );
  assert_output(
    command().env("ACCELERATE_DIRECTORY", "extension").args(&["ls", "-d", "basic"]),
    "basic/123456-foo\nbasic/234567-bar\n",
    ""
  );
  assert_output(
    command().env("ACCELERATE_DATABASE", "123456-foo").args(&["status", "-d", "basic", "-t", "test"]),
    "✔ basic/123456-foo\n𝙭 basic/234567-bar\n",
    ""
  );
  assert_output(
    command().env("ACCELERATE_DATABASE", "123456-foo,234567-bar").args(&["status", "-d", "basic", "-t", "test"]),
    "✔ basic/123456-foo\n✔ basic/234567-bar\n",
    ""
  );
  assert_output(
    command()
    .env("ACCELERATE_DATABASE", "123456-foo,234567-bar")
    .args(&["status", "-d", "basic", "-t", "test", "-c", "123456-foo"]),
    "✔ basic/123456-foo\n𝙭 basic/234567-bar\n",
    ""
  );
  assert_output(
    command()
    .env("ACCELERATE_DIRECTORY", "basic")
    .env("ACCELERATE_DATABASE", "123456-foo")
    .env("ACCELERATE_DRIVER", "test")
    .args(&["status"]),
    "✔ basic/123456-foo\n𝙭 basic/234567-bar\n",
    ""
  );
}

#[test]
fn test_add() {
  assert_output(
    command().args(&["add", "-d", "basic", "-t", "test", "-c", ""]),
    "Add basic/123456-foo\n",
    ""
  );
  assert_output(
    command().args(&["add", "-d", "basic", "-t", "test", "-c", "", "2"]),
    "Add basic/123456-foo\nAdd basic/234567-bar\n",
    ""
  );
  assert_output(
    command().args(&["add", "-d", "basic", "-t", "test", "-c", "", "2000"]),
    "Add basic/123456-foo\nAdd basic/234567-bar\n",
    ""
  );
  assert_output(
    command().args(&["add", "-d", "basic", "-t", "test", "-c", "123456-foo"]),
    "Add basic/234567-bar\n",
    ""
  );
  assert_output(
    command().args(&["add", "-d", "basic", "-t", "test", "-c", "123456-foo,234567-bar"]),
    "",
    ""
  );
}

#[test]
fn test_sub() {
  assert_output(
    command().args(&["sub", "-d", "basic", "-t", "test", "-c", "123456-foo,234567-bar"]),
    "Sub basic/234567-bar\n",
    ""
  );
  assert_output(
    command().args(&["sub", "-d", "basic", "-t", "test", "-c", "123456-foo,234567-bar", "2"]),
    "Sub basic/234567-bar\nSub basic/123456-foo\n",
    ""
  );
  assert_output(
    command().args(&["sub", "-d", "basic", "-t", "test", "-c", "123456-foo,234567-bar", "2000"]),
    "Sub basic/234567-bar\nSub basic/123456-foo\n",
    ""
  );
  assert_output(
    command().args(&["sub", "-d", "basic", "-t", "test", "-c", "123456-foo"]),
    "Sub basic/123456-foo\n",
    ""
  );
  assert_output(
    command().args(&["sub", "-d", "basic", "-t", "test", "-c", ""]),
    "",
    ""
  );
}

#[test]
fn test_up() {
  assert_output(
    command().args(&["up", "-d", "basic", "-t", "test", "-c", ""]),
    "Add basic/123456-foo\nAdd basic/234567-bar\n",
    ""
  );
  assert_output(
    command().args(&["up", "-d", "basic", "-t", "test", "-c", "123456-foo"]),
    "Add basic/234567-bar\n",
    ""
  );
  assert_output(
    command().args(&["up", "-d", "basic", "-t", "test", "-c", "123456-foo,234567-bar"]),
    "",
    ""
  );
}

#[test]
fn test_down() {
  assert_output(
    command().args(&["down", "-d", "basic", "-t", "test", "-c", "123456-foo,234567-bar"]),
    "Sub basic/234567-bar\nSub basic/123456-foo\n",
    ""
  );
  assert_output(
    command().args(&["down", "-d", "basic", "-t", "test", "-c", "123456-foo"]),
    "Sub basic/123456-foo\n",
    ""
  );
  assert_output(
    command().args(&["down", "-d", "basic", "-t", "test", "-c", ""]),
    "",
    ""
  );
}

#[test]
fn test_redo() {
  assert_output(
    command().args(&["redo", "-d", "basic", "-t", "test", "-c", ""]),
    "Add basic/123456-foo\n",
    ""
  );
  assert_output(
    command().args(&["redo", "-d", "basic", "-t", "test", "-c", "123456-foo"]),
    "Sub basic/123456-foo\nAdd basic/123456-foo\n",
    ""
  );
  assert_output(
    command().args(&["redo", "-d", "basic", "-t", "test", "-c", "123456-foo,234567-bar"]),
    "Sub basic/234567-bar\nAdd basic/234567-bar\n",
    ""
  );
}

#[test]
fn test_reset() {
  assert_output(
    command().args(&["reset", "-d", "basic", "-t", "test", "-c", ""]),
    "",
    ""
  );
  assert_output(
    command().args(&["reset", "-d", "basic", "-t", "test", "-c", "123456-foo"]),
    "Sub basic/123456-foo\nAdd basic/123456-foo\n",
    ""
  );
  assert_output(
    command().args(&["reset", "-d", "basic", "-t", "test", "-c", "123456-foo,234567-bar"]),
    "Sub basic/234567-bar\nSub basic/123456-foo\nAdd basic/123456-foo\nAdd basic/234567-bar\n",
    ""
  );
}
