--TEST--
Erasure: array<...> erases to plain array at runtime
--FILE--
<?php
function f(array<int> $x): array<string, int> { return $x; }
$r = new ReflectionFunction('f');
$pt = $r->getParameters()[0]->getType();
echo $pt->getName(), "\n";
$rt = $r->getReturnType();
echo $rt->getName(), "\n";

f([1, 2, 3]);
echo "ok\n";
?>
--EXPECT--
array
array
ok
