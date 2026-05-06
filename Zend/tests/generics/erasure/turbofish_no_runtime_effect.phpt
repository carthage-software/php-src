--TEST--
Erasure: turbofish has zero runtime effect
--FILE--
<?php
function f($x) { return $x * 2; }
var_dump(f(5));
var_dump(f::<int>(5));
var_dump(f::<int, string, float>(5));
?>
--EXPECT--
int(10)
int(10)
int(10)
