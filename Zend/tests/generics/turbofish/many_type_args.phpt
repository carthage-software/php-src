--TEST--
Turbofish: many type arguments (extras silently discarded)
--FILE--
<?php
function f($x) { return $x; }
var_dump(f::<int, string, float, bool, array, mixed, never, void, null>(7));
?>
--EXPECT--
int(7)
