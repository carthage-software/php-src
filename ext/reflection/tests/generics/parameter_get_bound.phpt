--TEST--
Reflection: ReflectionGenericTypeParameter::getBound() returns the bound type
--FILE--
<?php
class A<X, Y : object> {}
$ps = (new ReflectionClass('A'))->getGenericParameters();
var_dump($ps[0]->getBound());
echo $ps[1]->getBound()->getName(), "\n";
?>
--EXPECT--
NULL
object
