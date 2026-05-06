--TEST--
Reflection: hasDefault() and getDefault()
--FILE--
<?php
class A<X, Y = int> {}
$ps = (new ReflectionClass('A'))->getGenericParameters();
var_dump($ps[0]->hasDefault());
var_dump($ps[0]->getDefault());
var_dump($ps[1]->hasDefault());
echo $ps[1]->getDefault()->getName(), "\n";
?>
--EXPECT--
bool(false)
NULL
bool(true)
int
