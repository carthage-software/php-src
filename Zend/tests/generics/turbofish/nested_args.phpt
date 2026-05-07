--TEST--
Turbofish: nested type arguments
--FILE--
<?php
function f<T>($x) { return $x; }
var_dump(f::<array<int, string>>(["a", "b"]));
?>
--EXPECT--
array(2) {
  [0]=>
  string(1) "a"
  [1]=>
  string(1) "b"
}
