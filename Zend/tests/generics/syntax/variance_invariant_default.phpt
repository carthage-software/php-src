--TEST--
Generic syntax: type parameter without variance is invariant
--FILE--
<?php
class Box<T> {}
$p = (new ReflectionClass('Box'))->getGenericParameters()[0];
echo $p->getVariance()->name, "\n";
echo $p->getVariance()->value, "\n";
?>
--EXPECT--
Invariant
0
