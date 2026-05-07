--TEST--
Generics: 255 type arguments on a function return type is OK; 256 is a compile error
--FILE--
<?php

$N = implode(",", array_fill(0, 255, "int"));
eval("function f255(): Box<{$N}> {}");
echo "OK\n";

$N = implode(",", array_fill(0, 256, "int"));
eval("function f256(): Box<{$N}> {}");
?>
--EXPECTF--
OK

Fatal error: Cannot specify more than 255 generic type arguments (got 256) in %s : eval()'d code on line %d
