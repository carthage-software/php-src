--TEST--
Generics: 255 type arguments on a function call turbofish is OK; 256 is a compile error
--FILE--
<?php

$N = implode(",", array_fill(0, 255, "int"));
eval("function g255() { f::<{$N}>(); }");
echo "OK\n";

$N = implode(",", array_fill(0, 256, "int"));
eval("function g256() { f::<{$N}>(); }");
?>
--EXPECTF--
OK

Fatal error: Cannot specify more than 255 generic type arguments (got 256) in %s : eval()'d code on line %d
