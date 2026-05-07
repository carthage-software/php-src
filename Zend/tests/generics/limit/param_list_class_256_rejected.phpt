--TEST--
Generics: 255 type parameters on a class is OK; 256 is a compile error
--FILE--
<?php
$names255 = implode(",", array_map(fn($i) => "T{$i}", range(0, 254)));
eval("class C255<{$names255}> {}");
echo "OK\n";

$names256 = implode(",", array_map(fn($i) => "T{$i}", range(0, 255)));
eval("class C256<{$names256}> {}");
?>
--EXPECTF--
OK

Fatal error: Cannot declare more than 255 generic type parameters (got 256) in %s : eval()'d code on line %d
