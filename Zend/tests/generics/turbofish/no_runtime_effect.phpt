--TEST--
Turbofish: produces same result as no turbofish
--FILE--
<?php
function f(int $x): int { return $x * 2; }
var_dump(f(5));
var_dump(f::<int>(5));
var_dump(f::<string>(5));
var_dump(f::<int, mixed, never>(5));
?>
--EXPECT--
int(10)
int(10)
int(10)
int(10)
